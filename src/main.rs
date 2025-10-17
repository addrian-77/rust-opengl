#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::single_match)]

use beryllium::{events::Event, init::InitFlags, video::GlSwapInterval, *};
use ogl33::*;
mod shaders;
use rust_opengl::*;
use shaders::fragment_shader;
use shaders::vertex_shader;

const VERT_SHADER: &str = vertex_shader::VERT_SHADER;
const FRAG_SHADER: &str = fragment_shader::FRAG_SHADER;

type Vertex = [f32; 3];

fn main() {
    let sdl = Sdl::init(InitFlags::EVERYTHING);

    let win = create_window(&sdl, "main window", 800, 600);
    win.set_swap_interval(GlSwapInterval::Vsync).unwrap();
    unsafe {
        load_gl_with(|f_name| win.get_proc_address(f_name.cast()));

        glClearColor(0.2, 0.3, 0.3, 1.0);

        let mut vao = 0;
        glGenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
        glBindVertexArray(vao);

        let mut vbo = 0;
        glGenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);

        glBindBuffer(GL_ARRAY_BUFFER, vbo);

        let vertices: [Vertex; 3] = [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];
        glBufferData(
            GL_ARRAY_BUFFER,
            size_of_val(&vertices) as isize,
            vertices.as_ptr().cast(),
            GL_STATIC_DRAW,
        );

        glVertexAttribPointer(
            0,
            3,
            GL_FLOAT,
            GL_FALSE,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );
        glEnableVertexAttribArray(0);

        let vertex_shader = glCreateShader(GL_VERTEX_SHADER);
        assert_ne!(vertex_shader, 0);

        glShaderSource(
            vertex_shader,
            1,
            &(VERT_SHADER.as_bytes().as_ptr().cast()),
            &(VERT_SHADER.len().try_into().unwrap()),
        );

        glCompileShader(vertex_shader);

        let mut success = 0;
        glGetShaderiv(vertex_shader, GL_COMPILE_STATUS, &mut success);
        print!("success {}", success);

        // logging
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            glGetShaderInfoLog(vertex_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Vertex compile Error: {}", String::from_utf8_lossy(&v));
        }

        let fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);
        assert_ne!(fragment_shader, 0);

        glShaderSource(
            fragment_shader,
            1,
            &(FRAG_SHADER.as_bytes().as_ptr().cast()),
            &(FRAG_SHADER.len().try_into().unwrap()),
        );

        glCompileShader(fragment_shader);

        let mut success = 0;
        glGetShaderiv(fragment_shader, GL_COMPILE_STATUS, &mut success);
        print!("\nfragment success: {}", success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            glGetShaderInfoLog(fragment_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
        }

        let shader_program = glCreateProgram();
        glAttachShader(shader_program, vertex_shader);
        glAttachShader(shader_program, fragment_shader);
        glLinkProgram(shader_program);
        let mut success = 0;
        glGetProgramiv(shader_program, GL_LINK_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            glGetProgramInfoLog(shader_program, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
        }

        glDeleteShader(vertex_shader);
        glDeleteShader(fragment_shader);

        glUseProgram(shader_program);
    }
    'main_loop: loop {
        // handle events this frame
        while let Some((event, _timestamp)) = sdl.poll_events() {
            match event {
                Event::Quit => break 'main_loop,
                _ => (),
            }
        }
        // now the events are clear.

        // here's where we could change the world state and draw.
        unsafe {
            glClear(GL_COLOR_BUFFER_BIT);
            glDrawArrays(GL_TRIANGLES, 0, 3);
        }
        win.swap_window();
    }
}
