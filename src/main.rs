extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{Events, EventSettings};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

mod caengine;

pub struct App {
    gl: GlGraphics,
    sandbox: caengine::SandBox,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            clear(BLACK, gl);
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {}
}

fn main() {
    const W_HEIGHT: u32 = 200;
    const W_WIDTH: u32 = 200;
    const CELL_SIZE: u32 = 5;
    const HEIGHT: u32 = W_HEIGHT / CELL_SIZE;
    const WIDTH: u32 = W_HEIGHT / CELL_SIZE;

    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("sand-engine", [W_HEIGHT, W_WIDTH])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        //Handle user input

        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
