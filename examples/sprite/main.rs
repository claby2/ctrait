use ctrait::{
    camera::Camera,
    entity, entity_clone,
    game::Game,
    render::{Renderer, TextureManager, WindowCanvas},
    traits::Renderable,
};
use std::env;

#[derive(Debug)]
struct Image {
    path: String,
}

impl Image {
    fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
}

impl Renderable for Image {
    fn render(&self, _: &Camera, canvas: &mut WindowCanvas, texture_manager: &mut TextureManager) {
        let texture = texture_manager.load(&self.path).unwrap();
        canvas.copy(&texture, None, None).unwrap();
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let path = &args[1];
    let mut game = Game::default();
    let mut renderer = Renderer::default().with_camera(Camera::default());
    let image = entity!(Image::new(path));
    game.renderable_entities
        .push(&entity_clone!(Renderable, image));
    game.start(&mut renderer).unwrap();
}
