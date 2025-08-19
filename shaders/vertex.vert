#version 330 core

layout(location = 0) in vec3 in_pos;
layout(location = 1) in vec3 in_col;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform sampler2D col_tex;

out vec3 v_color;

void main()
{
    gl_Position = projection * view * model * vec4(in_pos, 1.0);
    v_color = texture(col_tex, vec2(0,0)).rgb;
}