#version 410 core

layout(location = 0) in vec3 in_pos;
layout(location = 1) in vec3 in_norm;
layout(location = 2) in vec2 in_tc;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform mat4 view_normal;
uniform mat4 model_normal;

out vec3 pos_ws;
out vec3 n_ws;
out vec2 tc;

void main()
{
    gl_Position = projection * view * model * vec4(in_pos, 1.0);
    pos_ws = vec3(model * vec4(in_pos, 1.0));
    n_ws = vec3(model_normal * vec4(in_norm, 0.0));
	tc = in_tc;
}