pub mod camera;
pub mod error;
pub mod game;
pub mod rect;
pub mod render;
pub mod traits;

// Re-export.
pub use cgmath as math;
pub use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
