//! Entity type and container.
use std::sync::{Arc, Mutex, Weak};

/// A type representing a single game entity.
pub type Entity<T> = Arc<Mutex<T>>;

type WeakEntity<T> = Weak<Mutex<T>>;

/// Macro to quickly create a new entity.
///
/// # Examples
///
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
/// The macro is mainly useful when passing entities to [`Game`](crate::game::Game) as a slice with
/// [`EntityContainer::add_entities`].
///
/// # Examples
///
/// ```
/// use ctrait::{entity, entities, entity::Entity};
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
        [$(ctrait::entity::Entity::clone(&$entity) as ctrait::entity::Entity<dyn $name>),+]
    };
}

/// Structure containing [`Weak`] references to entities.
#[derive(Debug)]
pub struct EntityContainer<T: ?Sized>(Arc<Mutex<Vec<WeakEntity<T>>>>);

impl<T: ?Sized> Default for EntityContainer<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: ?Sized> Clone for EntityContainer<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<T: ?Sized> EntityContainer<T> {
    /// Constructs a new entity container.
    ///
    /// # Examples
    ///
    /// ```
    /// use ctrait::{entity::EntityContainer, traits::Renderable};
    ///
    /// let entity_container = EntityContainer::<dyn Renderable>::new();
    /// ```
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(Vec::new())))
    }

    /// Add entities from a given entity slice.
    ///
    /// It is recommended to create the entity slice with [`entities`].
    ///
    /// # Examples
    ///
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
    /// let mut game = Game::new();
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

#[cfg(test)]
mod tests {
    use super::{Arc, EntityContainer};

    // Test struct to create test entity.
    struct Test;

    #[test]
    fn entity_container_new() {
        let entity_container = EntityContainer::<Test>::new();
        assert!(entity_container.0.lock().unwrap().is_empty());
    }

    #[test]
    fn entity_access() {
        impl Test {
            fn it_works(&self) -> bool {
                true
            }
        }
        let entity = entity!(Test {});
        assert!(entity.lock().unwrap().it_works());
    }

    #[test]
    fn entity_container_clone() {
        let container = EntityContainer::<Test>::default();
        let _container_clone = container.clone();
        assert_eq!(Arc::strong_count(&container.0), 2);
    }

    #[test]
    fn entity_container_push() {
        let entity = entity!(Test {});
        let mut container = EntityContainer::default();
        container.push(&entity);
        assert_eq!(container.0.lock().unwrap().len(), 1);
    }

    #[test]
    fn entity_container_add_entities() {
        let a = entity!(Test {});
        let b = entity!(Test {});
        let mut container = EntityContainer::default();
        container.add_entities(&[a, b]);
        assert_eq!(container.0.lock().unwrap().len(), 2);
    }

    #[test]
    fn entity_container_clear() {
        let entity = entity!(Test {});
        let mut container = EntityContainer::default();
        container.0.lock().unwrap().push(Arc::downgrade(&entity));
        container.clear();
        assert!(container.0.lock().unwrap().is_empty());
    }

    #[test]
    fn entity_container_prune() {
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
    fn entity_container_access() {
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
