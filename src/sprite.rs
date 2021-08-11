//! Sprite used to display textures.

use crate::{camera::Camera, rect::Rect, render::RenderContext, traits::Renderable};
use std::path::PathBuf;

/// A sprite which holds a path to a texture and a [`Rect`].
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Sprite {
    /// Path to the texture.
    pub path: PathBuf,
    /// Rectangle representing the sprite's position and size.
    pub rect: Rect,
}

impl Sprite {
    /// Constructs a new sprite.
    ///
    /// When rendered, the given texture path will be rendered onto the given [`Rect`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ctrait::{rect::Rect, sprite::Sprite};
    /// use std::path::PathBuf;
    ///
    /// let sprite = Sprite::new("path/to/image.png", &Rect::from_center(0, 0, 10, 10));
    ///
    /// // To ensure cross-platform compatibility:
    /// let sprite_path: PathBuf = ["sprites", "sprite.jpg"].iter().collect();
    /// let another_sprite = Sprite::new(&sprite_path, &Rect::from_center(20, 20, 5, 5));
    /// ```
    pub fn new<P: Into<PathBuf>>(path: P, rect: &Rect) -> Self {
        Self {
            path: path.into(),
            rect: *rect,
        }
    }
}

impl Renderable for Sprite {
    fn render(&self, camera: &Camera, context: &mut RenderContext) {
        if let Some(canvas_rect) = self.rect.as_canvas_rect(camera) {
            let texture = context
                .texture_manager
                .load(&self.path.as_os_str().to_string_lossy())
                .unwrap();
            context.canvas.copy(&texture, None, canvas_rect).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PathBuf, Rect, Sprite};

    #[test]
    fn sprite_new() {
        let sprite_path: PathBuf = PathBuf::from("image.png");
        let sprite = Sprite::new(&sprite_path, &Rect::from_center(20, 20, 5, 5));
        assert_eq!(sprite.path.to_str(), Some("image.png"));
    }
}
