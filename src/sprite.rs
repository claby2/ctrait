use crate::{camera::Camera, rect::Rect, render::RenderLayer, traits::Renderable};

/// A sprite which holds a path to a texture and a [`Rect`].
#[derive(Debug)]
pub struct Sprite {
    pub path: String,
    pub rect: Rect,
}

impl Sprite {
    /// Construct a new sprite.
    ///
    /// On render, the given texture path will be rendered onto the given [`Rect`].
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
    fn render(&self, camera: &Camera, layer: &mut RenderLayer) {
        if let Some(canvas_rect) = self.rect.to_canvas_rect(camera) {
            let texture = layer.texture_manager.load(&self.path).unwrap();
            layer.canvas.copy(&texture, None, canvas_rect).unwrap();
        }
    }
}
