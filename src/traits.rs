use crate::{camera::Camera, render::WindowCanvas};
use sdl2::event::Event;

/// A type that should update every game loop iteration.
pub trait Update: Send {
    /// Called once per game loop iteration.
    /// `delta` is the number of seconds since the last update.
    fn update(&mut self, delta: f64);
}

/// A type that should update every fixed timestep.
///
/// This should be used instead of [`Update`] for time-dependent operations.
pub trait FixedUpdate: Send {
    /// `delta` is the number of seconds since the last update.
    /// It should be approximately equal to the default timestep
    /// [`crate::game::Game::DEFAULT_TIMESTEP`].
    fn fixed_update(&mut self, delta: f64);
}

/// A type that is responsive to user events.
pub trait Interactive: Send {
    /// Called for each event in the event queue.
    fn on_event(&mut self, event: &Event);
}

/// A type that can be rendered.
pub trait Renderable: Send {
    /// Called by [`crate::render::Renderer`].
    fn render(&self, camera: &Camera, canvas: &mut WindowCanvas);
}
