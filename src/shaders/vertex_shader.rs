pub const VERT_SHADER: &str = r#"#version 330 core
	layout (location = 0) in vec3 pos;
	layout (location = 1) in vec3 vertexColor;
	out vec3 fragmentColor;

	void main() {
		gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);

		fragmentColor = vertexColor;
	}
"#;
