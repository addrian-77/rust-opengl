pub const FRAG_SHADER: &str = r#"#version 330 core
	uniform vec4 color;
	out vec4 fragCol;

	void main() {
		fragCol = color;
	}
"#;
