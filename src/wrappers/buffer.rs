use std::os::raw::c_void;

use gl33::{GlFns, *};

use crate::{Triangle, Vertex, types::types::Color};

// this function creates and binds vao, vbo and colorbuffer
pub fn create_buffers(
    gl: &GlFns,
    vao: &mut u32,
    vbo: &mut u32,
    ibo: &mut u32,
    data: &mut Vec<Vertex>,
    data_indices: &mut Vec<u32>,
) {
    unsafe {
        gl.GenVertexArrays(1, vao);
        gl.GenBuffers(1, vbo);
        gl.GenBuffers(1, ibo);

        print!("data {data:?}, indices {data_indices:?}");
        gl.BindVertexArray(*vao);

        gl.BindBuffer(GL_ARRAY_BUFFER, *vbo);
        gl.BufferData(
            GL_ARRAY_BUFFER,
            size_of_val(&data.to_owned()) as isize,
            data.to_owned().as_ptr().cast(),
            GL_STATIC_DRAW,
        );

        // // index 1 contains color r g b a
        // gl.VertexAttribPointer(1, 4, GL_FLOAT, 0, 4 * size_of::<f32>() as i32, 0 as *const _);
        // gl.EnableVertexAttribArray(1); // also enable it

        gl.BindBuffer(GL_ELEMENT_ARRAY_BUFFER, *ibo);
        gl.BufferData(
            GL_ELEMENT_ARRAY_BUFFER,
            size_of_val(&data_indices.to_owned()) as isize,
            data_indices.to_owned().as_ptr().cast(),
            GL_STATIC_DRAW,
        );

        // index 0 contains coordinates x y z
        gl.VertexAttribPointer(
            0,
            3,
            GL_FLOAT,
            0,
            0,
            // 3 * size_of::<f32>() as i32,
            0 as *const _,
        );
        gl.EnableVertexAttribArray(0); // also enable it

        gl.BindVertexArray(0); // unbind vao
        gl.BindBuffer(GL_ARRAY_BUFFER, 0); // unbind vbo
        gl.BindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0); // unbind ebo
    }
}

// struct Vertex{

// }
// this function adds a triangle to the buffer

pub fn add_triangle(data: &mut Vec<Vertex>, data_indices: &mut Vec<u32>, triangle: Triangle) {
    data.extend_from_slice(&[[0.05, 0.05, 0.0]]);
    data.extend_from_slice(&[[0.05, -0.05, 0.0]]);
    data.extend_from_slice(&[[-0.05, -0.05, 0.0]]);
    data.extend_from_slice(&[[-0.05, 0.05, 0.0]]);
    let index = (data.len() / 3) as u32;
    data_indices.extend_from_slice(&[
        0, 3, 1, 
        1, 3, 2
    ]);
}
