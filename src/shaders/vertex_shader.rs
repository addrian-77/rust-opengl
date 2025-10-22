pub const VERT_SHADER: &str = r#"#version 330 core
	layout (location = 0) in vec3 pos;
	uniform mat4 transform;

	void main() {
		gl_Position.xyz = pos;
		gl_Position.w = 1.0;

		gl_Position = transform * gl_Position;
	}
"#;
