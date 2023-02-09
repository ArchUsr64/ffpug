#version 420

in vec2 uv;
out vec4 out_color;

uniform int input_neuron_count;
uniform int output_neuron_count;

uniform sampler2D input_data;
uniform sampler2D weights;

float decode_float(vec4 rgba) {
	uint result = uint(rgba.a * 255.);
	result |= uint(rgba.b * 255.) << 8;
	result |= uint(rgba.g * 255.) << 16;
	result |= uint(rgba.r * 255.) << 24;
	return uintBitsToFloat(result);
}

float color_to_float(uvec2 index, sampler2D tex) {
	vec2 texel_coord = vec2(index) + 0.5;
	texel_coord.x /= input_neuron_count;
	texel_coord.y /= output_neuron_count;
	vec4 color = texture(tex, texel_coord);
	return decode_float(color);
}

uvec2 get_index() {
	return uvec2(
		uint(uv.x * input_neuron_count),
		uint(uv.y * output_neuron_count));
}

void main() {
	uint my_index = get_index().y;
	float result = 0;
	for (int j = 0; j < input_neuron_count; j++) {
		result += color_to_float(uvec2(j, my_index), input_data)
			* color_to_float(uvec2(j, my_index), weights);
	}
	out_color = vec4(result / 10);
}
