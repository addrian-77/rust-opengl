use gl33::{GlFns, *};

use crate::shaders::{fragment_shader::FRAG_SHADER, vertex_shader::VERT_SHADER};

pub fn create_program(gl: &GlFns) -> u32 {
    unsafe {
        let vertex_shader = gl.CreateShader(GL_VERTEX_SHADER);
        gl.ShaderSource(
            vertex_shader,
            1,
            &(VERT_SHADER.as_bytes().as_ptr().cast()),
            &(VERT_SHADER.len().try_into().unwrap()),
        );
        gl.CompileShader(vertex_shader);

        let mut success = 0;
        gl.GetShaderiv(vertex_shader, GL_COMPILE_STATUS, &mut success);
        println!("success {}", success);

        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl.GetShaderInfoLog(vertex_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Vertex compile Error: {}", String::from_utf8_lossy(&v));
        }

        let fragment_shader = gl.CreateShader(GL_FRAGMENT_SHADER);
        gl.ShaderSource(
            fragment_shader,
            1,
            &(FRAG_SHADER.as_bytes().as_ptr().cast()),
            &(FRAG_SHADER.len().try_into().unwrap()),
        );
        gl.CompileShader(fragment_shader);

        let mut success = 0;
        gl.GetShaderiv(fragment_shader, GL_COMPILE_STATUS, &mut success);
        println!("fragment success: {}", success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl.GetShaderInfoLog(fragment_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
        }

        let shader_program = gl.CreateProgram();
        gl.AttachShader(shader_program, vertex_shader);
        gl.AttachShader(shader_program, fragment_shader);
        gl.LinkProgram(shader_program);

        gl.DeleteShader(vertex_shader);
        gl.DeleteShader(fragment_shader);

        // gl.UseProgram(shader_program);
        println!("created program with id {shader_program}");
        gl.UseProgram(shader_program);
        shader_program
    }
}
