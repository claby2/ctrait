use crate::error::CtraitResult;
use sdl2::{video::Window, VideoSubsystem};

/// Configuration for [`Renderer`](crate::render::Renderer).
#[derive(Debug)]
pub struct RendererConfig {
    /// Dimensions of the window.
    ///
    /// If this is [`None`], a [`FALLBACK_WIDTH`](Self::FALLBACK_WIDTH) and
    /// [`FALLBACK_HEIGHT`](Self::FALLBACK_HEIGHT) will be used.
    pub dimensions: Option<(u32, u32)>,
    /// Title of the window.
    pub title: String,
    /// Position of the window.
    pub position: Option<(i32, i32)>,
    /// Set the window to start in fullscreen mode.
    pub fullscreen: bool,
    /// Allow the window to be usable with OpenGL context.
    pub opengl: bool,
    /// Set window to borderless.
    pub borderless: bool,
    /// Allow the window to be resized.
    pub resizable: bool,
    /// Start the window as minimized.
    pub minimized: bool,
    /// Start the window as maximized.
    pub maximized: bool,
    /// Set the window to have grabbed input focus.
    pub input_grabbed: bool,
    /// Set the window to fullscreen at the current desktop resolution.
    pub fullscreen_desktop: bool,
    /// Creates the window in high-DPI mode.
    pub allow_highdpi: bool,
    /// Allow the window to be usable with Vulkan instance.
    pub vulkan: bool,
}

impl RendererConfig {
    /// Default window width.
    pub const FALLBACK_WIDTH: u32 = 640;
    /// Default window height.
    pub const FALLBACK_HEIGHT: u32 = 480;

    /// Get the dimensions specified in the configuration. If dimensions is [`None`], returns
    /// fallback dimensions derived from [`FALLBACK_WIDTH`](Self::FALLBACK_WIDTH) and [`FALLBACK_HEIGHT`](Self::FALLBACK_HEIGHT).
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

#[cfg(test)]
mod tests {
    use super::RendererConfig;

    #[test]
    fn renderer_config_set_dimensions() {
        let config = RendererConfig {
            dimensions: Some((5, 10)),
            ..Default::default()
        };
        assert_eq!(config.dimensions(), (5, 10));
    }

    #[test]
    fn renderer_config_fallback_dimensions() {
        let config = RendererConfig {
            dimensions: None,
            ..Default::default()
        };
        assert_eq!(
            config.dimensions(),
            (
                RendererConfig::FALLBACK_WIDTH,
                RendererConfig::FALLBACK_HEIGHT
            )
        );
    }
}
