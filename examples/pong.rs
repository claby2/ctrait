use ctrait::{
    camera::Camera,
    entity, entity_clone, entity_slice,
    game::{Entity, Game},
    math::Vector2,
    rect::Rect,
    renderer::{CanvasWindow, Renderer},
    traits::{FixedUpdate, Interactive, Renderable, Update},
    Color, Event, Keycode,
};

#[derive(Debug, Default)]
struct Movement {
    up: bool,
    down: bool,
}

#[derive(Debug)]
struct Paddle {
    rect: Rect,
    movement: Movement,
    up_key: Keycode,
    down_key: Keycode,
}

impl Paddle {
    const SPEED: f64 = 600.0;

    fn new(x: i32, up_key: Keycode, down_key: Keycode) -> Self {
        Self {
            rect: Rect::with_center(x, 0, 20, 80),
            movement: Movement::default(),
            up_key,
            down_key,
        }
    }
}

impl FixedUpdate for Paddle {
    fn fixed_update(&mut self, delta: f64) {
        if self.movement.up {
            self.rect.position.y -= (Self::SPEED * delta) as i32;
        }
        if self.movement.down {
            self.rect.position.y += (Self::SPEED * delta) as i32;
        }
    }
}

impl Interactive for Paddle {
    fn on_event(&mut self, event: &Event) {
        match event {
            Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => {
                if *keycode == self.up_key {
                    self.movement.up = true;
                } else if *keycode == self.down_key {
                    self.movement.down = true;
                }
            }
            Event::KeyUp {
                keycode: Some(keycode),
                ..
            } => {
                if *keycode == self.up_key {
                    self.movement.up = false;
                } else if *keycode == self.down_key {
                    self.movement.down = false;
                }
            }
            _ => {}
        };
    }
}

impl Renderable for Paddle {
    fn render(&self, camera: &Camera, canvas: &mut CanvasWindow) {
        let canvas_rect = self.rect.to_canvas_rect(camera);
        if camera.is_canvas_rect_visible(&canvas_rect) {
            canvas.set_draw_color(Color::WHITE);
            canvas.fill_rect(canvas_rect).unwrap();
        }
    }
}

#[derive(Debug)]
struct Ball {
    rect: Rect,
    velocity: Vector2<f64>,
    camera: Entity<Camera>,
    paddle1: Entity<Paddle>,
    paddle2: Entity<Paddle>,
}

impl Ball {
    const SPEED: f64 = 800.0;
    fn new(camera: Entity<Camera>, paddle1: Entity<Paddle>, paddle2: Entity<Paddle>) -> Self {
        Self {
            rect: Rect::with_center(0, 0, 10, 10),
            velocity: Vector2::new(-Self::SPEED, 0.0),
            camera,
            paddle1,
            paddle2,
        }
    }

    fn calculate_y_velocity(paddle_movement: &Movement) -> f64 {
        if paddle_movement.up {
            -Self::SPEED
        } else if paddle_movement.down {
            Self::SPEED
        } else {
            0.0
        }
    }
}

impl Update for Ball {
    fn update(&mut self, _: f64) {
        let camera = self.camera.lock().unwrap();
        let canvas_rect = self.rect.to_canvas_rect(&camera);
        // Check if the Ball's x position is outside of the canvas.
        if canvas_rect.x < 0 || canvas_rect.x as u32 + canvas_rect.width() >= camera.canvas_size().x
        {
            // Reset position.
            self.rect.position = Vector2::new(-5, -5);
            self.velocity.y = 0.0;
        } else if canvas_rect.y < 0
            || canvas_rect.y as u32 + canvas_rect.height() >= camera.canvas_size().y
        {
            self.velocity.y *= -1.0;
        } else {
            let paddle1 = self.paddle1.lock().unwrap();
            let paddle2 = self.paddle2.lock().unwrap();
            if paddle1.rect.intersects(&self.rect) {
                self.velocity.x = Self::SPEED;
                self.velocity.y = Ball::calculate_y_velocity(&paddle1.movement);
            } else if paddle2.rect.intersects(&self.rect) {
                self.velocity.x = -Self::SPEED;
                self.velocity.y = Ball::calculate_y_velocity(&paddle2.movement);
            }
        }
    }
}

impl FixedUpdate for Ball {
    fn fixed_update(&mut self, delta: f64) {
        self.rect.position += Vector2::cast(&(self.velocity * delta)).unwrap();
    }
}

impl Renderable for Ball {
    fn render(&self, camera: &Camera, canvas: &mut CanvasWindow) {
        let canvas_rect = self.rect.to_canvas_rect(camera);
        if camera.is_canvas_rect_visible(&canvas_rect) {
            canvas.set_draw_color(Color::WHITE);
            canvas.fill_rect(canvas_rect).unwrap();
        }
    }
}

fn main() {
    // Define the camera as an entity so it can be referred to by Ball.
    let camera = entity!(Camera::default());
    let mut renderer = Renderer::new(None).unwrap().with_camera_entity(&camera);
    let paddle1 = entity!(Paddle::new(-400, Keycode::W, Keycode::S));
    let paddle2 = entity!(Paddle::new(400, Keycode::Up, Keycode::Down));
    let ball = entity!(Ball::new(
        camera,
        entity_clone!(paddle1),
        entity_clone!(paddle2)
    ));
    let mut game = Game::default();
    game.update_entities.push(&entity_clone!(Update, ball));
    game.fixed_update_entities.extend_from_slice(&entity_slice!(
        FixedUpdate,
        paddle1,
        paddle2,
        ball
    ));
    game.renderable_entities
        .extend_from_slice(&entity_slice!(Renderable, paddle1, paddle2, ball));
    game.interactive_entities
        .extend_from_slice(&entity_slice!(Interactive, paddle1, paddle2));
    game.start(&mut renderer);
}
