use crate::{camera::Camera, math::Vector2, render::RenderLayer, traits::Renderable};
use sdl2::{pixels::Color, rect::Rect as CanvasRect};

/// A rectangle relative to world coordinates.
#[derive(Debug, Clone, PartialEq)]
pub struct Rect {
    /// Position of top-left corner.
    pub position: Vector2<i32>,
    pub size: Vector2<u32>,
    pub color: Option<Color>,
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            position: Vector2::new(0, 0),
            size: Vector2::new(0, 0),
            color: None,
        }
    }
}

impl Rect {
    /// Construct a new rectangle.
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            position: Vector2::new(x, y),
            size: Vector2::new(width, height),
            color: None,
        }
    }

    /// Construct a new rectangle with the given x and y coordinates as the rectangle's center position.
    pub fn from_center(center_x: i32, center_y: i32, width: u32, height: u32) -> Self {
        Self::new(
            center_x - (width / 2) as i32,
            center_y - (height / 2) as i32,
            width,
            height,
        )
    }

    // Set the render color.
    pub fn with_color(mut self, color: &Color) -> Self {
        self.color = Some(*color);
        self
    }

    /// Return the center position as a [`Vector2`].
    pub fn center(&self) -> Vector2<i32> {
        self.position + Vector2::cast(&(self.size / 2)).unwrap()
    }

    // Centers the rectangle on the given x and y coordinates.
    pub fn center_on(&mut self, center_x: i32, center_y: i32) {
        self.position = Vector2::new(
            center_x - (self.size.x / 2) as i32,
            center_y - (self.size.y / 2) as i32,
        );
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

    // Get the equivalent CanvasRect relative to camera.
    // Will return None if the CanvasRect is outside of the camera's view.
    pub(crate) fn to_canvas_rect(&self, camera: &Camera) -> Option<CanvasRect> {
        let mut canvas_rect: CanvasRect = self.clone().into();
        let new_position = camera.get_canvas_position(self.position);
        canvas_rect.x = new_position.x;
        canvas_rect.y = new_position.y;
        if canvas_rect.x < camera.canvas_size.x as i32
            && (canvas_rect.x + canvas_rect.width() as i32) > 0
            && canvas_rect.y < camera.canvas_size.y as i32
            && (canvas_rect.y + canvas_rect.height() as i32) > 0
        {
            Some(canvas_rect)
        } else {
            // canvas_rect is positioned outside of camera's view, return None.
            None
        }
    }
}

impl Renderable for Rect {
    #[track_caller]
    fn render(&self, camera: &Camera, layer: &mut RenderLayer) {
        if let Some(color) = self.color {
            if let Some(canvas_rect) = self.to_canvas_rect(camera) {
                layer.canvas.set_draw_color(color);
                layer.canvas.fill_rect(canvas_rect).unwrap();
            }
        } else {
            panic!("Rect must have defined color to be rendered");
        }
    }
}

impl From<Rect> for CanvasRect {
    fn from(rect: Rect) -> CanvasRect {
        CanvasRect::new(rect.position.x, rect.position.y, rect.size.x, rect.size.y)
    }
}

#[cfg(test)]
mod tests {
    use super::{Camera, CanvasRect, Color, Rect, Vector2};

    #[test]
    fn rect_from_center() {
        let rect = Rect::from_center(0, 0, 10, 20);
        assert_eq!(rect, Rect::new(-5, -10, 10, 20));
    }

    #[test]
    fn rect_with_color() {
        let rect = Rect::new(0, 0, 10, 10).with_color(&Color::RED);
        assert_eq!(rect.color, Some(Color::RED));
    }

    #[test]
    fn rect_center() {
        let rect = Rect::new(0, 0, 10, 20);
        assert_eq!(rect.center(), Vector2::new(5, 10));
    }

    #[test]
    fn rect_to_canvas_rect_some() {
        let camera = Camera {
            canvas_size: Vector2::new(50, 50),
            ..Default::default()
        };
        let rect = Rect::new(0, 0, 10, 10);
        let canvas_rect = rect.to_canvas_rect(&camera);
        assert_eq!(canvas_rect, Some(CanvasRect::new(25, 25, 10, 10)));
    }

    #[test]
    fn rect_to_canvas_rect_none() {
        let camera = Camera {
            canvas_size: Vector2::new(50, 50),
            ..Default::default()
        };
        let rect = Rect::new(100, 100, 10, 10);
        let canvas_rect = rect.to_canvas_rect(&camera);
        assert!(canvas_rect.is_none());
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
