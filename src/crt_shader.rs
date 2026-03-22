use macroquad::prelude::*;

pub const CRT_VERTEX: &str = r#"#version 100
precision lowp float;

attribute vec3 position;
attribute vec2 texcoord;

varying vec2 uv;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    uv = texcoord;
    gl_Position = Projection * Model * vec4(position, 1.0);
}
"#;

pub const CRT_FRAGMENT: &str = r#"#version 100
precision lowp float;

varying vec2 uv;
uniform sampler2D Texture;

void main() {
    vec2 p = uv;

    // Escurecimento por linha de varredura.
    float scanline = sin(p.y * 240.0 * 3.14159) * 0.04;
    vec3 color = texture2D(Texture, p).rgb;
    color -= scanline;

    // Vinheta simples.
    float dist = distance(p, vec2(0.5));
    color *= smoothstep(0.75, 0.45, dist);

    gl_FragColor = vec4(color, 1.0);
}
"#;

pub fn load_crt_material() -> Material {
    // Compila o shader CRT em um material reutilizável.
    load_material(
        ShaderSource::Glsl {
            vertex: CRT_VERTEX,
            fragment: CRT_FRAGMENT,
        },
        MaterialParams {
            uniforms: vec![],
            ..Default::default()
        },
    )
    .unwrap()
}
