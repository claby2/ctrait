use ctrait::{
    camera::Camera,
    entity, entity_clone,
    game::Game,
    render::{RenderLayer, Renderer},
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
    fn render(&self, _: &Camera, layer: &mut RenderLayer) {
        let texture = layer.texture_manager.load(&self.path).unwrap();
        layer.canvas.copy(&texture, None, None).unwrap();
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
