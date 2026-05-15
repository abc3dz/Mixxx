shader_type spatial;

uniform vec4 color1 : source_color = vec4(0.25, 0.25, 0.25, 1.0);
uniform vec4 color2 : source_color = vec4(0.35, 0.35, 0.35, 1.0);

uniform float tile_size = 4.0;

void fragment() {
    vec2 grid = floor(UV * tile_size);

    float checker = mod(grid.x + grid.y, 2.0);

    vec3 color = mix(color1.rgb, color2.rgb, checker);

    ALBEDO = color;
}