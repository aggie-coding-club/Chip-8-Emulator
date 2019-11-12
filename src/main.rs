mod state;
mod constants;
mod cpu;
mod display;
use std::io;
use std::io::prelude::*;
use std::fs::File;
//includes for display
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use display::Display;

fn main() {

//display stuff
    //set openGL version
    let opengl = OpenGL::V2_1;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "CHIP8 EMULATOR",
            [800, 600]
        )
        .graphics_api(opengl)
        .exit_on_esc(false)
        .build()
        .unwrap();

    
    let mut display = Display {
        gl: GlGraphics::new(opengl)
    };
    //update display
    let mut events = Events::new(EventSettings::new());
    //the main display loop
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            display.render(&r);
        }

        if let Some(u) = e.update_args() {
            display.update(&u);
        }
    }
//end display stuff

    let mut file = File::open("roms/test_opcode.ch8");

    let mut buf: Vec<u8> = Vec::new();

    let a = match file {
        Ok(mut f) => f.read_to_end(&mut buf),
        Err(e) => Err(e)
    };





    for b in buf {
        print!("{:x} ", b);
    }
}
