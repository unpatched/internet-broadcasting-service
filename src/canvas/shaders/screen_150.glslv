#version 330 core

in vec2 a_Pos;
in vec2 a_Tex;

out vec2 v_Tex;


void main() {
    gl_Position = vec4(a_Pos, 0x0, 1.0);
    v_Tex = a_Tex;
}
