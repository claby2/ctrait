use crate::{
    renderer::Renderer,
    traits::{FixedUpdate, Interactive, Renderable, Update},
};
use chrono::Duration;
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
    time::Instant,
};
use timer::Timer;

pub type EntityType<T> = Arc<Mutex<T>>;

/// A New Type of [`EntityType`] that represents a single game entity.
#[derive(Debug)]
pub struct Entity<T>(EntityType<T>);

impl<T> Entity<T> {
    /// Returns an entity based on the given object.
    ///
    /// # Example
    /// ```
    /// use ctrait::game::Entity;
    ///
    /// struct Player;
    /// let entity = Entity::new(Player {});
    /// ```
    pub fn new(object: T) -> Self {
        Self(Arc::new(Mutex::new(object)))
    }
}

/// Allows the [`Entity`] to be dereferenced and directly accessed.
///
/// # Example
/// ```
/// use ctrait::game::Entity;
///
/// struct Player(u64);
/// impl Player {
///     fn get_value(&self) -> u64 {
///         self.0
///     }
/// }
/// let entity = Entity::new(Player(3));
/// assert_eq!(entity.lock().unwrap().get_value(), 3);
/// ```
impl<T> Deref for Entity<T> {
    type Target = EntityType<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Define a slice of entities that all implement a given trait.
///
/// This is mainly useful when passing entities to [`Game`] as a slice.
///
/// # Example
/// ```
/// use ctrait::{
///     camera::Camera,
///     entity_slice,
///     game::{Entity, EntityType},
///     renderer::CanvasWindow,
///     traits::Renderable
/// };
///
/// struct A;
/// impl Renderable for A {
///     fn render(&self, _: &Camera, _: &mut CanvasWindow) {}
/// }
/// struct B;
/// impl Renderable for B {
///     fn render(&self, _: &Camera, _: &mut CanvasWindow) {}
/// }
///
/// let a = Entity::new(A {});
/// let b = Entity::new(B {});
/// let renderable_entities: Vec<EntityType<dyn Renderable>> =
///     entity_slice!(Renderable, a, b).to_vec();
/// ```
#[macro_export]
macro_rules! entity_slice {
    ($name:ident, $($entity:expr),+) => {
        [$(std::sync::Arc::clone(&$entity) as ctrait::game::EntityType<dyn $name>),+]
    };
}

/// Game manager.
pub struct Game {
    update_entities: Vec<EntityType<dyn Update>>,
    fixed_update_entities: Vec<EntityType<dyn FixedUpdate>>,
    renderable_entities: Vec<EntityType<dyn Renderable>>,
    interactive_entities: Vec<EntityType<dyn Interactive>>,
    timestep: i64,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            update_entities: Vec::new(),
            fixed_update_entities: Vec::new(),
            renderable_entities: Vec::new(),
            interactive_entities: Vec::new(),
            // Manually implement Default for Game to specify default timestep.
            timestep: Self::DEFAULT_TIMESTEP,
        }
    }
}

impl Game {
    /// Default number of milliseconds between [`FixedUpdate::fixed_update`] method calls.
    pub const DEFAULT_TIMESTEP: i64 = ((1.0 / 50.0) * 1000.0) as i64;

    /// Customize the delay in milliseconds between [`FixedUpdate::fixed_update`] method calls.
    ///
    /// Default timestep is equal to [`Self::DEFAULT_TIMESTEP`].
    pub fn with_timestep(mut self, timestep: i64) -> Self {
        self.timestep = timestep;
        self
    }

    /// Add entities with [`Update`] trait.
    ///
    /// # Example
    /// ```
    /// use ctrait::{
    ///     entity_slice,
    ///     game::{Entity, Game},
    ///     traits::Update
    /// };
    ///
    /// struct A;
    /// impl Update for A {
    ///     fn update(&mut self, _: f64) {}
    /// }
    /// let a = Entity::new(A {});
    /// Game::default()
    ///     .with_update_entities(&entity_slice!(Update, a));
    /// ```
    pub fn with_update_entities(mut self, entities: &[EntityType<dyn Update>]) -> Self {
        self.update_entities = entities.to_vec();
        self
    }

    /// Add entities with [`FixedUpdate`] trait.
    ///
    /// # Example
    /// ```
    /// use ctrait::{
    ///     entity_slice,
    ///     game::{Entity, Game},
    ///     traits::FixedUpdate
    /// };
    ///
    /// struct A;
    /// impl FixedUpdate for A {
    ///     fn fixed_update(&mut self, _: f64) {}
    /// }
    /// let a = Entity::new(A {});
    /// Game::default()
    ///     .with_fixed_update_entities(&entity_slice!(FixedUpdate, a));
    /// ```
    pub fn with_fixed_update_entities(mut self, entities: &[EntityType<dyn FixedUpdate>]) -> Self {
        self.fixed_update_entities = entities.to_vec();
        self
    }

    /// Add entities with [`Renderable`] trait.
    ///
    /// # Example
    /// ```
    /// use ctrait::{
    ///     camera::Camera,
    ///     entity_slice,
    ///     game::{Entity, Game},
    ///     renderer::CanvasWindow,
    ///     traits::Renderable
    /// };
    ///
    /// struct A;
    /// impl Renderable for A {
    ///     fn render(&self, _: &Camera, _: &mut CanvasWindow) {}
    /// }
    /// let a = Entity::new(A {});
    /// Game::default()
    ///     .with_renderable_entities(&entity_slice!(Renderable, a));
    /// ```
    pub fn with_renderable_entities(mut self, entities: &[EntityType<dyn Renderable>]) -> Self {
        self.renderable_entities = entities.to_vec();
        self
    }

    /// Add entities with [`Interactive`] trait.
    ///
    /// # Example
    /// ```
    /// use ctrait::{
    ///     entity_slice,
    ///     game::{Entity, Game},
    ///     traits::Interactive,
    ///     Event
    /// };
    ///
    /// struct A;
    /// impl Interactive for A {
    ///     fn on_event(&mut self, _: &Event) {}
    /// }
    /// let a = Entity::new(A {});
    /// Game::default()
    ///     .with_interactive_entities(&entity_slice!(Interactive, a));
    /// ```
    pub fn with_interactive_entities(mut self, entities: &[EntityType<dyn Interactive>]) -> Self {
        self.interactive_entities = entities.to_vec();
        self
    }

    /// Start the game with the given renderer.
    /// This will consume the game and block until a quit signal has been sent.
    pub fn start(mut self, renderer: &mut Renderer) {
        // Start fixed update processs.
        let fixed_update_entities = self.fixed_update_entities;
        let timer = Timer::new();
        let mut fixed_update_instant = Instant::now();
        let _guard =
            timer.schedule_repeating(Duration::milliseconds(Self::DEFAULT_TIMESTEP), move || {
                // Iterate through fixed update entities and call fixed_update method.
                fixed_update_entities.iter().for_each(|entity| {
                    entity
                        .lock()
                        .unwrap()
                        .fixed_update(fixed_update_instant.elapsed().as_secs_f64())
                });
                fixed_update_instant = Instant::now();
            });
        // Start standard game loop.
        let mut standard_instant = Instant::now();
        loop {
            renderer.process_event(&mut self.interactive_entities);
            self.update_entities.iter().for_each(|entity| {
                entity
                    .lock()
                    .unwrap()
                    .update(standard_instant.elapsed().as_secs_f64())
            });
            standard_instant = Instant::now();
            if renderer.has_quit() {
                break;
            }
            renderer.render(&mut self.renderable_entities);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Entity;

    #[test]
    fn entity_deref() {
        struct Alice;
        impl Alice {
            fn it_works(&self) -> bool {
                true
            }
        }
        let alice = Entity::new(Alice {});
        // Should only work if entity can be dereferenced.
        assert!(alice.lock().unwrap().it_works());
    }
}
