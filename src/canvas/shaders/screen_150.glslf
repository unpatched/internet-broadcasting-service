#version 330 core

in vec2 v_Tex;

out vec4 Target0;
// out vec4 Ffmpeg0;

uniform sampler2D u_Source;

void main() {
    Target0  = texture(u_Source, v_Tex);
//    Ffmpeg0  = texture(u_Source, v_Tex);
}
