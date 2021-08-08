use crate::error::CtraitResult;
use sdl2::{video::Window, VideoSubsystem};

/// Configuration for [`crate::render::Renderer`].
#[derive(Debug)]
pub struct RendererConfig {
    pub dimensions: Option<(u32, u32)>,
    pub title: String,
    pub position: Option<(i32, i32)>,
    pub fullscreen: bool,
    pub opengl: bool,
    pub borderless: bool,
    pub resizable: bool,
    pub minimized: bool,
    pub maximized: bool,
    pub input_grabbed: bool,
    pub fullscreen_desktop: bool,
    pub allow_highdpi: bool,
    pub vulkan: bool,
}

impl RendererConfig {
    pub const FALLBACK_WIDTH: u32 = 640;
    pub const FALLBACK_HEIGHT: u32 = 480;

    /// Get the dimensions specified in the configuration. If dimensions is [`None`], returns
    /// fallback dimensions derived from [`Self::FALLBACK_WIDTH`] and [`Self::FALLBACK_HEIGHT`].
    pub fn dimensions(&self) -> (u32, u32) {
        if let Some(dimensions) = self.dimensions {
            dimensions
        } else {
            (
                RendererConfig::FALLBACK_WIDTH,
                RendererConfig::FALLBACK_HEIGHT,
            )
        }
    }

    // Apply configuration to build window.
    pub(crate) fn get_window(&self, video_subsystem: &VideoSubsystem) -> CtraitResult<Window> {
        let (width, height) = self.dimensions();
        let mut window = &mut video_subsystem.window(&self.title, width, height);
        macro_rules! set_flag {
            ($flag:expr, $new:expr) => {
                if $flag {
                    window = $new;
                }
            };
        }
        set_flag!(self.fullscreen, window.fullscreen());
        set_flag!(self.opengl, window.opengl());
        set_flag!(self.borderless, window.borderless());
        set_flag!(self.resizable, window.resizable());
        set_flag!(self.minimized, window.minimized());
        set_flag!(self.maximized, window.maximized());
        set_flag!(self.input_grabbed, window.input_grabbed());
        set_flag!(self.fullscreen_desktop, window.fullscreen_desktop());
        set_flag!(self.allow_highdpi, window.allow_highdpi());
        set_flag!(self.vulkan, window.vulkan());
        Ok(window.build()?)
    }
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self {
            title: String::from("ctrait"),
            dimensions: None,
            position: None,
            fullscreen: false,
            opengl: false,
            borderless: false,
            // Set resizable as default window behavior.
            resizable: true,
            minimized: false,
            maximized: false,
            input_grabbed: false,
            fullscreen_desktop: false,
            allow_highdpi: false,
            vulkan: false,
        }
    }
}