#version 410 core

in vec3 v_color;
out vec4 out_col;

void main()
{
    out_col = vec4(v_color, 1.0); // Red color
}