use crate::{
    camera::Camera,
    error::CtraitResult,
    game::{Entity, EntityContainer},
    traits::{Interactive, Renderable},
};
pub use sdl2::render::WindowCanvas;
use sdl2::{self, event::Event, pixels::Color, video::Window, EventPump, VideoSubsystem};

/// Configuration for [`Renderer`].
#[derive(Debug)]
pub struct RendererConfig {
    pub title: String,
    pub dimensions: Option<(u32, u32)>,
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

    // Apply appropriate configuration to build window.
    fn get_window(&self, video_subsystem: &VideoSubsystem) -> CtraitResult<Window> {
        let (width, height) = if let Some(dimensions) = self.dimensions {
            (dimensions.0, dimensions.1)
        } else {
            // width and height are required parameters, fallback to default if dimensions is None.
            (Self::FALLBACK_WIDTH, Self::FALLBACK_HEIGHT)
        };
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

/// Renders entities.
pub struct Renderer {
    pub canvas: WindowCanvas,
    event_pump: EventPump,
    quit: bool,
    camera: Option<Entity<Camera>>,
}

impl Renderer {
    /// Construct a new renderer with an optional configuration.
    ///
    /// Creating the renderer may return [`crate::error::CtraitError`] if [`sdl2`] fails to start.
    ///
    /// If the configuration is [`None`], the default configuration will be used
    /// ([`RendererConfig::default`]).
    ///
    /// # Example
    /// ```no_run
    /// use ctrait::renderer::{Renderer, RendererConfig};
    ///
    /// // Create renderer with default configuration.
    /// let default_renderer = Renderer::new(None).unwrap();
    ///
    /// // Create renderer with custom configuration.
    /// let custom_renderer = Renderer::new(Some(
    ///     RendererConfig {
    ///         title: String::from("Custom Renderer"),
    ///         dimensions: Some((100, 100)),
    ///         resizable: false,
    ///         // Let all other fields equal to default value.
    ///         ..Default::default()
    ///     }
    /// )).unwrap();
    /// ```
    pub fn new(config: Option<RendererConfig>) -> CtraitResult<Self> {
        let config = config.unwrap_or_else(RendererConfig::default);
        let sdl_context = sdl2::init()?;
        let event_pump = sdl_context.event_pump()?;
        let video_subsystem = sdl_context.video()?;
        let window = config.get_window(&video_subsystem)?;
        let canvas = window.into_canvas().build()?;
        Ok(Self {
            canvas,
            event_pump,
            quit: false,
            camera: None,
        })
    }

    /// Attach a camera to the renderer.
    /// A camera is **required** to render [`Renderable`] entities.
    ///
    /// # Example
    /// ```no_run
    /// use ctrait::{camera::Camera, renderer::Renderer};
    ///
    /// let renderer = Renderer::new(None).unwrap()
    ///     .with_camera(Camera::default());
    /// ```
    pub fn with_camera(mut self, camera: Camera) -> Self {
        self.camera = Some(crate::entity_clone!(&crate::entity!(camera)));
        self
    }

    /// Attach a reference counted camera to the renderer.
    /// Useful if you want to refer to the same camera elsewhere.
    ///
    /// # Example
    /// ```no_run
    /// use ctrait::{camera::Camera, entity, renderer::Renderer};
    ///
    /// let camera = entity!(Camera::default());
    /// // camera can now be passed to other entities...
    /// let renderer = Renderer::new(None).unwrap()
    ///     .with_camera_entity(&camera);
    /// ```
    pub fn with_camera_entity(mut self, camera: &Entity<Camera>) -> Self {
        self.camera = Some(crate::entity_clone!(&camera));
        self
    }

    // Check if quit has been requested.
    pub(crate) fn has_quit(&self) -> bool {
        self.quit
    }

    // Poll for pending events. Will mark quit as true if quit event was received.
    pub(crate) fn process_event(&mut self, entities: &mut EntityContainer<dyn Interactive>) {
        let entities = entities.access();
        for event in self.event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                self.quit = true;
                break;
            }
            entities
                .lock()
                .unwrap()
                .iter_mut()
                .for_each(|entity| entity.upgrade().unwrap().lock().unwrap().on_event(&event));
        }
    }

    // Render a vector of Rederable objects to canvas.
    pub(crate) fn render(&mut self, entities: &mut EntityContainer<dyn Renderable>) {
        if let Some(camera) = &mut self.camera {
            let mut camera = camera.lock().unwrap();
            camera.update(&self.canvas);
            self.canvas.set_draw_color(Color::BLACK);
            self.canvas.clear();
            for entity in entities.access().lock().unwrap().iter() {
                entity
                    .upgrade()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .render(&camera, &mut self.canvas);
            }
            self.canvas.present();
        }
    }
}
