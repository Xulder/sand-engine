#![allow(dead_code)]

use glutin_window::GlutinWindow as Window;
pub use image;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{Events, EventSettings};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use time::{Duration, Instant};

use crate::caengine::{Cell, SandBox};
//use crate::caengine::celltypes::CellType;
use crate::carender::CaRender;

//Local modules
mod caengine;
mod carender;

//Constants
const W_HEIGHT: i32 = 500;
const W_WIDTH: i32 = 500;

//Simulation App
pub struct App {
    gl: GlGraphics,
    sandbox: SandBox,
    renderer: CaRender
}

//App implemented methods
impl App {
    fn render(&mut self, args: &RenderArgs) {
//        let now = Instant::now();
//        let delta = now -
        self.renderer.render_sandbox(&self.sandbox, &mut self.gl, args);
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.sandbox.tick();
    }
}

//main() function. Entry point of the simulation.
fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Sand", [W_WIDTH as u32, W_HEIGHT as u32])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .vsync(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        sandbox: SandBox::new(W_WIDTH, W_HEIGHT),
        renderer: CaRender::new(W_WIDTH as u32, W_HEIGHT as u32),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}