#version 330 core

layout(location = 0) in vec3 aPos;
layout(location = 1) in vec3 a_color;

uniform mat3 model;
uniform mat3 view;
uniform mat3 projection;

out vec3 v_color;

void main()
{
    gl_Position = vec4(projection * view * model * aPos, 1.0);
    v_color = a_color;
}