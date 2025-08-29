#version 330 core

in vec3 n_ws;
in vec3 pos_ws;
in vec2 tc;

uniform float pointlight_intensity;
uniform vec3 pointlight_color;
uniform vec3 pointlight_pos;
uniform vec3 camera_pos;
uniform vec3 dirlight_dir;
uniform vec3 dirlight_color;
uniform float dirlight_intensity;
//uniform vec4 k_diff;
//uniform vec4 k_spec;

uniform sampler2D specular;
uniform sampler2D diffuse;
uniform sampler2D alphamap;
uniform int has_alphamap;

out vec4 out_col;

vec3 phong(vec3 n, vec3 l, vec3 v, vec3 I, float ns) {

	vec3 diff = texture(diffuse,tc).rgb * max(0, dot(n, l));

	vec3 r = 2*n*dot(n,l)-l;
	vec3 spec = texture(specular,tc).rgb * pow(max(0, dot(r, v)), ns);

	return (diff + spec) * I;
}

void main()
{
		
	float alpha = texture(alphamap, tc).r;
	
	if (has_alphamap==1 && alpha<0.3) {
		discard;
	}

    vec3 v = normalize(camera_pos - pos_ws);
    vec3 n = normalize(n_ws);
    vec3 to_light = pointlight_pos - pos_ws;
	float dist = length(to_light);
	to_light = normalize(to_light);

    float attenuation = 1.0 / (dist/100);
	vec3 pointlight_illum = phong(n, to_light, v, pointlight_color*pointlight_intensity, 140) * attenuation;
	vec3 dirlight_illum = phong(n, -dirlight_dir, v, dirlight_color*dirlight_intensity, 4);

	out_col = vec4(pointlight_illum + dirlight_illum, 1);
}