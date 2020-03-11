use ::image;
pub use graphics::*;
use image::RgbaImage;
use opengl_graphics::{GlGraphics, Texture, TextureSettings};
use piston::input::RenderArgs;

use crate::caengine::SandBox;

pub struct CaRender {
    width: u32,
    height: u32,
}

impl CaRender {
    pub fn render_sandbox(&mut self, sandbox: &SandBox, gl: &mut GlGraphics, args: &RenderArgs) {

        //Turn the buffer into a texture
        let cell_tex = {
            let img: image::RgbaImage  = image::ImageBuffer::from_fn(
                self.width as u32, self.height as u32,
                |x, y| sandbox.to_rgba(x as i32, y as i32));


            Texture::from_image(
                &img,
                &TextureSettings::new(),
            )
        };

        gl.draw(args.viewport(), |c, g| {
            // Clear the screen
            graphics::clear(graphics::color::TRANSPARENT, g);
            //Draw image
            Image::new().draw(&cell_tex, &c.draw_state, c.transform, g);
        });
    }

    pub fn new(width: u32, height: u32) -> CaRender {
        CaRender {
            width,
            height,
        }
    }
}