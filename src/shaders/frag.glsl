#version 130
out mediump vec4 color;
in mediump vec2 texcoord;

uniform sampler2D tex;
uniform vec2 cursorPos;
uniform vec2 windowSize;
uniform float flShadow;
uniform float flRadius;
uniform float cameraScale;
