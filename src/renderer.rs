use crate::{
    camera::Camera,
    error::CtraitResult,
    game::EntityType,
    traits::{Interactive, Renderable},
};
use sdl2::{self, event::Event, pixels::Color, render::Canvas, video::Window, EventPump};

pub type CanvasWindow = Canvas<Window>;

/// Renders entities.
pub struct Renderer {
    pub canvas: CanvasWindow,
    event_pump: EventPump,
    quit: bool,
    camera: Option<Camera>,
}

impl Renderer {
    /// Construct a new renderer.
    /// May return [`crate::error::CtraitError`] if [`sdl2`] fails to start.
    pub fn initialize(title: &str, width: u32, height: u32) -> CtraitResult<Self> {
        let sdl_context = sdl2::init()?;
        let event_pump = sdl_context.event_pump()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window(title, width, height)
            .resizable()
            .build()
            .unwrap();
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
    pub fn with_camera(mut self, camera: Camera) -> Self {
        self.camera = Some(camera);
        self
    }

    // Check if quit has been requested.
    pub(crate) fn has_quit(&self) -> bool {
        self.quit
    }

    // Poll for pending events. Will mark quit as true if quit event was received.
    pub(crate) fn process_event(&mut self, entities: &mut Vec<EntityType<dyn Interactive>>) {
        for event in self.event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                self.quit = true;
                break;
            }
            entities.iter_mut().for_each(|entity| {
                entity.borrow_mut().on_event(&event);
            });
        }
    }

    // Render a vector of Rederable objects to canvas.
    pub(crate) fn render(&mut self, entities: &mut Vec<EntityType<dyn Renderable>>) {
        if let Some(camera) = &mut self.camera {
            camera.update(&self.canvas);
            self.canvas.set_draw_color(Color::BLACK);
            self.canvas.clear();
            for entity in entities {
                entity.borrow_mut().render(&camera, &mut self.canvas);
            }
            self.canvas.present();
        }
    }
}
