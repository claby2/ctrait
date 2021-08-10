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
//!     entity, entities,
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
//!             // Create a red rectangle with a width and height of 50 pixels.
//!             rect: Rect::from_center(0, 0, 50, 50).with_color(&Color::RED),
//!         }
//!     }
//! }
//!
//! // Allow the player to be rendered.
//! impl Renderable for Player {
//!     fn render(&self, camera: &Camera, context: &mut RenderContext) {
//!         // Since Rect implements Renderable, it can be rendered with a single function call.
//!         self.rect.render(camera, context);
//!     }
//! }
//!
//! let mut renderer = Renderer::default().with_camera(Camera::default());
//!
//! let player = entity!(Player::new());
//!
//! let mut game = Game::default();
//! // Register the player entity as a Renderable entity.
//! game.renderable_entities
//!     .add_entities(&entities!(Renderable; player));
//! game.start(&mut renderer).unwrap();
//! ```

pub mod camera;
pub mod error;
pub mod game;
pub mod rect;
pub mod render;
pub mod sprite;
pub mod tile;
pub mod traits;

// Re-export.
pub use cgmath as math;
pub use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
