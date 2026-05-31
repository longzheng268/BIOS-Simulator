#version 100
precision mediump float;

varying vec2 uv;
uniform sampler2D Texture;
uniform float time;
uniform vec2 resolution;

// Barrel distortion for CRT curvature
vec2 barrel_distort(vec2 p) {
    float distortion = 0.04;
    float r2 = dot(p, p);
    return p + p * r2 * distortion;
}

void main() {
    // Map UV to -1..1 range for barrel distortion
    vec2 centered = uv * 2.0 - 1.0;

    // Apply barrel distortion (CRT curvature)
    vec2 distorted = barrel_distort(centered);

    // Map back to 0..1 range
    vec2 sample_uv = distorted * 0.5 + 0.5;

    // Discard pixels outside the screen area (curved edges)
    if (sample_uv.x < 0.0 || sample_uv.x > 1.0 || sample_uv.y < 0.0 || sample_uv.y > 1.0) {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
        return;
    }

    // Chromatic aberration — slight RGB channel offset
    float aberration = 0.002;
    vec2 uv_r = sample_uv + vec2(aberration, 0.0);
    vec2 uv_b = sample_uv - vec2(aberration, 0.0);

    float r = texture2D(Texture, uv_r).r;
    float g = texture2D(Texture, sample_uv).g;
    float b = texture2D(Texture, uv_b).b;

    vec3 color = vec3(r, g, b);

    // Scanlines — horizontal dark lines
    float scanline = sin(sample_uv.y * resolution.y * 3.14159) * 0.08;
    color -= scanline;

    // Slight vignette (darken edges)
    float vignette = 1.0 - dot(centered * 0.5, centered * 0.5) * 0.3;
    color *= vignette;

    // Phosphor glow — slight brightness boost
    color *= 1.1;

    // Subtle flicker
    float flicker = 1.0 + sin(time * 60.0) * 0.005;
    color *= flicker;

    gl_FragColor = vec4(color, 1.0);
}
