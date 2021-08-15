use crate::error::CtraitResult;
use sdl2::{render::WindowCanvas, video::Window, VideoSubsystem};

// Macro to quickly set builder pattern style flag based on condition.
macro_rules! set_flag {
    ($self:ident, $value:expr, $flag:ident) => {
        if $self.$flag {
            $value = $value.$flag();
        }
    };
}

/// Configuration for [`Renderer`](crate::graphics::Renderer).
#[allow(clippy::module_name_repetitions, clippy::struct_excessive_bools)]
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
    /// Centers the window.
    pub position_centered: bool,
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
    /// Use hardware acceleration.
    pub accelerated: bool,
    /// Use VSync.
    pub present_vsync: bool,
}

impl RendererConfig {
    /// Default window width.
    pub const FALLBACK_WIDTH: u32 = 640;
    /// Default window height.
    pub const FALLBACK_HEIGHT: u32 = 480;

    /// Get the dimensions specified in the configuration. If dimensions is [`None`], returns
    /// fallback dimensions derived from [`FALLBACK_WIDTH`](Self::FALLBACK_WIDTH) and [`FALLBACK_HEIGHT`](Self::FALLBACK_HEIGHT).
    #[must_use]
    pub fn dimensions(&self) -> (u32, u32) {
        self.dimensions.map_or(
            (
                RendererConfig::FALLBACK_WIDTH,
                RendererConfig::FALLBACK_HEIGHT,
            ),
            |dimensions| dimensions,
        )
    }

    fn create_window(&self, video_subsystem: &VideoSubsystem) -> CtraitResult<Window> {
        let (width, height) = self.dimensions();
        let mut window = &mut video_subsystem.window(&self.title, width, height);
        if let Some((x, y)) = self.position {
            window.position(x, y);
        }
        set_flag!(self, window, position_centered);
        set_flag!(self, window, fullscreen);
        set_flag!(self, window, opengl);
        set_flag!(self, window, borderless);
        set_flag!(self, window, resizable);
        set_flag!(self, window, minimized);
        set_flag!(self, window, maximized);
        set_flag!(self, window, input_grabbed);
        set_flag!(self, window, fullscreen_desktop);
        set_flag!(self, window, allow_highdpi);
        set_flag!(self, window, vulkan);
        Ok(window.build()?)
    }

    pub(crate) fn create_canvas(
        &self,
        video_subsystem: &VideoSubsystem,
    ) -> CtraitResult<WindowCanvas> {
        let mut canvas = self.create_window(video_subsystem)?.into_canvas();
        set_flag!(self, canvas, accelerated);
        set_flag!(self, canvas, present_vsync);
        Ok(canvas.build()?)
    }
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self {
            title: String::from("ctrait"),
            dimensions: None,
            position: None,
            position_centered: false,
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
            accelerated: false,
            present_vsync: false,
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
            ..RendererConfig::default()
        };
        assert_eq!(config.dimensions(), (5, 10));
    }

    #[test]
    fn renderer_config_fallback_dimensions() {
        let config = RendererConfig {
            dimensions: None,
            ..RendererConfig::default()
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
