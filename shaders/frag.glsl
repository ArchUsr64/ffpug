#version 420

uniform float some_float;

in vec4 pos_frag;
out vec4 out_color;

void main() { out_color = some_float * pos_frag; }
