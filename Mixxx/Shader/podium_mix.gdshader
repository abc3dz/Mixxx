shader_type spatial;
render_mode blend_add, depth_draw_opaque, cull_back, unshaded;

uniform vec4 base_color : source_color = vec4(0.0, 0.5, 1.0, 0.4);
uniform sampler2D noise_texture : repeat_enable;
uniform float noise_scale : hint_range(0.1, 10.0) = 2.0;
uniform float noise_speed : hint_range(0.0, 5.0) = 0.5;
uniform float noise_strength : hint_range(0.0, 1.0) = 0.3;

// vertex options
uniform bool enable_vertex_wave = true;
uniform float vertex_amplitude : hint_range(0.0, 0.2) = 0.02;
uniform float vertex_speed : hint_range(0.0, 5.0) = 1.5;
uniform float vertex_frequency : hint_range(0.0, 10.0) = 3.0;

void vertex() {
    if (enable_vertex_wave) {
        float wave = sin(VERTEX.x * vertex_frequency + TIME * vertex_speed)
                   * cos(VERTEX.z * vertex_frequency + TIME * vertex_speed);
        VERTEX.y += wave * vertex_amplitude;
    }
}

void fragment() {
    // เลื่อน noise ตามเวลา
    vec2 uv_scroll = UV * noise_scale + vec2(TIME * noise_speed, TIME * noise_speed * 0.7);
    float noise = texture(noise_texture, uv_scroll).r;

    // noise layer ที่สองเลื่อนทิศตรงข้ามเพื่อให้ดูซับซ้อนขึ้น
    vec2 uv_scroll2 = UV * noise_scale * 1.3 - vec2(TIME * noise_speed * 0.5, TIME * noise_speed);
    float noise2 = texture(noise_texture, uv_scroll2).r;

    float combined_noise = mix(noise, noise2, 0.5);

    // ผสม noise เข้ากับสีหลัก
    vec3 color = base_color.rgb + combined_noise * noise_strength * vec3(0.0, 0.5, 1.0);
    float alpha = base_color.a + combined_noise * 0.2;

    // rim light เพื่อให้ขอบสว่างขึ้น
    float rim = 1.0 - dot(NORMAL, VIEW);
    rim = pow(rim, 2.0);
    color += rim * vec3(0.0, 0.6, 1.0) * 0.5;
    alpha += rim * 0.3;

    ALBEDO = color;
    ALPHA = clamp(alpha, 0.0, 1.0);
}