use ctrait::{
    camera::Camera,
    entities, entity,
    entity::{Entity, EntityContainer},
    game::Game,
    math::Vector2,
    rect::Rect,
    render::{RenderContext, Renderer},
    traits::{FixedUpdate, Interactive, Renderable, Update},
    Color, Event, Keycode,
};

#[derive(Debug)]
struct Block {
    rect: Rect,
}

impl Block {
    const FALL_SPEED: f64 = 600.0;

    fn new(position: &Vector2<i32>) -> Self {
        Self {
            rect: Rect::from_center(position.x, position.y, 50, 50).with_color(&Color::GRAY),
        }
    }
}

impl FixedUpdate for Block {
    fn fixed_update(&mut self, delta: f64) {
        self.rect.position.y += (Self::FALL_SPEED * delta) as i32;
    }
}

impl Renderable for Block {
    fn render(&self, camera: &Camera, context: &mut RenderContext) {
        self.rect.render(camera, context);
    }
}

#[derive(Debug, Default)]
struct Movement {
    left: bool,
    right: bool,
}

struct Spawner {
    rect: Rect,
    movement: Movement,
    renderable_entities: EntityContainer<dyn Renderable>,
    fixed_update_entities: EntityContainer<dyn FixedUpdate>,
    // Vector owning all instantiated blocks. This makes it easy to manage the blocks after they
    // have been instantiated.
    blocks: Vec<Entity<Block>>,
}

impl Spawner {
    const SPEED: f64 = 500.0;

    fn new(
        renderable_entities: EntityContainer<dyn Renderable>,
        fixed_update_entities: EntityContainer<dyn FixedUpdate>,
    ) -> Self {
        Self {
            rect: Rect::from_center(0, -200, 100, 20).with_color(&Color::GREEN),
            movement: Movement::default(),
            // Clone the entity containers.
            renderable_entities,
            fixed_update_entities,
            blocks: Vec::new(),
        }
    }
}

impl Update for Spawner {
    fn update(&mut self, _: f64) {
        // The internal implementation of EntityContainer means that if an entity is dropped, its
        // references in the corresponding container(s) will also be removed.
        self.blocks
            .retain(|block| block.lock().unwrap().rect.position.y < 100);
    }
}

impl Interactive for Spawner {
    fn on_event(&mut self, event: &Event) {
        match event {
            Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => {
                if *keycode == Keycode::A {
                    self.movement.left = true;
                } else if *keycode == Keycode::D {
                    self.movement.right = true;
                } else if *keycode == Keycode::Space {
                    // Instantiate a block.
                    let block = entity!(Block::new(&self.rect.center()));
                    self.renderable_entities
                        .add_entities(&entities!(Renderable; block));
                    self.fixed_update_entities
                        .add_entities(&entities!(FixedUpdate; block));
                    // blocks is the new owner of the newly-instantiated block entity.
                    self.blocks.push(block);
                }
            }
            Event::KeyUp {
                keycode: Some(keycode),
                ..
            } => {
                if *keycode == Keycode::A {
                    self.movement.left = false;
                } else if *keycode == Keycode::D {
                    self.movement.right = false;
                }
            }
            _ => {}
        }
    }
}

impl FixedUpdate for Spawner {
    fn fixed_update(&mut self, delta: f64) {
        if self.movement.left {
            self.rect.position.x -= (Self::SPEED * delta) as i32;
        }
        if self.movement.right {
            self.rect.position.x += (Self::SPEED * delta) as i32;
        }
    }
}

impl Renderable for Spawner {
    fn render(&self, camera: &Camera, context: &mut RenderContext) {
        self.rect.render(camera, context);
    }
}

fn main() {
    let mut renderer = Renderer::default().with_camera(Camera::default());
    let mut game = Game::new();
    // References to entity containers are passed to spawner to allow it to instantiate entities
    // during run-time.
    let spawner = entity!(Spawner::new(
        EntityContainer::clone(&game.renderable_entities),
        EntityContainer::clone(&game.fixed_update_entities)
    ));
    game.update_entities
        .add_entities(&entities!(Update; spawner));
    game.interactive_entities
        .add_entities(&entities!(Interactive; spawner));
    game.fixed_update_entities
        .add_entities(&entities!(FixedUpdate; spawner));
    game.fixed_update_entities
        .add_entities(&entities!(FixedUpdate; spawner));
    game.renderable_entities
        .add_entities(&entities!(Renderable; spawner));
    game.start(&mut renderer).unwrap();
}
