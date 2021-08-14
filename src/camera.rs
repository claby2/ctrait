//! Camera used to convert between world and canvas positions.

use crate::math::Vector2;
use sdl2::render::WindowCanvas;

/// Camera with a position used to calculate relative world and canvas positions.
#[derive(Debug, Copy, Clone)]
pub struct Camera {
    /// World position of the camera.
    pub position: Vector2<f32>,
    pub(crate) canvas_size: Vector2<u32>,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(Vector2::new(0.0, 0.0))
    }
}

impl Camera {
    /// Constructs a new camera with a given world position.
    ///
    /// # Examples
    ///
    /// ```
    /// use ctrait::{camera::Camera, math::Vector2};
    ///
    /// let camera = Camera::new(Vector2::new(5.0, 10.0));
    /// // camera is located at world position (5, 10).
    /// assert_eq!(camera.position, Vector2::new(5.0, 10.0));
    /// ```
    #[must_use]
    pub fn new(position: Vector2<f32>) -> Self {
        Self {
            position,
            canvas_size: Vector2::new(0, 0),
        }
    }

    /// Retrieves the size of the canvas.
    ///
    /// The value is internally updated once per game loop iteration.
    ///
    /// # Panics
    ///
    /// This function will panic if the width or height of the canvas exceeds [`f32::MAX`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ctrait::camera::Camera;
    ///
    /// let camera = Camera::default();
    /// let canvas_size = camera.canvas_size();
    /// println!("width: {}, height: {}", canvas_size.x, canvas_size.y);
    /// ```
    #[must_use]
    pub fn canvas_size(&self) -> Vector2<f32> {
        self.canvas_size.cast().unwrap()
    }

    /// Converts the given canvas position to its equivalent world position.
    #[must_use]
    pub fn get_world_position(&self, canvas_position: Vector2<f32>) -> Vector2<f32> {
        canvas_position + self.position - self.canvas_size() / 2.0
    }

    /// Converts the given world position to its equivalent canvas position.
    #[must_use]
    pub fn get_canvas_position(&self, world_position: Vector2<f32>) -> Vector2<f32> {
        world_position - self.position + self.canvas_size() / 2.0
    }

    pub(crate) fn update(&mut self, canvas: &WindowCanvas) {
        let (width, height) = canvas.output_size().unwrap();
        self.canvas_size = Vector2::new(width, height);
    }
}

#[cfg(test)]
mod tests {
    use super::{Camera, Vector2};

    #[test]
    fn camera_new() {
        let camera = Camera::new(Vector2::new(2.0, 3.0));
        assert_eq!(camera.position, Vector2::new(2.0, 3.0));
        assert_eq!(camera.canvas_size, Vector2::new(0, 0));
    }

    #[test]
    fn camera_canvas_size() {
        let camera = Camera {
            canvas_size: Vector2::new(50, 50),
            ..Camera::default()
        };
        assert_eq!(camera.canvas_size(), camera.canvas_size.cast().unwrap());
    }

    #[test]
    fn camera_get_world_position() {
        let camera = Camera {
            canvas_size: Vector2::new(50, 50),
            ..Camera::default()
        };
        let canvas_position = Vector2::new(35.0, 35.0);
        assert_eq!(
            camera.get_world_position(canvas_position),
            Vector2::new(10.0, 10.0)
        );
    }

    #[test]
    fn camera_get_canvas_position() {
        let camera = Camera {
            canvas_size: Vector2::new(50, 50),
            ..Camera::default()
        };
        let world_position = Vector2::new(10.0, 10.0);
        assert_eq!(
            camera.get_canvas_position(world_position),
            Vector2::new(35.0, 35.0)
        );
    }
}
