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
use crate::types::types::{Color, Shape, GameObjects, Transform, Triangle, Vertex};

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

        // this function creates the program using vertex and fragment shader
        let program_id = create_program(&gl);

        // this function creates vao and vbo
        // let mut vao: u32 = 0;
        // let mut vbo: u32 = 0;
        // let mut ibo: u32 = 0;

        // let mut data: Vec<Vertex> = Vec::new();
        // let mut data_indices: Vec<u32> = Vec::new();
        // let triangle: Triangle = [[0.5, -0.5, 0.0], [-0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];

        // add_cube(&mut data, &mut data_indices, (0.1, 0.1));

        // let triangle: Shape = Shape::new_triangle(0.2, 0.2);
        let mut square: Shape = Shape::new_cube(0.0, 0.0, 0.0, 0.3);
        square.create_buffers(&gl, &program_id);
        let mut shapes: GameObjects = GameObjects::new();
        shapes.add_shape(square);
        // shapes.add_shape(triangle);

        // println!("debug shapes vectors: {:?}", shapes.shapes.);
        // println!("debug indices {:?}", shapes.indices);
        // println!("debug last index {}", shapes.last_index);
        // panic!();

        // for shape in &shapes.shapes {
        //     data.extend_from_slice(&shape.vertex);
        // }
        // data_indices.extend_from_slice(&shapes.indices);

        println!("bef");
        // create_buffers(
        //     &gl,
        //     &mut vao,
        //     &mut vbo,
        //     &mut ibo,
        //     &mut data,
        //     &mut data_indices,
        // );
        // println!("buff");
        // println!("indices len {:?}", data_indices.len());
        // boilerplate end

        let mut r = 0.1;
        let mut g = 0.3;
        let mut b = 0.0;
        println!("reached");

        // this is where the main loop starts
        // gl.Enable(GL_DEPTH_TEST);
        gl.PolygonMode(GL_FRONT_AND_BACK, GL_LINE);
        // panic!();
        'main_loop: loop {
            // handle events this frame
            let (w, h) = win.get_window_size();

            while let Some((event, _timestamp)) = sdl.poll_events() {
                // print!("received event {:?}", event);

                match event {
                    // Event::Key { win_id, pressed, repeat, scancode, keycode, modifiers }
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
                            // println!("pressed? {pressed}");
                            println!("pressed button {button} at x: {x} y: {y}");
                            let normalized_x: f32 = 2.0 * x as f32 / w as f32 - 1.0;
                            let normalized_y: f32 = -2.0 * y as f32 / h as f32 + 1.0;

                            let mut new_triangle = Shape::new_square(normalized_x, normalized_y, 0.1);
                            new_triangle.create_buffers(&gl, &program_id);
                            shapes.add_shape(new_triangle);
                            // data.clear();
                            // data_indices.clear();
                            // for shape in &shapes.shapes {
                            //     data.extend_from_slice(&shape.vertex);
                            // }
                            // data_indices.extend_from_slice(&shapes.indices);
                            // println!("current data {data:?}");
                            // println!("current indices {data_indices:?}");
                            // println!("new coords x {normalized_x}, new coord y {normalized_y}");
                            // create_buffers(
                            //     &gl,
                            //     &mut vao,
                            //     &mut vbo,
                            //     &mut ibo,
                            //     &mut data,
                            //     &mut data_indices,
                            // );
                        }
                    }
                    // Event::MouseMotion {
                    //     win_id: _,
                    //     mouse_id: _,
                    //     button_state: _,
                    //     x_win,
                    //     y_win,
                    //     x_delta: _,
                    //     y_delta: _,
                    // } => {
                    //     println!("mouse pos: {x_win}, {y_win}")
                    // }
                    Event::Key {
                        win_id: _,
                        pressed: _,
                        repeat: _,
                        scancode: _,
                        keycode,
                        modifiers: _,
                    } => match keycode {
                        SDLK_ESCAPE => {
                            println!("pressed escape");
                            break 'main_loop;
                        }
                        _ => println!("other key?"),
                    },
                    Event::Quit => break 'main_loop,
                    _ => (),
                }
            }

            // gl.BindVertexArray(vao);
            // bind all the arrays?
            let _ = shapes.shapes.iter().for_each(|shape| gl.BindVertexArray(shape.vao));    // this might not be needed, we might just be missing the transform and color
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

            // clear_color(&gl, r, g, b, 1.0);
            gl.Clear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

            // println!("data {data:?}");
            // println!("indices {data_indices:?}");
            // println!("indices len {:?}", data_indices.len());

            // let transform_loc = gl.GetUniformLocation(program_id, transform.as_ptr().cast());
            // let transform_mat: Transform = [
            //     // [
            //     [1.0, 0.0, 0.0, 0.0],
            //     [0.0, 1.0, 0.0, 0.0],
            //     [0.0, 0.0, 1.0, 0.0],
            //     [0.0, 0.0, 0.0, 1.0],
            //     // ],
            //     // [
            //     //     [r, 0.0, 0.0, 0.0],
            //     //     [0.0, b, 0.0, 0.0],
            //     //     [0.0, 0.0, g, 0.0],
            //     //     [0.0, 0.0, 0.0, 1.0],
            //     // ],
            // ];
            // gl.UniformMatrix4fv(transform_loc, 1, 1, transform_mat.as_ptr().cast());

            // let color_loc = gl.GetUniformLocation(program_id, color.as_ptr().cast());
            // let color_vec: Color = [b, r, g, 1.0];
            // // println!(
            // //     "transform loc {:?} color loc {:?}",
            // //     transform_loc, color_loc
            // // );
            // // println!(
            // //     "transform ptr {:?}, color_ptr {:?}",
            // //     transform.as_ptr(),
            // //     color.as_ptr()
            // // );
            // gl.Uniform4fv(color_loc, 1, color_vec.as_ptr().cast());

            // println!("transform {transform}");

            // let mut size: *mut i32 = std::ptr::null_mut();
            // gl.GetBufferParameteriv(GL_ELEMENT_ARRAY_BUFFER, GL_BUFFER_SIZE, size);
            gl.DrawElements(
                GL_TRIANGLES,
                shapes.total_indices,
                GL_UNSIGNED_INT,
                0 as *const _,
            );

            win.swap_window();
            // println!("current window size is {w}, {h}");

            // sleep for 0.5 seconds
            sleep(Duration::new(0, 100000000));
        }
    }
}
