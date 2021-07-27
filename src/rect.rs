use crate::{camera::Camera, math::Vector2};
pub use sdl2::rect::Rect as CanvasRect;

/// A rectangle relative to world coordinates.
#[derive(Debug, Clone, PartialEq)]
pub struct Rect {
    /// Position of top-left corner.
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
    /// Construct a new rectangle.
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            position: Vector2::new(x, y),
            size: Vector2::new(width, height),
        }
    }

    /// Construct a new rectangle with the given x and y coordinates as the rectangle's center position.
    pub fn with_center(center_x: i32, center_y: i32, width: u32, height: u32) -> Self {
        Self::new(
            center_x - (width / 2) as i32,
            center_y - (height / 2) as i32,
            width,
            height,
        )
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
        let new_position = camera.get_canvas_position(self.position);
        sdl_rect.x = new_position.x;
        sdl_rect.y = new_position.y;
        sdl_rect
    }

    /// Returns `true` if the rectangle has no area.
    ///
    /// # Example
    /// ```
    /// use ctrait::rect::Rect;
    ///
    /// assert!(Rect::new(0, 0, 0, 0).is_empty());  // width = 0, height = 0
    /// assert!(Rect::new(0, 0, 1, 0).is_empty());  // width = 1, height = 0
    /// assert!(!Rect::new(0, 0, 1, 1).is_empty()); // width = 1, height = 1
    /// ```
    pub fn is_empty(&self) -> bool {
        self.size.x == 0 || self.size.y == 0
    }

    /// Returns `true` if the given rectangle intersects.
    ///
    /// Will return `false` if either of the rectangles have no area.
    pub fn intersects(&self, other: &Rect) -> bool {
        // Special case if one of the rectangles have no area.
        if self.is_empty() || other.is_empty() {
            return false;
        }
        self.position.x < other.position.x + other.size.x as i32
            && self.position.x + self.size.x as i32 > other.position.x
            && self.position.y < other.position.y + other.size.y as i32
            && self.position.y + self.size.y as i32 > other.position.y
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
    fn rect_with_center() {
        let rect = Rect::with_center(0, 0, 10, 20);
        assert_eq!(rect, Rect::new(-5, -10, 10, 20));
    }

    #[test]
    fn rect_to_canvas_rect() {
        let camera = Camera {
            canvas_size: Vector2::new(50, 50),
            ..Default::default()
        };
        let rect = Rect::new(0, 0, 10, 10);
        let canvas_rect = rect.to_canvas_rect(&camera);
        assert_eq!(canvas_rect, CanvasRect::new(25, 25, 10, 10));
    }

    #[test]
    fn rect_is_empty() {
        let rect = Rect::new(0, 0, 0, 1);
        assert!(rect.is_empty());
    }

    #[test]
    fn rect_is_not_empty() {
        let rect = Rect::new(0, 0, 1, 1);
        assert!(!rect.is_empty());
    }

    #[test]
    fn rect_intersects() {
        let a = Rect::new(0, 0, 10, 10);
        let b = Rect::new(9, 9, 10, 3);
        assert!(a.intersects(&b));
    }

    #[test]
    fn rect_no_intersects() {
        let a = Rect::new(0, 0, 10, 10);
        let b = Rect::new(11, 11, 10, 10);
        assert!(!a.intersects(&b));
    }
}
