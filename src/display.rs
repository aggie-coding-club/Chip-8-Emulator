extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct Display {
    pub gl: GlGraphics, // OpenGL drawing backend.
}

impl Display {
    //render is called to draw the display
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        //define the colors used for the dispaly
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK:   [f32; 4] = [0.0, 0.0, 0.0, 0.0];
        let pixel = rectangle::square(0.0, 0.0, 10.0);
        let (x, y) = (args.window_size[0] / 2.0,
                      args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            //this function tells open gl to draw
            // Clear the screen.
            clear(BLACK, gl);
            let transform = c.transform.trans(x,y);
            rectangle(WHITE, pixel,transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        //get the data from vid mem and draw it to the dispaly
    }
}