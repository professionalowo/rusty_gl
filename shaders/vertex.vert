#version 330 core

layout(location = 0) in vec3 aPos;
layout(location = 1) in vec3 a_color;

out vec3 v_color;

void main()
{
    gl_Position = vec4(aPos, 1.0);
    v_color = a_color;
}