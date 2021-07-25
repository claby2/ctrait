use crate::{rect::Rect, renderer::CanvasWindow, vector::Vector2};
use sdl2::rect::Rect as SdlRect;

/// Camera that displays the entities rendered to the canvas.
#[derive(Debug, Default)]
pub struct Camera {
    pub position: Vector2<i32>,
    canvas_size: Vector2<u32>,
}

impl Camera {
    /// Returns a new rectangle representing the given rectangle's position and size on the canvas.
    /// The returned rectangle should be used when rendered to the canvas.
    pub fn get_canvas_rect(&self, rect: &Rect) -> SdlRect {
        let mut sdl_rect: SdlRect = rect.clone().into();
        sdl_rect.x = (rect.position.x - self.position.x) + (self.canvas_size.x / 2) as i32
            - (rect.size.x as i32 / 2);
        sdl_rect.y = (rect.position.y - self.position.y) + (self.canvas_size.y / 2) as i32
            - (rect.size.y as i32 / 2);
        sdl_rect
    }

    /// Returns whether or not the given rectangle would be visible on the canvas.
    /// The given rectangle should be derived from [`Camera::get_canvas_rect`].
    pub fn is_rect_visible(&self, rect: &SdlRect) -> bool {
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
    use super::{Camera, Rect, SdlRect, Vector2};

    #[test]
    fn camera_get_canvas_rect() {
        let camera = Camera {
            canvas_size: Vector2::new(50, 50),
            ..Default::default()
        };
        let canvas_rect = camera.get_canvas_rect(&Rect::new(0, 0, 10, 10));
        assert_eq!(canvas_rect, SdlRect::new(20, 20, 10, 10));
    }

    #[test]
    fn camera_detect_invisible_rect() {
        let camera = Camera {
            canvas_size: Vector2::new(50, 50),
            ..Default::default()
        };
        // Rectangle should be located outside canvas in top-left direction.
        assert!(!camera.is_rect_visible(&SdlRect::new(-10, -10, 10, 10)));
    }

    #[test]
    fn camera_detect_visible_rect() {
        let camera = Camera {
            canvas_size: Vector2::new(50, 50),
            ..Default::default()
        };
        // Rectangle should be located in the middle of canvas.
        assert!(camera.is_rect_visible(&SdlRect::new(20, 20, 10, 10)));
    }
}
