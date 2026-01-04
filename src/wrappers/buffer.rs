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
        // generate the buffers
        gl.GenVertexArrays(1, vao);
        gl.GenBuffers(1, vbo);
        gl.GenBuffers(1, ibo);

        // check if the buffers failed to initialize, if any of them are 0, panic
        // or we should try again? to make the program more reliable?
        assert_ne!(*vao, 0);
        assert_ne!(*vbo, 0);
        assert_ne!(*ibo, 0);

        // set up vao
        gl.BindVertexArray(*vao);
        gl.BindBuffer(GL_ARRAY_BUFFER, *vbo);
        // load data into vao, the vertexes
        gl.BufferData(
            GL_ARRAY_BUFFER,
            (data.len() * size_of::<Vertex>()) as isize,
            data.as_ptr().cast(),
            GL_STATIC_DRAW,
        );

        // set up ibo, this contains indices
        gl.BindBuffer(GL_ELEMENT_ARRAY_BUFFER, *ibo);
        // load indices into ibo
        gl.BufferData(
            GL_ELEMENT_ARRAY_BUFFER,
            (data_indices.len() * size_of::<u32>()) as isize,
            data_indices.to_owned().as_ptr().cast(),
            GL_STATIC_DRAW,
        );

        // index 0 contains coordinates x y z
        // TODO: use index 1 for textures or other things, or load textures using a function for each object?
        gl.VertexAttribPointer(
            0,
            3,
            GL_FLOAT,
            0,
            3 * size_of::<f32>() as i32,
            0 as *const _,
        );
        gl.EnableVertexAttribArray(0); // enable attributes at index 0

        // unbind, idk why
        gl.BindVertexArray(0); // unbind vao
        gl.BindBuffer(GL_ARRAY_BUFFER, 0); // unbind vbo

        // we should not unbind ebo, it does not work without it
        // gl.BindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0); // unbind ebo
    }
}

pub fn delete_buffers(gl: &GlFns, vao: &mut u32, vbo: &mut u32, ibo: &mut u32) {
    // this function is used when we fail to initialize a shape, and we must try again
    // delete the buffers and generate them again
    unsafe {
        gl.DeleteBuffers(1, vao);
        gl.DeleteBuffers(1, vbo);
        gl.DeleteBuffers(1, ibo);
    }
}
