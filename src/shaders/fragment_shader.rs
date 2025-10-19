pub const FRAG_SHADER: &str = r#"#version 330 core
	in fragmentColor;
	out vec3 color;

	void main() {
		color = fragmentColor;
	}
"#;
