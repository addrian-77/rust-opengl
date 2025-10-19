#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::single_match)]

use std::thread::sleep;
use std::time::Duration;

use beryllium::{events::Event, init::InitFlags, video::GlSwapInterval, *};
use gl33::{GlFns, *};

use rust_opengl::*;

mod shaders;

mod types;
use crate::types::types::{Color, Triangle, Vertex};

mod wrappers;
use wrappers::buffer::*;
use wrappers::shader::*;

// type Vertex = [f32; 3];
// type Triangle = [Vertex; 3];
// type Color = [f32; 4];

fn main() {
    // boilerplate
    // this block creates the gl instance, compiles shaders and creates the program
    let sdl = Sdl::init(InitFlags::EVERYTHING);
    // this function is located in lib.rs, it creates and returns the window
    let win = create_window(&sdl, "main window", 800, 600);
    // refresh rate? i guess
    win.set_swap_interval(GlSwapInterval::Vsync).unwrap();
    unsafe {
        let gl = GlFns::load_from(&|f_name| win.get_proc_address(f_name.cast())).unwrap();
        // clear color wrapper, located in lib.rs
        clear_color(&gl, 0.2, 0.0, 0.0, 1.0);
        // this function creates vao and vbo
        create_buffers(&gl);
        // this function returns the vertex and fragment shaders to be used in creating the program
        let (vertex_shader, fragment_shader) = compile_shaders(&gl);
        // this function creates the program using vertex and fragment shader
        create_program(&gl, vertex_shader, fragment_shader);
        // boilerplate end

        let mut r = 0.0;
        let mut g = 0.0;
        let mut b = 0.0;

        let mut vertices: Vec<Vertex> = Vec::new();
        let mut colors: Vec<Color> = Vec::new();
        let triangle: Triangle = [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];
        let color: Color = [0.5, 0.3, 0.0, 1.0];
        add_triangle(&mut vertices, &mut colors, triangle, color);
        update_buffer(&gl, &vertices, &colors);

        // this is where the main loop starts
        'main_loop: loop {
            // handle events this frame
            while let Some((event, _timestamp)) = sdl.poll_events() {
                match event {
                    Event::Quit => break 'main_loop,
                    _ => (),
                }
            }

            if r < 1.0 {
                r += 0.1;
            } else {
                r = 0.0;
            }
            if g < 0.8 {
                g += 0.2;
            } else {
                g = 0.0;
            }
            if b < 0.7 {
                b += 0.3;
            } else {
                b = 0.0;
            }

            clear_color(&gl, r, g, b, 1.0);
            gl.Clear(GL_COLOR_BUFFER_BIT);

            gl.DrawArrays(GL_TRIANGLES, 0, 3);

            win.swap_window();

            // sleep for 0.5 seconds
            sleep(Duration::new(0, 500000000));
        }
    }
}
