use ctrait::{
    camera::Camera,
    entity, entity_clone,
    game::Game,
    rect::Rect,
    render::{RenderContext, Renderer},
    sprite::Sprite,
    traits::Renderable,
};
use std::env;

#[derive(Debug)]
struct Image {
    sprite: Sprite,
}

impl Image {
    const SPRITE_SIZE: u32 = 256;

    fn new(path: &str) -> Self {
        Self {
            sprite: Sprite::new(
                path,
                &Rect::from_center(0, 0, Self::SPRITE_SIZE, Self::SPRITE_SIZE),
            ),
        }
    }
}

impl Renderable for Image {
    fn render(&self, camera: &Camera, context: &mut RenderContext) {
        self.sprite.render(camera, context);
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run /path/to/image.(png|jpg)");
    } else {
        let path = &args[1];
        let mut game = Game::default();
        let mut renderer = Renderer::default().with_camera(Camera::default());
        let image = entity!(Image::new(path));
        game.renderable_entities
            .push(&entity_clone!(Renderable, image));
        game.start(&mut renderer).unwrap();
    }
}
