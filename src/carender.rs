extern crate image;

use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use image::ImageBuffer;
use image::Rgba;

use crate::caengine::Cell;
use self::image::RgbaImage;

pub struct CaRender {
    gl: GlGraphics,
}

impl CaRender {
    pub fn render_sandbox(&mut self, cells: *const Cell, gl: *mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            let img: RgbaImage = ImageBuffer::from_fn(
                args.window_size[0] as u32, args.window_size[1] as u32,
                |
            );
            // Clear the screen
            clear(GREEN, gl);
            //Draw cells:

        });
    }
}