use gl33::GlFns;

use crate::wrappers::buffer::create_buffers;

pub type Vertex = [f32; 3]; // x,y,z
pub type Triangle = [Vertex; 3];
pub type Color = [f32; 4];
pub type Transform = [[f32; 4]; 4];

// pub trait Shape???

// // shape types?
// // triangle, square, circle even??
// // have 3 separate buffers with each of them
// // have a combined buffer?
// //
// big TODO here

enum ShapeTypes {
    Triangle,
    Square,
    Cube,
}

pub struct Shape {
    pub vertex: Vec<Vertex>,
    pub indices: Vec<u32>,
    typ: ShapeTypes,
    pub vao: u32,
    vbo: u32,
    ibo: u32,
}

impl Shape {
    pub fn create_buffers(&mut self, gl: &GlFns, program_id: &u32) {
        create_buffers(gl, &mut self.vao, &mut self.vbo, &mut self.ibo, &mut self.vertex, &mut self.indices);
        let transform: String = "transform".to_string();
        let color: String = "color".to_string();
            unsafe {
        
        let transform_loc = gl.GetUniformLocation(*program_id, transform.as_ptr().cast());
            let transform_mat: Transform = [
                // [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
                // ],
                // [
                //     [r, 0.0, 0.0, 0.0],
                //     [0.0, b, 0.0, 0.0],
                //     [0.0, 0.0, g, 0.0],
                //     [0.0, 0.0, 0.0, 1.0],
                // ],
            ];
                gl.UniformMatrix4fv(transform_loc, 1, 1, transform_mat.as_ptr().cast());

                let color_loc = gl.GetUniformLocation(*program_id, color.as_ptr().cast());
                println!("color loc? {color_loc}");
                // panic!();
                let color_vec: Color = [1.0, 1.0, 1.0, 1.0];
                // println!(
                //     "transform loc {:?} color loc {:?}",
                //     transform_loc, color_loc
                // );
                // println!(
                //     "transform ptr {:?}, color_ptr {:?}",
                //     transform.as_ptr(),
                //     color.as_ptr()
                // );
                gl.Uniform4fv(color_loc, 1, color_vec.as_ptr().cast());
            }
    }
    pub fn new_cube(x: f32, y: f32, z: f32, size: f32) -> Self {
        // return the object
        Self {
            vertex: vec![
                [x, y, z + size],
                [x + size, y, z + size],
                [x + size, y + size, z + size],
                [x, y + size, z + size],
                [x, y, z],
                [x + size, y, z],
                [x + size, y + size, z],
                [x, y + size, z],
            ],
            indices: get_indices_slice(0, &ShapeTypes::Cube),
            typ: ShapeTypes::Cube,
            vao: 0,
            vbo: 0,
            ibo: 0,
        }
    }

    pub fn new_square(x: f32, y: f32, size: f32) -> Self {
        Self {
            vertex: vec![
                [x, y, 0.0],
                [x, y - size, 0.0],
                [x + size, y - size, 0.0],
                [x + size, y, 0.0],
            ],
            indices: get_indices_slice(0, &ShapeTypes::Square),
            typ: ShapeTypes::Square,
            vao: 0,
            vbo: 0,
            ibo: 0,
        }
    }

    pub fn new_triangle(x: f32, y: f32) -> Self {
        Self {
            vertex: vec![[x, -y, 0.0], [-x, -y, 0.0], [x, y, 0.0]],
            indices: get_indices_slice(0, &ShapeTypes::Triangle),
            typ: ShapeTypes::Triangle,
            vao: 0,
            vbo: 0,
            ibo: 0,
        }
    }
}

pub struct GameObjects {
    pub shapes: Vec<Shape>,
    pub total_indices: i32,
}

impl GameObjects {
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
            total_indices: 0,
        }
    }

    pub fn add_shape(&mut self, shape: Shape) {
        self.total_indices += match shape.typ {
            ShapeTypes::Cube => {
                36
            }
            ShapeTypes::Square => {
                6
            }
            ShapeTypes::Triangle => {
                3
            }
        };
        self.shapes.push(shape);
    }

    pub fn remove_shape(&mut self, index: usize) {
        self.shapes.remove(index);
        // shift everything... nightmare
        // or maybe not?
    }
}

fn get_indices_slice(index: u32, typ: &ShapeTypes) -> Vec<u32> {
    match typ {
        ShapeTypes::Triangle => {
            vec![index, index + 1, index + 2]
        }
        ShapeTypes::Square => {
            vec![
                index,
                index + 1,
                index + 3,
                //
                index + 1,
                index + 2,
                index + 3,
            ]
        }
        ShapeTypes::Cube => {
            vec![
                index,
                index + 1,
                index + 2,
                //
                index + 2,
                index + 3,
                index,
                //
                index + 1,
                index + 5,
                index + 6,
                //
                index + 6,
                index + 2,
                index + 1,
                //
                index + 7,
                index + 6,
                index + 5,
                //
                index + 5,
                index + 4,
                index + 7,
                //
                index + 4,
                index,
                index + 3,
                //
                index + 3,
                index + 7,
                index + 4,
                //
                index + 4,
                index + 5,
                index + 1,
                //
                index + 1,
                index,
                index + 4,
                //
                index + 3,
                index + 2,
                index + 6,
                //
                index + 6,
                index + 7,
                index + 3,
            ]
        }
        _ => {
            panic!("unknown shape??")
        }
    }
}
