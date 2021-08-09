#![warn(missing_docs)]

//! # ctrait
//! Trait-based game engine for Rust programmers.
//!
//! # Basic Example
//! The following is a basic example which simply renders a red square.
//!
//! ```no_run
//! use ctrait::{
//!     camera::Camera,
//!     entity, entity_clone,
//!     game::Game,
//!     rect::Rect,
//!     render::{RenderContext, Renderer},
//!     traits::Renderable,
//!     Color,
//! };
//!
//! #[derive(Debug)]
//! struct Player {
//!     rect: Rect,
//! }
//!
//! impl Player {
//!     fn new() -> Self {
//!         Self {
//!             rect: Rect::from_center(0, 0, 50, 50).with_color(&Color::RED),
//!         }
//!     }
//! }
//!
//! impl Renderable for Player {
//!     fn render(&self, camera: &Camera, context: &mut RenderContext) {
//!         self.rect.render(camera, context);
//!     }
//! }
//!
//! let mut renderer = Renderer::default().with_camera(Camera::default());
//!
//! let player = entity!(Player::new());
//!
//! let mut game = Game::default();
//! game.renderable_entities
//!     .push(&entity_clone!(Renderable, player));
//! game.start(&mut renderer).unwrap();
//! ```

pub mod camera;
pub mod error;
pub mod game;
pub mod rect;
pub mod render;
pub mod sprite;
pub mod traits;

// Re-export.
pub use cgmath as math;
pub use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
