// NOTE: Shader automatically converted from Godot Engine 4.6.2.stable.mono's StandardMaterial3D.
shader_type spatial;
render_mode blend_mix, depth_draw_opaque, cull_back, diffuse_toon, specular_toon;

uniform vec4 albedo : source_color;
uniform sampler2D texture_albedo : source_color, filter_linear_mipmap, repeat_enable;

uniform float roughness = 0.02;
uniform float specular = 0.03;
uniform float metallic = 0.03;

uniform float shadow_steps : hint_range(1.0, 8.0) = 3.0;
uniform float light_wrap : hint_range(0.0, 1.0) = 0.1;

void fragment() {
	vec4 albedo_tex = texture(texture_albedo, UV);
	ALBEDO = albedo.rgb * albedo_tex.rgb;	
	METALLIC = metallic;
	SPECULAR = specular;
	ROUGHNESS = roughness;
}
void light() {
    float NdotL = dot(NORMAL, LIGHT);

    // wrap light
    NdotL = (NdotL + light_wrap) / (1.0 + light_wrap);

    // toon step
    float toon = floor(NdotL * shadow_steps) / shadow_steps;

    toon = max(toon, 0.05);

    DIFFUSE_LIGHT += toon * LIGHT_COLOR * ATTENUATION;
}