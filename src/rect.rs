use crate::vector::Vector2;
use sdl2::rect::Rect as SdlRect;

#[derive(Debug, Clone, Default)]
pub struct Rect {
    pub position: Vector2<i32>,
    pub size: Vector2<u32>,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            position: Vector2::new(x, y),
            size: Vector2::new(width, height),
        }
    }
}

impl From<Rect> for SdlRect {
    fn from(rect: Rect) -> SdlRect {
        SdlRect::new(rect.position.x, rect.position.y, rect.size.x, rect.size.y)
    }
}
