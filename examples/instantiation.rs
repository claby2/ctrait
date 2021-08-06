use ctrait::{
    camera::Camera,
    entity, entity_clone,
    game::{Entity, EntityContainer, Game},
    math::Vector2,
    rect::Rect,
    render::{Renderer, WindowCanvas},
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
    fn render(&self, camera: &Camera, canvas: &mut WindowCanvas) {
        self.rect.render(camera, canvas);
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
    blocks: Vec<Entity<Block>>,
}

impl Spawner {
    const SPEED: f64 = 500.0;

    fn new(
        renderable_entities: &EntityContainer<dyn Renderable>,
        fixed_update_entities: &EntityContainer<dyn FixedUpdate>,
    ) -> Self {
        Self {
            rect: Rect::from_center(0, -200, 100, 20).with_color(&Color::GREEN),
            movement: Movement::default(),
            renderable_entities: renderable_entities.clone(),
            fixed_update_entities: fixed_update_entities.clone(),
            blocks: Vec::new(),
        }
    }
}

impl Update for Spawner {
    fn update(&mut self, _: f64) {
        // The internal implementation of EntityContainer means that if an entity is dropped, its
        // references in the corresponding container(s) will be removed.
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
                        .push(&entity_clone!(Renderable, block));
                    self.fixed_update_entities
                        .push(&entity_clone!(FixedUpdate, block));
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
    fn render(&self, camera: &Camera, canvas: &mut WindowCanvas) {
        self.rect.render(camera, canvas);
    }
}

fn main() {
    let mut game = Game::default();
    let mut renderer = Renderer::new(None).unwrap().with_camera(Camera::default());
    let spawner = entity!(Spawner::new(
        &game.renderable_entities,
        &game.fixed_update_entities
    ));
    game.update_entities.push(&entity_clone!(Update, spawner));
    game.interactive_entities
        .push(&entity_clone!(Interactive, spawner));
    game.fixed_update_entities
        .push(&entity_clone!(FixedUpdate, spawner));
    game.fixed_update_entities
        .push(&entity_clone!(FixedUpdate, spawner));
    game.renderable_entities
        .push(&entity_clone!(Renderable, spawner));
    game.start(&mut renderer);
}
