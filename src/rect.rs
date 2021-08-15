//! Rectangle.

use crate::{camera::Camera, graphics::RenderContext, math::Vector2, traits::Renderable};
use sdl2::{pixels::Color, rect::Rect as CanvasRect};

/// A rectangle relative to world coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    /// Position of top-left corner.
    pub position: Vector2<f32>,
    /// Width and height of rectangle.
    pub size: Vector2<f32>,
    /// Color of the rectangle. This must  be [`Some`] for the rectangle to be rendered.
    pub color: Option<Color>,
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            position: Vector2::zeros(),
            size: Vector2::zeros(),
            color: None,
        }
    }
}

impl Rect {
    /// Constructs a new rectangle.
    ///
    /// `x` and `y` represent the top-left corner of the rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// use ctrait::{math::Vector2, rect::Rect};
    ///
    /// let rect = Rect::new(-5.0, -5.0, 10.0, 10.0);
    /// // rect now represents a rectangle centered at (0, 0).
    /// assert_eq!(rect.center(), Vector2::repeat(0.0));
    /// ```
    #[must_use]
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            position: Vector2::new(x, y),
            size: Vector2::new(width, height),
            color: None,
        }
    }

    /// Constructs a new rectangle with the given x and y coordinates as the rectangle's center position.
    ///
    /// # Examples
    ///
    /// ```
    /// use ctrait::{math::Vector2, rect::Rect};
    ///
    /// let rect = Rect::from_center(0.0, 0.0, 10.0, 10.0);
    /// // rect now represents a rectangle centered at (0, 0).
    /// assert_eq!(rect.center(), Vector2::repeat(0.0));
    /// ```
    #[must_use]
    pub fn from_center(center_x: f32, center_y: f32, width: f32, height: f32) -> Self {
        Self::new(
            center_x - (width / 2.0),
            center_y - (height / 2.0),
            width,
            height,
        )
    }

    /// Construct rectangle with given color.
    ///
    /// # Examples
    ///
    /// ```
    /// use ctrait::{rect::Rect, Color};
    ///
    /// let rect = Rect::default().with_color(&Color::GRAY);
    /// assert_eq!(rect.color, Some(Color::GRAY));
    /// ```
    #[must_use]
    pub fn with_color(mut self, color: &Color) -> Self {
        self.color = Some(*color);
        self
    }

    /// Returns the center position as a [`Vector2`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ctrait::{math::Vector2, rect::Rect};
    ///
    /// let rect = Rect::from_center(1.0, 2.0, 3.0, 4.0);
    /// assert_eq!(rect.center(), Vector2::new(1.0, 2.0));
    /// ```
    #[must_use]
    pub fn center(&self) -> Vector2<f32> {
        self.position + self.size / 2.
    }

    /// Centers the rectangle on the given x and y coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use ctrait::{math::Vector2, rect::Rect};
    ///
    /// let mut rect = Rect::from_center(0.0, 0.0, 10.0, 10.0);
    /// // Set the rectangle's center point to (4, 5).
    /// rect.center_on(4.0, 5.0);
    /// assert_eq!(rect.center(), Vector2::new(4.0, 5.0));
    /// ```
    pub fn center_on(&mut self, center_x: f32, center_y: f32) {
        self.position = Vector2::new(center_x, center_y) - self.size / 2.;
    }

    /// Returns `true` if the rectangle has no area.
    ///
    /// # Examples
    ///
    /// ```
    /// use ctrait::rect::Rect;
    ///
    /// assert!(Rect::new(0.0, 0.0, 0.0, 0.0).is_empty());  // width = 0, height = 0
    /// assert!(Rect::new(0.0, 0.0, 1.0, 0.0).is_empty());  // width = 1, height = 0
    /// assert!(!Rect::new(0.0, 0.0, 1.0, 1.0).is_empty()); // width = 1, height = 1
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.size.x == 0. || self.size.y == 0.
    }

    /// Returns `true` if the given rectangle intersects.
    ///
    /// Will return `false` if either of the rectangles have no area.
    #[must_use]
    pub fn intersects(&self, other: &Rect) -> bool {
        // Special case if one of the rectangles have no area.
        if self.is_empty() || other.is_empty() {
            return false;
        }
        self.position.x < other.position.x + other.size.x
            && self.position.x + self.size.x > other.position.x
            && self.position.y < other.position.y + other.size.y
            && self.position.y + self.size.y > other.position.y
    }

    // Retrieves the equivalent CanvasRect relative to camera.
    // Will return None if the CanvasRect is outside of the camera's view.
    pub(crate) fn as_canvas_rect(&self, camera: &Camera) -> Option<CanvasRect> {
        let mut canvas_rect: CanvasRect = (*self).into();
        let new_position = camera.get_canvas_position(self.position);
        canvas_rect.x = new_position.x as i32;
        canvas_rect.y = new_position.y as i32;
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
    fn render(&self, camera: &Camera, context: &mut RenderContext) {
        if let Some(color) = self.color {
            if let Some(canvas_rect) = self.as_canvas_rect(camera) {
                context.canvas.set_draw_color(color);
                context.canvas.fill_rect(canvas_rect).unwrap();
            }
        } else {
            panic!("Rect must have defined color to be rendered");
        }
    }
}

impl From<Rect> for CanvasRect {
    fn from(rect: Rect) -> CanvasRect {
        CanvasRect::new(
            rect.position.x as i32,
            rect.position.y as i32,
            rect.size.x as u32,
            rect.size.y as u32,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Camera, CanvasRect, Color, Rect, Vector2};

    #[test]
    fn rect_default() {
        let rect = Rect::default();
        assert_eq!(rect.position, Vector2::zeros());
        assert_eq!(rect.size, Vector2::zeros());
        assert_eq!(rect.color, None);
    }

    #[test]
    fn rect_from_center() {
        let rect = Rect::from_center(0.0, 0.0, 10.0, 20.0);
        assert_eq!(rect, Rect::new(-5.0, -10.0, 10.0, 20.0));
    }

    #[test]
    fn rect_with_color() {
        let rect = Rect::new(0.0, 0.0, 10.0, 10.0).with_color(&Color::RED);
        assert_eq!(rect.color, Some(Color::RED));
    }

    #[test]
    fn rect_center() {
        let rect = Rect::new(0.0, 0.0, 10.0, 20.0);
        assert_eq!(rect.center(), Vector2::new(5.0, 10.0));
    }

    #[test]
    fn rect_center_on() {
        let mut rect = Rect::from_center(0.0, 0.0, 10.0, 10.0);
        rect.center_on(5.0, 5.0);
        assert_eq!(rect.center(), Vector2::repeat(5.0));
    }

    #[test]
    fn rect_as_canvas_rect_some() {
        let camera = Camera {
            canvas_size: Vector2::repeat(50),
            ..Camera::default()
        };
        let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
        let canvas_rect = rect.as_canvas_rect(&camera);
        assert_eq!(canvas_rect, Some(CanvasRect::new(25, 25, 10, 10)));
    }

    #[test]
    fn rect_as_canvas_rect_none() {
        let camera = Camera {
            canvas_size: Vector2::repeat(50),
            ..Camera::default()
        };
        let rect = Rect::new(100.0, 100.0, 10.0, 10.0);
        let canvas_rect = rect.as_canvas_rect(&camera);
        assert!(canvas_rect.is_none());
    }

    #[test]
    fn rect_is_empty() {
        let rect = Rect::new(0.0, 0.0, 0.0, 1.0);
        assert!(rect.is_empty());
    }

    #[test]
    fn rect_is_not_empty() {
        let rect = Rect::new(0.0, 0.0, 1.0, 1.0);
        assert!(!rect.is_empty());
    }

    #[test]
    fn rect_intersects() {
        let a = Rect::new(0.0, 0.0, 10.0, 10.0);
        let b = Rect::new(9.0, 9.0, 10.0, 3.0);
        assert!(a.intersects(&b));
        assert!(b.intersects(&a));
    }

    #[test]
    fn rect_no_intersects() {
        let a = Rect::new(0.0, 0.0, 10.0, 10.0);
        let b = Rect::new(11.0, 11.0, 10.0, 10.0);
        assert!(!a.intersects(&b));
        assert!(!b.intersects(&a));
    }

    #[test]
    fn rect_no_intersects_empty() {
        // a is empty.
        let a = Rect::new(0.0, 0.0, 10.0, 0.0);
        let b = Rect::new(9.0, 9.0, 10.0, 3.0);
        assert!(!a.intersects(&b));
        assert!(!b.intersects(&a));
    }
}
