use ctrait::{
    camera::Camera,
    entity, entity_clone, entity_slice,
    game::{Entity, Game},
    math::Vector2,
    rect::Rect,
    render::{RenderContext, Renderer},
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
            rect: Rect::from_center(x, 0, 20, 80).with_color(&Color::WHITE),
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
    fn render(&self, camera: &Camera, context: &mut RenderContext) {
        self.rect.render(camera, context);
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
            rect: Rect::from_center(0, 0, 10, 10).with_color(&Color::WHITE),
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
        let canvas_position = camera.get_canvas_position(self.rect.position);
        // Check if the Ball's x position is outside of the canvas.
        if canvas_position.x < 0
            || canvas_position.x as u32 + self.rect.size.x >= camera.canvas_size().x
        {
            // Reset position to center.
            self.rect.center_on(0, 0);
            self.velocity.y = 0.0;
        } else if canvas_position.y < 0
            || canvas_position.y as u32 + self.rect.size.y >= camera.canvas_size().y
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
    fn render(&self, camera: &Camera, context: &mut RenderContext) {
        self.rect.render(camera, context);
    }
}

fn main() {
    // Define the camera as an entity so it can be referred to by Ball.
    let camera = entity!(Camera::default());
    let mut renderer = Renderer::default().with_camera_entity(&camera);
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
    game.start(&mut renderer).unwrap();
}
