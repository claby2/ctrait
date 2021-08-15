use ctrait::{
    camera::Camera,
    entities, entity,
    entity::{Entities, Entity},
    game::Game,
    graphics::{RenderContext, Renderer},
    math::Vector2,
    rect::Rect,
    traits::{FixedUpdate, Interactive, Renderable, Update},
    Color, Event, Keycode,
};

// Rect that will be instantiated by the spawner.
#[derive(Debug)]
struct Block {
    rect: Rect,
}

impl Block {
    const FALL_SPEED: f32 = 600.0;

    fn new(position: Vector2<f32>) -> Self {
        Self {
            rect: Rect::from_center(position.x, position.y, 50.0, 50.0).with_color(&Color::GRAY),
        }
    }
}

impl FixedUpdate for Block {
    fn fixed_update(&mut self, delta: f32) {
        // Increase the y position to make the block fall.
        self.rect.position.y += Self::FALL_SPEED * delta;
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
    renderable_entities: Entities<dyn Renderable>,
    fixed_update_entities: Entities<dyn FixedUpdate>,
    // Vector owning all instantiated blocks. This makes it easy to manage the blocks after they
    // have been instantiated.
    blocks: Vec<Entity<Block>>,
}

impl Spawner {
    const SPEED: f32 = 500.0;

    fn new(
        renderable_entities: Entities<dyn Renderable>,
        fixed_update_entities: Entities<dyn FixedUpdate>,
    ) -> Self {
        Self {
            rect: Rect::from_center(0.0, -200.0, 100.0, 20.0).with_color(&Color::GREEN),
            movement: Movement::default(),
            renderable_entities,
            fixed_update_entities,
            blocks: Vec::new(),
        }
    }
}

impl Update for Spawner {
    fn update(&mut self, _: f32) {
        // The internal implementation of entity container means that if an entity is dropped, its
        // references in the corresponding container(s) will also be removed.
        self.blocks
            .retain(|block| block.lock().unwrap().rect.position.y < 100.0);
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
                    let block = entity!(Block::new(self.rect.center()));
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
    fn fixed_update(&mut self, delta: f32) {
        if self.movement.left {
            self.rect.position.x -= Self::SPEED * delta;
        }
        if self.movement.right {
            self.rect.position.x += Self::SPEED * delta;
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
        Entities::clone(&game.renderable_entities),
        Entities::clone(&game.fixed_update_entities)
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
