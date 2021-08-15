use ctrait::{
    camera::Camera,
    entities, entity,
    entity::Entity,
    game::Game,
    graphics::{RenderContext, Renderer},
    math::Vector2,
    rect::Rect,
    traits::{Interactive, Renderable, Update},
    Color, Event,
};

#[derive(Debug)]
struct Cursor {
    rect: Rect,
    cursor_position: Vector2<f32>,
    camera: Entity<Camera>,
}

impl Cursor {
    const SIZE: f32 = 100.0;
    fn new(camera: Entity<Camera>) -> Self {
        Self {
            rect: Rect::from_center(0.0, 0.0, Self::SIZE, Self::SIZE).with_color(Color::WHITE),
            cursor_position: Vector2::zeros(),
            camera,
        }
    }
}

impl Interactive for Cursor {
    fn on_event(&mut self, event: &Event) {
        if let Event::MouseMotion { x, y, .. } = event {
            // Get cursor position relative to canvas.
            self.cursor_position = Vector2::new(*x, *y).cast();
        }
    }
}

impl Update for Cursor {
    fn update(&mut self, _: f32) {
        // The cursor position is relative to the canvas and not the world.
        // It must be converted first.
        let cursor_world_position = self
            .camera
            .lock()
            .unwrap()
            .get_world_position(self.cursor_position);
        // Center the cursor rect to the mouse cursor's world position.
        self.rect
            .center_on(cursor_world_position.x, cursor_world_position.y);
    }
}

impl Renderable for Cursor {
    fn render(&self, camera: &Camera, context: &mut RenderContext) {
        self.rect.render(camera, context);
    }
}

#[derive(Debug)]
struct Detector {
    rect: Rect,
    colliding: bool,
    cursor: Entity<Cursor>,
}

impl Detector {
    fn new(cursor: Entity<Cursor>) -> Self {
        Self {
            rect: Rect::from_center(0.0, 0.0, 300.0, 300.0),
            colliding: false,
            cursor,
        }
    }
}

impl Update for Detector {
    fn update(&mut self, _: f32) {
        self.colliding = self.cursor.lock().unwrap().rect.intersects(&self.rect);
        // Change the color of the rectangle depending on if it is colliding or not.
        self.rect.color = Some(if self.colliding {
            Color::GREEN
        } else {
            Color::WHITE
        });
    }
}

impl Renderable for Detector {
    fn render(&self, camera: &Camera, context: &mut RenderContext) {
        self.rect.render(camera, context);
    }
}

fn main() {
    // Camera is constructed as an entity so it can be referred to by other structs.
    let camera = entity!(Camera::default());
    let mut renderer = Renderer::default().with_camera_entity(Entity::clone(&camera));
    let cursor = entity!(Cursor::new(camera));
    let detector = entity!(Detector::new(Entity::clone(&cursor)));
    let mut game = Game::new();
    game.update_entities
        .add_entities(&entities!(Update; cursor, detector));
    game.interactive_entities
        .add_entities(&entities!(Interactive; cursor));
    // The detector is defined prior to cursor. This means the cursor is rendered on top of the
    // detector.
    game.renderable_entities
        .add_entities(&entities!(Renderable; detector, cursor));
    game.start(&mut renderer).unwrap();
}
