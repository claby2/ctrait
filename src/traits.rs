//! Traits that structs can implement.

use crate::{camera::Camera, render::RenderContext};
use sdl2::event::Event;

/// A type that should update every game loop iteration.
///
/// # Example
/// ```
/// use ctrait::traits::Update;
///
/// struct UpdateExample;
///
/// impl Update for UpdateExample {
///     fn update(&mut self, delta: f64) {
///         // Any code here will be executed every frame.
///     }
/// }
/// ```
pub trait Update: Send {
    /// Called once per game loop iteration.
    /// `delta` is the number of seconds since the last update.
    fn update(&mut self, delta: f64);
}

/// A type that should update every fixed timestep.
///
/// This should be used instead of [`Update`] for time-dependent operations.
///
/// # Example
/// ```
/// use ctrait::traits::FixedUpdate;
///
/// struct FixedUpdateExample;
///
/// impl FixedUpdate for FixedUpdateExample {
///     fn fixed_update(&mut self, delta: f64) {
///         // Any code here will be executed at a fixed rate.
///     }
/// }
/// ```
pub trait FixedUpdate: Send {
    /// `delta` is the number of seconds since the last update.
    /// It should be approximately equal to the default timestep
    /// [`Game::DEFAULT_TIMESTEP`](crate::game::Game::DEFAULT_TIMESTEP).
    fn fixed_update(&mut self, delta: f64);
}

/// A type that is responsive to user events.
///
/// # Example
/// ```
/// use ctrait::{Event, traits::Interactive};
///
/// struct InteractiveExample;
///
/// impl Interactive for InteractiveExample {
///     fn on_event(&mut self, event: &Event) {
///         match event {
///             Event::KeyDown {
///                 keycode: Some(keycode),
///                 ..
///             } => {
///                 // Handle key down.
///             }
///             Event::KeyUp {
///                 keycode: Some(keycode),
///                 ..
///             } => {
///                 // Handle key up.
///             }
///             _ => {}
///         }
///     }
/// }
/// ```
pub trait Interactive: Send {
    /// Called for each event in the event queue.
    fn on_event(&mut self, event: &Event);
}

/// A type that can be rendered.
///
/// # Example
/// ```
/// use ctrait::{camera::Camera, render::RenderContext, traits::Renderable};
///
/// struct RenderableExample;
///
/// impl Renderable for RenderableExample {
///     fn render(&self, camera: &Camera, context: &mut RenderContext) {
///         // Render logic goes here.
///     }
/// }
/// ```
pub trait Renderable: Send {
    /// Called by [`Renderer`](crate::render::Renderer).
    fn render(&self, camera: &Camera, context: &mut RenderContext);
}
