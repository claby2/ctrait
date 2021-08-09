//! Sprite used to display textures.

use crate::{camera::Camera, rect::Rect, render::RenderContext, traits::Renderable};

/// A sprite which holds a path to a texture and a [`Rect`].
#[derive(Debug)]
pub struct Sprite {
    /// Path to the texture.
    pub path: String,
    /// Rectangle representing the sprite's position and size.
    pub rect: Rect,
}

impl Sprite {
    /// Constructs a new sprite.
    ///
    /// When rendered, the given texture path will be rendered onto the given [`Rect`].
    ///
    /// # Example
    /// ```
    /// use ctrait::{rect::Rect, sprite::Sprite};
    ///
    /// let sprite = Sprite::new("path/to/image.png", &Rect::from_center(0, 0, 10, 10));
    /// ```
    pub fn new(path: &str, rect: &Rect) -> Self {
        Self {
            path: path.to_string(),
            rect: rect.clone(),
        }
    }
}

impl Renderable for Sprite {
    fn render(&self, camera: &Camera, context: &mut RenderContext) {
        if let Some(canvas_rect) = self.rect.to_canvas_rect(camera) {
            let texture = context.texture_manager.load(&self.path).unwrap();
            context.canvas.copy(&texture, None, canvas_rect).unwrap();
        }
    }
}
