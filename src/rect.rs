use crate::{camera::Camera, math::Vector2};
pub use sdl2::rect::Rect as CanvasRect;

/// A rectangle relative to world coordinates.
#[derive(Debug, Clone)]
pub struct Rect {
    pub position: Vector2<i32>,
    pub size: Vector2<u32>,
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            position: Vector2::new(0, 0),
            size: Vector2::new(0, 0),
        }
    }
}

impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            position: Vector2::new(x, y),
            size: Vector2::new(width, height),
        }
    }

    /// Copies `self` into a new [`CanvasRect`] relative to the given camera.
    ///
    /// # Example
    /// ```
    /// use ctrait::{camera::Camera, rect::Rect};
    ///
    /// let rect = Rect::new(0, 0, 10, 10);
    /// let canvas_rect = rect.to_canvas_rect(&Camera::default());
    /// // Here, `rect` and `canvas_rect` can be modified independently.
    /// ```
    pub fn to_canvas_rect(&self, camera: &Camera) -> CanvasRect {
        let mut sdl_rect: CanvasRect = self.clone().into();
        sdl_rect.x = (self.position.x - camera.position.x) + (camera.canvas_size.x / 2) as i32
            - (self.size.x as i32 / 2);
        sdl_rect.y = (self.position.y - camera.position.y) + (camera.canvas_size.y / 2) as i32
            - (self.size.y as i32 / 2);
        sdl_rect
    }
}

impl From<Rect> for CanvasRect {
    fn from(rect: Rect) -> CanvasRect {
        CanvasRect::new(rect.position.x, rect.position.y, rect.size.x, rect.size.y)
    }
}

#[cfg(test)]
mod tests {
    use super::{Camera, CanvasRect, Rect, Vector2};

    #[test]
    fn rect_to_canvas_rect() {
        let camera = Camera {
            canvas_size: Vector2::new(50, 50),
            ..Default::default()
        };
        let rect = Rect::new(0, 0, 10, 10);
        let canvas_rect = rect.to_canvas_rect(&camera);
        assert_eq!(canvas_rect, CanvasRect::new(20, 20, 10, 10));
    }
}
