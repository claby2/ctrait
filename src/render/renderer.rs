use crate::{
    camera::Camera,
    entity::{Entities, Entity},
    render::{RenderContext, RendererConfig},
    traits::{Interactive, Renderable},
};
use sdl2::{self, event::Event, pixels::Color, EventPump};

/// Renders entities.
#[derive(Debug)]
pub struct Renderer {
    /// The renderer's current configuration.
    pub config: RendererConfig,
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
    /// # Examples
    ///
    /// ```
    /// use ctrait::render::{Renderer, RendererConfig};
    ///
    /// // Create renderer with custom configuration.
    /// let custom_renderer = Renderer::new(
    ///     RendererConfig {
    ///         title: String::from("Custom Renderer"),
    ///         dimensions: Some((100, 100)),
    ///         resizable: false,
    ///         // Let all other fields equal to default value.
    ///         ..RendererConfig::default()
    ///     }
    /// );
    ///
    /// // Create renderer with default configuration.
    /// let default_renderer = Renderer::default();
    /// ```
    #[must_use]
    pub fn new(config: RendererConfig) -> Self {
        Self {
            config,
            quit: false,
            camera: None,
        }
    }

    /// Attach a camera to the renderer.
    /// A camera is **required** to render [`Renderable`] entities.
    ///
    /// # Examples
    ///
    /// ```
    /// use ctrait::{camera::Camera, render::Renderer};
    ///
    /// let renderer = Renderer::default()
    ///     .with_camera(Camera::default());
    /// ```
    #[must_use]
    pub fn with_camera(mut self, camera: Camera) -> Self {
        self.camera = Some(crate::entity!(camera));
        self
    }

    /// Attach a reference counted camera to the renderer.
    /// Useful if you want to refer to the same camera elsewhere.
    ///
    /// # Examples
    ///
    /// ```
    /// use ctrait::{camera::Camera, entity, entity::Entity, render::Renderer, math::Vector2};
    ///
    /// fn increment_camera_x(camera: Entity<Camera>) {
    ///     camera.lock().unwrap().position.x += 1.0;
    /// }
    ///
    /// let camera = entity!(Camera::new(Vector2::repeat(0.0)));
    ///
    /// // camera can now be cloned and passed multiple times.
    /// increment_camera_x(Entity::clone(&camera));
    /// increment_camera_x(Entity::clone(&camera));
    ///
    /// assert_eq!(camera.lock().unwrap().position.x, 2.0);
    ///
    /// // There is no need to clone camera here because it is not being used after this point.
    /// let renderer = Renderer::default()
    ///     .with_camera_entity(camera);
    /// ```
    #[must_use]
    pub fn with_camera_entity(mut self, camera: Entity<Camera>) -> Self {
        self.camera = Some(camera);
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
        entities: &mut Entities<dyn Interactive>,
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
        context: &mut RenderContext,
        entities: &mut Entities<dyn Renderable>,
    ) {
        if let Some(camera) = &mut self.camera {
            let mut camera = camera.lock().unwrap();
            camera.update(&context.canvas);
            context.canvas.set_draw_color(Color::BLACK);
            context.canvas.clear();
            for entity in entities.access().lock().unwrap().iter() {
                entity
                    .upgrade()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .render(&camera, context);
            }
            context.canvas.present();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Camera, Renderer};

    #[test]
    fn renderer_with_camera() {
        let renderer = Renderer::default().with_camera(Camera::default());
        assert!(renderer.camera.is_some());
    }

    #[test]
    fn renderer_with_camera_entity() {
        let camera = crate::entity!(Camera::default());
        let renderer = Renderer::default().with_camera_entity(camera);
        assert!(renderer.camera.is_some());
    }
}
