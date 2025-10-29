#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::single_match)]

use std::thread::sleep;
use std::time::Duration;

use beryllium::events::{SDL_Keycode, SDLK_ESCAPE};
use beryllium::{events::Event, init::InitFlags, video::GlSwapInterval, *};
use gl33::{GlFns, *};

use rust_opengl::*;

mod shaders;

mod types;
use crate::types::types::{Color, GameObjects, Shape, Transform, Triangle, Vertex};

mod wrappers;
use wrappers::shader::*;

fn main() {
    // boilerplate
    // this block creates the gl instance, compiles shaders and creates the program
    let sdl = Sdl::init(InitFlags::EVERYTHING);
    // this function is located in lib.rs, it creates and returns the window
    let win = create_window(&sdl, "main window", 800, 600);
    // refresh rate? i guess
    win.set_swap_interval(GlSwapInterval::Vsync).unwrap();
    unsafe {
        // new glfns instance
        let gl = GlFns::load_from(&|f_name| win.get_proc_address(f_name.cast())).unwrap();
        // this function creates the program using vertex and fragment shader
        let program_id = create_program(&gl);
        // boilerplate end

        // initialize game objects
        let mut objects: GameObjects = GameObjects::new();

        // add a cube (idk)
        let mut square: Shape = Shape::new_cube(0.0, 0.0, 0.0, 0.3);
        square.create_buffers(&gl, &program_id);
        // add the cube to the game objects
        objects.add_shape(square);

        // setup gl settings before going in main loop
        clear_color(&gl, 0.2, 0.0, 0.0, 1.0);
        gl.PolygonMode(GL_FRONT_AND_BACK, GL_LINE);

        'main_loop: loop {
            // we can use this to check for changes and scale everything on update
            let (w, h) = win.get_window_size();

            // handle events this frame
            while let Some((event, _timestamp)) = sdl.poll_events() {
                match event {
                    // mouse buttons
                    Event::MouseButton {
                        win_id: _,
                        mouse_id: _,
                        button,
                        pressed,
                        clicks: _,
                        x,
                        y,
                    } => {
                        if pressed {
                            match button {
                                // left click
                                1 => {
                                    println!("\nAdding square at x: {x} y: {y}");
                                    let normalized_x: f32 = 2.0 * x as f32 / w as f32 - 1.0;
                                    let normalized_y: f32 = -2.0 * y as f32 / h as f32 + 1.0;
                                    let mut new_square =
                                        Shape::new_square(normalized_x, normalized_y, 0.1);
                                    new_square.create_buffers(&gl, &program_id);
                                    objects.add_shape(new_square);
                                }

                                // right click
                                3 => {
                                    // check if we clicked a shape
                                    let normalized_x: f32 = 2.0 * x as f32 / w as f32 - 1.0;
                                    let normalized_y: f32 = -2.0 * y as f32 / h as f32 + 1.0;
                                    for shape in &objects.shapes {
                                        if shape.contains(normalized_x, normalized_y) {}
                                    }
                                }
                                _ => {
                                    println!("unhandled")
                                }
                            }
                        }
                    }

                    // other keys
                    Event::Key {
                        win_id: _,
                        pressed: _,
                        repeat: _,
                        scancode: _,
                        keycode,
                        modifiers: _,
                    } => match keycode {
                        // ESCAPE
                        SDLK_ESCAPE => {
                            println!("pressed escape");
                            break 'main_loop;
                        }

                        // unhandled keys
                        _ => println!("other key?"),
                    },

                    // quit
                    Event::Quit => break 'main_loop,

                    // unknown event?
                    _ => (),
                }
            }

            gl.Clear(GL_COLOR_BUFFER_BIT);

            // draw each object, maybe we should check if the object is visible? --todo
            let _ = objects.shapes.iter().for_each(|shape| {
                gl.BindVertexArray(shape.vao);
                gl.DrawElements(
                    GL_TRIANGLES,
                    shape.indices.len() as i32,
                    GL_UNSIGNED_INT,
                    0 as *const _,
                );
            });

            win.swap_window();

            // sleep for 0.5 seconds
            sleep(Duration::new(0, 100000000));
        }
    }
}
