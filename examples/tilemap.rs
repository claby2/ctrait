use ctrait::{
    camera::Camera,
    entities, entity,
    game::Game,
    render::{RenderContext, Renderer},
    tile::{TileLayout, TileType, Tilemap},
    traits::Renderable,
    Color,
};

#[derive(Debug)]
pub struct World {
    // Create a tilemap with 2 rows and 3 columns.
    tilemap: Tilemap<2, 3>,
}

impl World {
    fn new() -> Self {
        Self {
            tilemap: Tilemap::new(
                // The tilemap has two possible tiles: a red square or a white square.
                &[TileType::Color(Color::RED), TileType::Color(Color::WHITE)],
                64,
            )
            // The layout represents how the tiles are arranged.
            .with_layout(
                TileLayout::new(&[
                    Some(0), // Red tile will be rendered at the top-left.
                    None,    // No tile will be rendered.
                    Some(1), // White tile will be rendered.
                    Some(1),
                    Some(0),
                    None,
                ])
                .unwrap(),
            ),
        }
    }
}

impl Renderable for World {
    fn render(&self, camera: &Camera, context: &mut RenderContext) {
        self.tilemap.render(camera, context);
    }
}

fn main() {
    let mut renderer = Renderer::default().with_camera(Camera::default());
    let world = entity!(World::new());
    let mut game = Game::new();
    game.renderable_entities
        .add_entities(&entities!(Renderable; world));
    game.start(&mut renderer).unwrap();
}
