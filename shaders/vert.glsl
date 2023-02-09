#version 420

in vec4 pos;
out vec2 uv;

void main() {
  gl_Position = pos;
  uv = (pos.xy + 1) * 0.5;
}
