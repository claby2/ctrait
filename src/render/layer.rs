use crate::render::manager::TextureManager;
use sdl2::render::WindowCanvas;

/// Abstraction layer providing render functionality.
pub struct RenderLayer<'a> {
    pub canvas: WindowCanvas,
    pub texture_manager: TextureManager<'a>,
}

impl<'a> RenderLayer<'a> {
    pub(crate) fn new(canvas: WindowCanvas, texture_manager: TextureManager<'a>) -> Self {
        Self {
            canvas,
            texture_manager,
        }
    }
}
