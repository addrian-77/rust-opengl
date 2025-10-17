pub const FRAG_SHADER: &str = r#"#version 330 core
	out vec4 final_color;

	void main() {
		final_color = vec4(0.0, 0.5, 0.0, 1.0);
	}
"#;
