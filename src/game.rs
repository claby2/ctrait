use crate::{
    renderer::Renderer,
    traits::{Interactive, Renderable, Update},
};
use std::{cell::RefCell, ops::Deref, rc::Rc, time::Instant};

pub type EntityType<T> = Rc<RefCell<T>>;

/// A New Type that represents a single game entity.
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
    pub fn new(object: T) -> Self {
        Self(Rc::new(RefCell::new(object)))
    }
}

/// Allows the [`Entity`] to be dereferenced so its methods can be used.
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
/// assert_eq!(entity.borrow_mut().get_value(), 3);
impl<T> Deref for Entity<T> {
    type Target = EntityType<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Define a slice of entities that all implement a given trait.
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
///
/// struct B;
/// impl Renderable for B {
///     fn render(&self, _: &Camera, _: &mut CanvasWindow) {}
/// }
///
/// let a = Entity::new(A {});
/// let b = Entity::new(B {});
/// let renderable_entities: Vec<EntityType<dyn Renderable>> =
///     entity_slice!(Renderable, a, b).to_vec();
#[macro_export]
macro_rules! entity_slice {
    ($name:ident, $($entity:expr),+) => {
        [$(std::rc::Rc::clone(&$entity) as ctrait::game::EntityType<dyn $name>),+]
    };
}

/// Game manager.
pub struct Game {
    renderer: Renderer,
    update_entities: Vec<EntityType<dyn Update>>,
    renderable_entities: Vec<EntityType<dyn Renderable>>,
    interactive_entities: Vec<EntityType<dyn Interactive>>,
}

impl Game {
    pub fn new(renderer: Renderer) -> Self {
        Self {
            renderer,
            update_entities: Vec::new(),
            renderable_entities: Vec::new(),
            interactive_entities: Vec::new(),
        }
    }

    pub fn with_update_entities(mut self, entities: &[EntityType<dyn Update>]) -> Self {
        self.update_entities = entities.to_vec();
        self
    }

    pub fn with_renderable_entities(mut self, entities: &[EntityType<dyn Renderable>]) -> Self {
        self.renderable_entities = entities.to_vec();
        self
    }

    pub fn with_interactive_entities(mut self, entities: &[EntityType<dyn Interactive>]) -> Self {
        self.interactive_entities = entities.to_vec();
        self
    }

    pub fn start(mut self) {
        let mut now = Instant::now();
        loop {
            self.renderer.process_event(&mut self.interactive_entities);
            self.update_entities
                .iter()
                .for_each(|entity| entity.borrow_mut().update(now.elapsed().as_secs_f64()));
            now = Instant::now();
            if self.renderer.has_quit() {
                break;
            }
            self.renderer.render(&mut self.renderable_entities);
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
        // borrow_mut should only work if Entity can be dereferenced.
        assert!(alice.borrow_mut().it_works());
    }
}
