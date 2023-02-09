#version 420

in vec2 uv;
out vec4 out_color;

void main() { out_color = some_float * pos_frag; }
uniform uvec2 neuron_count;

uniform vec2 window_size;

