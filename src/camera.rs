//! Camera used to convert between world and canvas positions.

use crate::math::Vector2;
use sdl2::render::WindowCanvas;

/// Camera with a position used to calculate relative world and canvas positions.
#[derive(Debug)]
pub struct Camera {
    /// World position of the camera.
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
    /// Retrieves the size of the canvas.
    ///
    /// The value is internally updated once per game loop iteration.
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

    /// Converts the given canvas position to its equivalent world position.
    pub fn get_world_position(&self, canvas_position: Vector2<i32>) -> Vector2<i32> {
        canvas_position + self.position - Vector2::cast(&(self.canvas_size / 2)).unwrap()
    }

    /// Converts the given world position to its equivalent canvas position.
    pub fn get_canvas_position(&self, world_position: Vector2<i32>) -> Vector2<i32> {
        world_position - self.position + Vector2::cast(&(self.canvas_size / 2)).unwrap()
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
    fn camera_canvas_size() {
        let camera = Camera {
            canvas_size: Vector2::new(50, 50),
            ..Default::default()
        };
        assert_eq!(*camera.canvas_size(), camera.canvas_size);
    }

    #[test]
    fn camera_get_world_position() {
        let camera = Camera {
            canvas_size: Vector2::new(50, 50),
            ..Default::default()
        };
        let canvas_position = Vector2::new(35, 35);
        assert_eq!(
            camera.get_world_position(canvas_position),
            Vector2::new(10, 10)
        );
    }

    #[test]
    fn camera_get_canvas_position() {
        let camera = Camera {
            canvas_size: Vector2::new(50, 50),
            ..Default::default()
        };
        let world_position = Vector2::new(10, 10);
        assert_eq!(
            camera.get_canvas_position(world_position),
            Vector2::new(35, 35)
        );
    }
}
