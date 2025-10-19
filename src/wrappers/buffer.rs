use gl33::{GlFns, *};

use crate::{types::types::Color, Triangle, Vertex};

// this function creates and binds vao, vbo and colorbuffer
pub fn create_buffers(gl: &GlFns) {
    unsafe {
        let mut vao = 0;
        gl.GenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
        gl.BindVertexArray(vao);

        let mut vbo = 0;
        gl.GenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);
        gl.BindBuffer(GL_ARRAY_BUFFER, vbo);

        let mut colorbuffer = 0;
        gl.GenBuffers(1, &mut colorbuffer);
        assert_ne!(colorbuffer, 0);
        gl.BindBuffer(GL_ARRAY_BUFFER, colorbuffer);
    }
}

pub fn update_buffer(gl: &GlFns, vertices: &Vec<Vertex>, colors: &Vec<Color>) {
    unsafe {
        gl.EnableVertexAttribArray(0);
        gl.VertexAttribPointer(
            0,
            3,
            GL_FLOAT,
            0,
            size_of::<Vertex>() as i32,
            0 as *const _,
        );
        gl.BufferData(
            GL_ARRAY_BUFFER,
            size_of_val(&vertices.to_owned()) as isize,
            vertices.to_owned().as_ptr().cast(),
            GL_STATIC_DRAW,
        );
        gl.EnableVertexAttribArray(1);
        gl.VertexAttribPointer(
            1,
            4,
            GL_FLOAT,
            0,
            0,
            0 as *const _,
        );
        gl.BufferData(
            GL_ARRAY_BUFFER,
            size_of_val(&colors.to_owned()) as isize,
            colors.to_owned().as_ptr().cast(),
            GL_STATIC_DRAW,
        );
    }
}

// struct Vertex{

// }
// this function adds a triangle to the buffer

pub fn add_triangle(vertices: &mut Vec<Vertex>, colors: &mut Vec<Color>, triangle: Triangle, color: Color) {
    colors.extend_from_slice(&[color, color, color]);
    vertices.extend_from_slice(&triangle);
}
