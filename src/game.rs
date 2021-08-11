//! Main storage for entity containers.

use crate::{
    entity::EntityContainer,
    error::CtraitResult,
    render::{manager::TextureManager, RenderContext, Renderer},
    traits::{FixedUpdate, Interactive, Renderable, Update},
};
use chrono::Duration;
use std::time::Instant;
use timer::Timer;

/// Game manager.
///
/// The game manager holds multiple [`EntityContainer`]s, each representing
/// [`Weak`](std::sync::Weak) pointers to
/// entities.
pub struct Game {
    /// Entities implementing [`Update`] trait.
    pub update_entities: EntityContainer<dyn Update>,
    /// Entities implementing [`FixedUpdate`] trait.
    pub fixed_update_entities: EntityContainer<dyn FixedUpdate>,
    /// Entities implementing [`Renderable`] trait.
    pub renderable_entities: EntityContainer<dyn Renderable>,
    /// Entities implementing [`Interactive`] trait.
    pub interactive_entities: EntityContainer<dyn Interactive>,
    timestep: i64,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    /// Default number of milliseconds between [`FixedUpdate::fixed_update`] method calls.
    pub const DEFAULT_TIMESTEP: i64 = ((1.0 / 50.0) * 1000.0) as i64;

    /// Create a new game.
    ///
    /// # Examples
    ///
    /// ```
    /// use ctrait::game::Game;
    ///
    /// let game = Game::new();
    /// ```
    pub fn new() -> Self {
        Self {
            update_entities: EntityContainer::default(),
            fixed_update_entities: EntityContainer::default(),
            renderable_entities: EntityContainer::default(),
            interactive_entities: EntityContainer::default(),
            timestep: Self::DEFAULT_TIMESTEP,
        }
    }

    /// Customize the delay in milliseconds between [`FixedUpdate::fixed_update`] method calls.
    ///
    /// Default timestep is equal to [`Self::DEFAULT_TIMESTEP`].
    pub fn with_timestep(mut self, timestep: i64) -> Self {
        self.timestep = timestep;
        self
    }

    /// Start the game with the given renderer.
    ///
    /// This will block until a quit signal is sent.
    ///
    /// # Errors
    ///
    /// If [`sdl2`] fails to start, a [`CtraitError`](crate::error::CtraitError) variant will be returned.
    pub fn start(&mut self, renderer: &mut Renderer) -> CtraitResult<()> {
        let sdl_context = sdl2::init()?;
        let mut event_pump = sdl_context.event_pump()?;
        let video_subsystem = sdl_context.video()?;
        let canvas = renderer.config.create_canvas(&video_subsystem)?;
        let texture_creator = canvas.texture_creator();
        let texture_manager = TextureManager::new(&texture_creator);
        let mut render_context = RenderContext::new(canvas, texture_manager);
        // Start fixed update processs.
        let timer = Timer::new();
        let mut fixed_update_instant = Instant::now();
        let fixed_update_entities = self.fixed_update_entities.clone();
        let _guard = timer.schedule_repeating(Duration::milliseconds(self.timestep), move || {
            fixed_update_entities
                .access()
                .lock()
                .unwrap()
                .iter()
                .for_each(|entity| {
                    entity
                        .upgrade()
                        .unwrap()
                        .lock()
                        .unwrap()
                        .fixed_update(fixed_update_instant.elapsed().as_secs_f64())
                });
            fixed_update_instant = Instant::now();
        });
        // Start standard game loop.
        let mut standard_instant = Instant::now();
        loop {
            renderer.process_event(&mut event_pump, &mut self.interactive_entities);
            self.update_entities
                .access()
                .lock()
                .unwrap()
                .iter()
                .for_each(|entity| {
                    entity
                        .upgrade()
                        .unwrap()
                        .lock()
                        .unwrap()
                        .update(standard_instant.elapsed().as_secs_f64())
                });
            standard_instant = Instant::now();
            if renderer.has_quit() {
                break;
            }
            renderer.render(&mut render_context, &mut self.renderable_entities);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Game;

    #[test]
    fn game_default() {
        let game = Game::default();
        // Ensure, by default, there are no entities in any of the entity containers.
        assert!(game.update_entities.access().lock().unwrap().is_empty());
        assert!(game
            .fixed_update_entities
            .access()
            .lock()
            .unwrap()
            .is_empty());
        assert!(game.renderable_entities.access().lock().unwrap().is_empty());
        assert!(game
            .interactive_entities
            .access()
            .lock()
            .unwrap()
            .is_empty());
        // Timestep should be default.
        assert_eq!(game.timestep, Game::DEFAULT_TIMESTEP);
    }

    #[test]
    fn game_with_timestep() {
        let game = Game::default().with_timestep(12);
        assert_eq!(game.timestep, 12);
    }
}
