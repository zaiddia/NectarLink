// SPDX-License-Identifier: GPL-3.0-or-later
// The Nova theme's backdrop: a holographic grid on deep space (or frost)
// that drifts slowly, with a scan line sweeping down the window, a cyan
// halo that rises from the logo and a violet one in the far corner.
// Samples no textures. Output is premultiplied.
//
// Compiled to nova.frag.qsb with Qt's qsb (see shaders.qrc).
#version 440

layout(location = 0) in vec2 qt_TexCoord0;
layout(location = 0) out vec4 fragColor;

layout(std140, binding = 0) uniform buf {
    mat4 qt_Matrix;
    float qt_Opacity;
    vec2 itemSize;
    vec2 origin;        // the cyan halo's centre, in pixels (the logo)
    float time;         // seconds
    float cell;         // grid cell size, in pixels
    float motion;       // 0: still (reduced motion), 1: drifting and scanning
    float strength;     // overall intensity
    float driftPeriod;  // seconds for the grid to drift one cell
    float scanPeriod;   // seconds per scan-line sweep
    vec4 lineColor;
    vec4 glowColor;
    vec4 accentColor;
};

float hash(vec2 p) {
    return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453);
}

// 1 on a grid line, 0 between, anti-aliased to about a pixel.
float gridLine(vec2 p, float size) {
    vec2 g = abs(fract(p / size - 0.5) - 0.5) * size;
    float d = min(g.x, g.y);
    return 1.0 - smoothstep(0.0, 1.1, d);
}

void main() {
    vec2 px = qt_TexCoord0 * itemSize;
    float far = length(itemSize);

    // The grid drifts diagonally, one cell per drift period.
    vec2 drift = vec2(1.0, 0.6) * cell * fract(time / driftPeriod) * motion;
    vec2 gp = px + drift;
    float minor = gridLine(gp, cell);
    float major = gridLine(gp, cell * 4.0);

    // A dot of light where major lines cross, twinkling cell by cell.
    vec2 cellId = floor(gp / (cell * 4.0));
    vec2 toNode = abs(fract(gp / (cell * 4.0) + 0.5) - 0.5) * cell * 4.0;
    float node = exp(-dot(toNode, toNode) / 6.0);
    float twinkle = 0.55 + 0.45 * sin(time * 1.7 + hash(cellId) * 6.2831853) * motion;

    // Calm in the middle of the content, stronger toward the edges.
    vec2 uv = qt_TexCoord0 - 0.5;
    float vignette = smoothstep(0.15, 0.85, length(uv * vec2(1.2, 1.0)));
    float lineA = (minor * 0.55 + major * 1.0) * (0.55 + 0.45 * vignette);

    // The scan line: a soft band sweeping down the window.
    float scanY = fract(time / scanPeriod) * (itemSize.y + 240.0) - 120.0;
    float scan = exp(-pow((px.y - scanY) / 46.0, 2.0)) * motion;
    float scanCore = exp(-pow((px.y - scanY) / 1.6, 2.0)) * motion;

    // The halos: cyan from the logo, violet from the far corner.
    float dCyan = distance(px, origin);
    float dViolet = distance(px, itemSize * vec2(0.92, 1.05));
    float breathe = 0.85 + 0.15 * sin(time * 0.6) * motion;
    float cyanA = exp(-dCyan / (far * 0.32)) * 0.55 * breathe;
    float violetA = exp(-dViolet / (far * 0.30)) * 0.45 * (1.7 - breathe);

    vec3 rgb = lineColor.rgb * lineColor.a * (lineA + scan * minor * 2.2 + scanCore * 0.35)
             + lineColor.rgb * lineColor.a * node * twinkle * 2.4
             + glowColor.rgb * glowColor.a * (cyanA + scan * 0.10)
             + accentColor.rgb * accentColor.a * violetA;
    float a = lineColor.a * (lineA + scan * minor * 2.2 + scanCore * 0.35 + node * twinkle * 2.4)
            + glowColor.a * (cyanA + scan * 0.10)
            + accentColor.a * violetA;

    float k = strength * qt_Opacity;
    fragColor = vec4(rgb * k, clamp(a, 0.0, 1.0) * k);
}
