//! Main storage for entity containers.

use crate::{
    error::CtraitResult,
    render::{manager::TextureManager, RenderContext, Renderer},
    traits::{FixedUpdate, Interactive, Renderable, Update},
};
use chrono::Duration;
use std::{
    sync::{Arc, Mutex, Weak},
    time::Instant,
};
use timer::Timer;

/// A type representing a single game entity.
pub type Entity<T> = Arc<Mutex<T>>;

type WeakEntity<T> = Weak<Mutex<T>>;

/// Macro to quickly create a new entity.
///
/// # Example
/// ```
/// use ctrait::entity;
///
/// struct Player;
///
/// let player = entity!(Player {});
/// // player now refers to an entity.
/// ```
#[macro_export]
macro_rules! entity {
    ($object:expr) => {
        std::sync::Arc::new(std::sync::Mutex::new($object))
    };
}

/// Macro to define a slice of cloned entities that all implement a given trait.
///
/// The first argument should be a trait that all following entities implement.
/// The macro is mainly useful when passing entities to [`Game`] as a slice with
/// [`EntityContainer::add_entities`].
///
/// # Example
/// ```
/// use ctrait::{entity, entities, game::Entity};
///
/// trait FooTrait {}
///
/// struct A;
/// impl FooTrait for A {}
///
/// struct B;
/// impl FooTrait for B {}
///
/// let a = entity!(A {});
/// let b = entity!(B {});
/// let foo_entities: &[Entity<dyn FooTrait>] = &entities!(FooTrait; a, b);
/// // Although entities a and b are derived from different types, they are both coerced to trait
/// // objects and can be stored together in foo_entities.
/// ```
#[macro_export]
macro_rules! entities {
    ($name:ident; $($entity:expr),+) => {
        [$(ctrait::game::Entity::clone(&$entity) as ctrait::game::Entity<dyn $name>),+]
    };
}

/// Structure containing [`Weak`] references to entities.
pub struct EntityContainer<T: ?Sized>(Arc<Mutex<Vec<WeakEntity<T>>>>);

impl<T: ?Sized> Default for EntityContainer<T> {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(Vec::new())))
    }
}

impl<T: ?Sized> Clone for EntityContainer<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<T: ?Sized> EntityContainer<T> {
    /// Add entities from a given entity slice.
    ///
    /// It is recommended to create the entity slice with [`entities`].
    ///
    /// # Example
    /// ```
    /// use ctrait::{
    ///     camera::Camera,
    ///     entity, entities,
    ///     game::Game,
    ///     render::RenderContext,
    ///     traits::Renderable
    /// };
    ///
    /// struct A;
    /// impl Renderable for A {
    ///     fn render(&self, _: &Camera, _: &mut RenderContext) {}
    /// }
    /// struct B;
    /// impl Renderable for B {
    ///     fn render(&self, _: &Camera, _: &mut RenderContext) {}
    /// }
    ///
    /// let a = entity!(A {});
    /// let b = entity!(B {});
    ///
    /// // Pass a slice of Renderable entities to the game.
    /// let mut game = Game::default();
    /// game.renderable_entities.add_entities(&entities!(Renderable; a, b));
    /// ```
    pub fn add_entities(&mut self, other: &[Entity<T>]) {
        for entity in other.iter() {
            self.push(entity);
        }
    }

    /// Clears the entity container, removing all entities.
    pub fn clear(&mut self) {
        self.0.lock().unwrap().clear();
    }

    // Append a new entity to the entity container.
    fn push(&mut self, entity: &Entity<T>) {
        self.0.lock().unwrap().push(Arc::downgrade(entity));
    }

    fn prune(entities: &mut Vec<WeakEntity<T>>) {
        // Whenever the entities are accessed, check if inner values for each entity exists.
        // If an inner value does not exist, it indicates that the original entity has been
        // dropped. Thus, it should be removed from the container as well.
        entities.retain(|entity| entity.upgrade().is_some());
    }

    pub(crate) fn access(&self) -> &Arc<Mutex<Vec<WeakEntity<T>>>> {
        let entities = &mut self.0.lock().unwrap();
        Self::prune(entities);
        &self.0
    }
}

/// Game manager.
///
/// The game manager holds multiple [`EntityContainer`]s, each representing [`Weak`] pointers to
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
        Self {
            update_entities: EntityContainer::default(),
            fixed_update_entities: EntityContainer::default(),
            renderable_entities: EntityContainer::default(),
            interactive_entities: EntityContainer::default(),
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

    /// Start the game with the given renderer.
    ///
    /// This will block until a quit signal is sent.
    pub fn start(&mut self, renderer: &mut Renderer) -> CtraitResult<()> {
        let sdl_context = sdl2::init()?;
        let mut event_pump = sdl_context.event_pump()?;
        let video_subsystem = sdl_context.video()?;
        let canvas = renderer.config.get_canvas(&video_subsystem)?;
        let texture_creator = canvas.texture_creator();
        let texture_manager = TextureManager::new(&texture_creator);
        let mut render_context = RenderContext::new(canvas, texture_manager);
        // Start fixed update processs.
        let timer = Timer::new();
        let mut fixed_update_instant = Instant::now();
        let fixed_update_entities = Arc::clone(&self.fixed_update_entities.0);
        let _guard = timer.schedule_repeating(Duration::milliseconds(self.timestep), move || {
            let mut fixed_update_entities = fixed_update_entities.lock().unwrap();
            // Since fixed_update_entities is an Arc clone, EntityContainer::access cannot be used.
            // Use EntityContainer::prune instead.
            EntityContainer::prune(&mut fixed_update_entities);
            fixed_update_entities.iter().for_each(|entity| {
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
    use super::{Arc, EntityContainer};

    // Test struct to create test entity.
    struct Test;

    #[test]
    fn game_entity_access() {
        impl Test {
            fn it_works(&self) -> bool {
                true
            }
        }
        let entity = entity!(Test {});
        assert!(entity.lock().unwrap().it_works());
    }

    #[test]
    fn game_entity_container_push() {
        let entity = entity!(Test {});
        let mut container = EntityContainer::default();
        container.push(&entity);
        assert_eq!(container.0.lock().unwrap().len(), 1);
    }

    #[test]
    fn game_entity_container_add_entities() {
        let a = entity!(Test {});
        let b = entity!(Test {});
        let mut container = EntityContainer::default();
        container.add_entities(&[a, b]);
        assert_eq!(container.0.lock().unwrap().len(), 2);
    }

    #[test]
    fn game_entity_container_clear() {
        let entity = entity!(Test {});
        let mut container = EntityContainer::default();
        container.0.lock().unwrap().push(Arc::downgrade(&entity));
        container.clear();
        assert!(container.0.lock().unwrap().is_empty());
    }

    #[test]
    fn game_entity_prune() {
        let a = entity!(Test {});
        let b = entity!(Test {});
        let mut entities = vec![Arc::downgrade(&a), Arc::downgrade(&b)];
        drop(a);
        EntityContainer::prune(&mut entities);
        assert_eq!(entities.len(), 1);
        drop(b);
        EntityContainer::prune(&mut entities);
        assert!(entities.is_empty());
    }

    #[test]
    fn game_entity_container_access() {
        let a = entity!(Test {});
        let b = entity!(Test {});
        let container = EntityContainer::default();
        // Intentional avoidance of EntityContainer::add_entities. This function is tested
        // elsewhere.
        container
            .0
            .lock()
            .unwrap()
            .extend_from_slice(&[Arc::downgrade(&a), Arc::downgrade(&b)]);
        // Drop a. The Weak entity pointing to A in the container should be removed upon access
        // method call.
        drop(a);
        container.access();
        assert_eq!(container.0.lock().unwrap().len(), 1);
        drop(b);
        container.access();
        assert!(container.0.lock().unwrap().is_empty());
    }
}
