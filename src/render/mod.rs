//! Render functionality.
#![allow(clippy::module_name_repetitions)]

mod config;
pub(crate) mod manager;
mod renderer;

pub use config::RendererConfig;
pub use renderer::Renderer;

use manager::TextureManager;
use sdl2::render::WindowCanvas;

/// Abstraction layer providing render functionality.
///
/// This is used as an argument to the [`Renderable`](crate::traits::Renderable) trait.
#[allow(clippy::module_name_repetitions)]
pub struct RenderContext<'a> {
    /// Canvas used for rendering.
    pub canvas: WindowCanvas,
    /// Manager to organize and delegate the game's textures.
    pub texture_manager: TextureManager<'a>,
}

impl<'a> RenderContext<'a> {
    pub(crate) fn new(canvas: WindowCanvas, texture_manager: TextureManager<'a>) -> Self {
        Self {
            canvas,
            texture_manager,
        }
    }
}
