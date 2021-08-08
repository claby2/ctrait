use crate::error::CtraitResult;
use sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};
use std::{collections::HashMap, rc::Rc};

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
