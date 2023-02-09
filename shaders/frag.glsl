#version 420

in vec2 uv;
out vec4 out_color;

uniform int input_neuron_count;
uniform int output_neuron_count;

uniform sampler2D input_data;
uniform sampler2D weights;
uniform uvec2 neuron_count;

uniform vec2 window_size;

uint input_neuron_count = neuron_count[0];
uint output_neuron_count = neuron_count[1];

vec2 pixel_size = 2 / window_size;
float decode_float(vec4 rgba) {
	uint result = uint(rgba.a * 255.);
	result |= uint(rgba.b * 255.) << 8;
	result |= uint(rgba.g * 255.) << 16;
	result |= uint(rgba.r * 255.) << 24;
	return uintBitsToFloat(result);
}

float color_to_float(uvec2 index, sampler2D tex) {
	vec4 color = texture(tex, index * pixel_size);
	return decode_float(color);
}

uvec2 get_index() {
	vec2 index = uv / pixel_size;
	return uvec2(uint(index.x), uint(index.y));
}

void main() {
	out_color = vec4(color_to_float(get_index(), input_data));
}
