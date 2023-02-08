#version 420
in vec2 pos;
out vec4 pos_frag;

void main() {
  pos_frag = vec4(pos, vec2(1.));
  gl_Position = pos_frag;
}
