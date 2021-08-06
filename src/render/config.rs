/// Configuration for [`Renderer`].
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
    const FALLBACK_WIDTH: u32 = 640;
    const FALLBACK_HEIGHT: u32 = 480;

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
