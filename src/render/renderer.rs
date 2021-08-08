use crate::{
    camera::Camera,
    game::{Entity, EntityContainer},
    render::{RendererConfig, TextureManager, WindowCanvas},
    traits::{Interactive, Renderable},
};
use sdl2::{self, event::Event, pixels::Color, EventPump};

/// Renders entities.
pub struct Renderer {
    pub config: RendererConfig,
    texture_paths: Vec<String>,
    quit: bool,
    camera: Option<Entity<Camera>>,
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new(RendererConfig::default())
    }
}

impl Renderer {
    /// Construct a new renderer with a custom configuration.
    ///
    /// # Example
    /// ```no_run
    /// use ctrait::render::{Renderer, RendererConfig};
    ///
    /// // Create renderer with custom configuration.
    /// let custom_renderer = Renderer::new(
    ///     RendererConfig {
    ///         title: String::from("Custom Renderer"),
    ///         dimensions: Some((100, 100)),
    ///         resizable: false,
    ///         // Let all other fields equal to default value.
    ///         ..Default::default()
    ///     }
    /// );
    ///
    /// // Create renderer with default configuration.
    /// let default_renderer = Renderer::default();
    /// ```
    pub fn new(config: RendererConfig) -> Self {
        Self {
            config,
            texture_paths: Vec::new(),
            quit: false,
            camera: None,
        }
    }

    pub fn with_texture_paths(mut self, textures: &[&str]) {
        self.texture_paths = textures.iter().map(|s| s.to_string()).collect();
    }

    pub fn texture_paths(&mut self) -> &Vec<String> {
        &self.texture_paths
    }

    /// Attach a camera to the renderer.
    /// A camera is **required** to render [`Renderable`] entities.
    ///
    /// # Example
    /// ```no_run
    /// use ctrait::{camera::Camera, render::Renderer};
    ///
    /// let renderer = Renderer::default()
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
    /// use ctrait::{camera::Camera, entity, render::Renderer};
    ///
    /// let camera = entity!(Camera::default());
    /// // camera can now be passed to other entities...
    /// let renderer = Renderer::default()
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
    pub(crate) fn process_event(
        &mut self,
        event_pump: &mut EventPump,
        entities: &mut EntityContainer<dyn Interactive>,
    ) {
        let entities = entities.access();
        for event in event_pump.poll_iter() {
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
    pub(crate) fn render(
        &mut self,
        canvas: &mut WindowCanvas,
        texture_manager: &mut TextureManager,
        entities: &mut EntityContainer<dyn Renderable>,
    ) {
        if let Some(camera) = &mut self.camera {
            let mut camera = camera.lock().unwrap();
            camera.update(canvas);
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();
            for entity in entities.access().lock().unwrap().iter() {
                entity
                    .upgrade()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .render(&camera, canvas, texture_manager);
            }
            canvas.present();
        }
    }
}
