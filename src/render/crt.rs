// CRT monitor visual effects
//
// Renders VGA content to an off-screen texture, then draws it
// with a CRT shader (scanlines, curvature, chromatic aberration).

use macroquad::prelude::*;

const VERTEX_SHADER: &str = r#"#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
varying vec2 uv;
uniform mat4 Model;
uniform mat4 Projection;
void main() {
    gl_Position = Projection * Model * vec4(position, 1.0);
    uv = texcoord;
}
"#;

pub struct CrtEffect {
    material: Material,
    render_target: RenderTarget,
}

impl CrtEffect {
    pub fn new(width: u32, height: u32) -> Result<Self, String> {
        let render_target = render_target(width, height);
        render_target.texture.set_filter(FilterMode::Nearest);

        let fragment_shader = std::fs::read_to_string("assets/shaders/crt.frag")
            .map_err(|e| format!("Failed to load CRT shader: {}", e))?;

        let material = load_material(
            ShaderSource::Glsl {
                vertex: VERTEX_SHADER,
                fragment: &fragment_shader,
            },
            MaterialParams {
                uniforms: vec![
                    UniformDesc::new("time", UniformType::Float1),
                    UniformDesc::new("resolution", UniformType::Float2),
                ],
                ..Default::default()
            },
        )
        .map_err(|e| format!("Failed to create CRT material: {}", e))?;

        Ok(Self { material, render_target })
    }

    /// Begin off-screen pass — draw VGA content after this call
    pub fn begin_pass(&self) {
        set_camera(&Camera2D {
            render_target: Some(self.render_target.clone()),
            ..Default::default()
        });
        clear_background(BLACK);
    }

    /// End off-screen pass and draw CRT effect to screen
    pub fn end_pass_and_draw(&self, x: f32, y: f32, w: f32, h: f32) {
        set_default_camera();

        self.material.set_uniform("time", macroquad::time::get_time() as f32);
        self.material.set_uniform("resolution", (w, h));

        gl_use_material(&self.material);
        draw_texture_ex(
            &self.render_target.texture,
            x, y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(w, h)),
                ..Default::default()
            },
        );
        gl_use_default_material();
    }
}
