use crate::{math::Vector2, rect::CanvasRect, renderer::CanvasWindow};

/// Camera that displays the entities rendered to the canvas.
#[derive(Debug)]
pub struct Camera {
    pub position: Vector2<i32>,
    pub(crate) canvas_size: Vector2<u32>,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vector2::new(0, 0),
            canvas_size: Vector2::new(0, 0),
        }
    }
}

impl Camera {
    /// Retrieve the size of the canvas.
    /// The value is updated once per game loop iteration.
    ///
    /// # Example
    /// ```
    /// use ctrait::{camera::Camera};
    ///
    /// let camera = Camera::default();
    /// let canvas_size = camera.canvas_size();
    /// println!("width: {}, height: {}", canvas_size.x, canvas_size.y);
    /// ```
    pub fn canvas_size(&self) -> &Vector2<u32> {
        &self.canvas_size
    }

    /// Returns whether or not the given rectangle would be visible on the canvas.
    /// The given rectangle should be derived from [`crate::rect::Rect::to_canvas_rect`].
    ///
    /// # Example
    /// ```
    /// use ctrait::{camera::Camera, rect::Rect};
    ///
    /// let camera = Camera::default();
    /// let canvas_rect = Rect::new(0, 0, 10, 10).to_canvas_rect(&camera);
    /// if camera.is_canvas_rect_visible(&canvas_rect) {
    ///     println!("The camera can see the rectangle.");
    /// } else {
    ///     println!("The camera is outside of the camera's view.");
    /// }
    /// ```
    pub fn is_canvas_rect_visible(&self, rect: &CanvasRect) -> bool {
        rect.x < self.canvas_size.x as i32
            && (rect.x + rect.width() as i32) > 0
            && rect.y < self.canvas_size.y as i32
            && (rect.y + rect.height() as i32) > 0
    }

    pub(crate) fn update(&mut self, canvas: &CanvasWindow) {
        let (width, height) = canvas.output_size().unwrap();
        self.canvas_size = Vector2::new(width, height);
    }
}

#[cfg(test)]
mod tests {
    use super::{Camera, CanvasRect, Vector2};

    #[test]
    fn camera_detect_invisible_rect() {
        let camera = Camera {
            canvas_size: Vector2::new(50, 50),
            ..Default::default()
        };
        // Rectangle should be located outside canvas in top-left direction.
        assert!(!camera.is_canvas_rect_visible(&CanvasRect::new(-10, -10, 10, 10)));
    }

    #[test]
    fn camera_detect_visible_rect() {
        let camera = Camera {
            canvas_size: Vector2::new(50, 50),
            ..Default::default()
        };
        // Rectangle should be located in the middle of canvas.
        assert!(camera.is_canvas_rect_visible(&CanvasRect::new(20, 20, 10, 10)));
    }
}
