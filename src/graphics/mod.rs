//! Render functionality.
mod config;
mod renderer;

pub use config::RendererConfig;
pub use renderer::Renderer;

use crate::error::CtraitResult;
use sdl2::{
    image::LoadTexture,
    render::WindowCanvas,
    render::{Texture, TextureCreator},
    video::WindowContext,
};
use std::{collections::HashMap, rc::Rc};

/// Resource manager for [`Texture`]s.
///
/// This is one of the fields of [`RenderContext`] which can be accessed through the
/// [`Renderable`](crate::traits::Renderable) trait.
pub struct TextureManager<'a> {
    texture_creator: &'a TextureCreator<WindowContext>,
    cache: HashMap<String, Rc<Texture<'a>>>,
}

impl<'a> TextureManager<'a> {
    /// Create a texture manager from the given [`TextureCreator`].
    pub(crate) fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        Self {
            texture_creator,
            cache: HashMap::new(),
        }
    }

    /// Load and return a texture from the given path.
    ///
    /// The loaded texture is cached and will be retrieved if loaded again.
    ///
    /// # Errors
    ///
    /// This function will return an error if the texture fails to load.
    pub fn load(&mut self, path: &str) -> CtraitResult<Rc<Texture>> {
        self.cache.get(path).cloned().map_or_else(
            || {
                let resource = Rc::new(self.texture_creator.load_texture(path)?);
                self.cache.insert(path.to_string(), Rc::clone(&resource));
                Ok(resource)
            },
            Ok,
        )
    }
}

/// Abstraction layer providing render functionality.
///
/// This is used as an argument to the [`Renderable`](crate::traits::Renderable) trait.
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
