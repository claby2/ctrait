//! Render functionality.

mod config;
mod context;
pub(crate) mod manager;
mod renderer;

pub use config::RendererConfig;
pub use context::RenderContext;
pub use renderer::Renderer;
