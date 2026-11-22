// SPDX-License-Identifier: GPL-3.0-or-later
package app.nectarlink.android.ui.theme

import android.graphics.RuntimeShader
import android.os.Build
import android.os.SystemClock
import androidx.annotation.RequiresApi
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.compositionLocalOf
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableFloatStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.drawWithCache
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.Path
import androidx.compose.ui.graphics.ShaderBrush
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.platform.LocalDensity
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.compose.LocalLifecycleOwner
import androidx.lifecycle.repeatOnLifecycle
import kotlinx.coroutines.delay

/** The Nova theme is active. */
val LocalNovaTheme = compositionLocalOf { false }

/**
 * Nova's holographic grid behind the app, the same grid as the PC app
 * (desktop/app/shaders/nova.frag): it drifts slowly, a scan line sweeps down
 * the screen, a cyan halo rises from the top and a violet one from the far
 * corner.
 *
 * Light on the battery, like Hive's comb: on Android 13 and newer one GPU
 * shader draws it, time advances about 30 times a second and only while the
 * app is in front, and only the backdrop redraws. With [animate] false it
 * holds still. Older Android draws a still grid from a cached path.
 */
@Composable
fun NovaGrid(
    dark: Boolean,
    animate: Boolean,
    modifier: Modifier = Modifier,
    cellDp: Float = Tokens.GRID_CELL_DP,
) {
    val palette = if (dark) Tokens.novaVoid else Tokens.novaFrost
    val accent = if (dark) Tokens.novaVoidAccent else Tokens.novaFrostAccent
    val line = palette.primary.copy(alpha = if (dark) Tokens.GRID_LINE_ALPHA_DARK else Tokens.GRID_LINE_ALPHA_LIGHT)
    val glowAlpha = if (dark) Tokens.GRID_GLOW_ALPHA_DARK else Tokens.GRID_GLOW_ALPHA_LIGHT
    val glow = palette.primary.copy(alpha = glowAlpha)
    val violet = accent.copy(alpha = glowAlpha * 0.8f)
    val cellPx = with(LocalDensity.current) { cellDp * density }

    // Seconds, carried across pauses so it picks up where it stopped.
    var time by remember { mutableFloatStateOf(0f) }
    val lifecycle = LocalLifecycleOwner.current.lifecycle
    LaunchedEffect(animate, lifecycle) {
        if (!animate) return@LaunchedEffect
        lifecycle.repeatOnLifecycle(Lifecycle.State.RESUMED) {
            val base = time
            val since = SystemClock.uptimeMillis()
            while (true) {
                delay(Tokens.GRID_FRAME_MS)
                time = base + (SystemClock.uptimeMillis() - since) / 1000f
            }
        }
    }
    val motion = if (animate) 1f else 0f

    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
        ShaderGrid(modifier, { time }, motion, cellPx, line, glow, violet)
    } else {
        PathGrid(modifier, cellPx, line, glow, violet)
    }
}

@RequiresApi(Build.VERSION_CODES.TIRAMISU)
@Composable
private fun ShaderGrid(
    modifier: Modifier,
    time: () -> Float,
    motion: Float,
    cell: Float,
    line: Color,
    glow: Color,
    accent: Color,
) {
    val shader = remember { RuntimeShader(GRID_AGSL) }
    val brush = remember(shader) { ShaderBrush(shader) }
    Canvas(modifier.fillMaxSize()) {
        shader.setFloatUniform("size", size.width, size.height)
        shader.setFloatUniform("origin", size.width / 2f, size.height * 0.08f)
        shader.setFloatUniform("time", time())
        shader.setFloatUniform("cell", cell)
        shader.setFloatUniform("motion", motion)
        shader.setFloatUniform("driftPeriod", Tokens.GRID_DRIFT_SECONDS)
        shader.setFloatUniform("scanPeriod", Tokens.GRID_SCAN_SECONDS)
        shader.setFloatUniform("lineColor", line.red, line.green, line.blue, line.alpha)
        shader.setFloatUniform("glowColor", glow.red, glow.green, glow.blue, glow.alpha)
        shader.setFloatUniform("accentColor", accent.red, accent.green, accent.blue, accent.alpha)
        drawRect(brush)
    }
}

/** Android 12 and older: a still grid as one cached path, with the two halos. */
@Composable
private fun PathGrid(modifier: Modifier, cell: Float, line: Color, glow: Color, accent: Color) {
    androidx.compose.foundation.layout.Box(
        modifier.fillMaxSize().drawWithCache {
            val minor = Path()
            val major = Path()
            var x = 0f
            var i = 0
            while (x <= size.width) {
                (if (i % 4 == 0) major else minor).apply { moveTo(x, 0f); lineTo(x, size.height) }
                x += cell; i++
            }
            var y = 0f
            i = 0
            while (y <= size.height) {
                (if (i % 4 == 0) major else minor).apply { moveTo(0f, y); lineTo(size.width, y) }
                y += cell; i++
            }
            val thin = Stroke(width = 1f * density)
            val far = kotlin.math.hypot(size.width, size.height)
            val top = Offset(size.width / 2f, size.height * 0.08f)
            val corner = Offset(size.width * 0.92f, size.height * 1.05f)
            onDrawBehind {
                drawPath(minor, line.copy(alpha = line.alpha * 0.45f), style = thin)
                drawPath(major, line.copy(alpha = line.alpha * 0.9f), style = thin)
                drawCircle(
                    brush = Brush.radialGradient(listOf(glow.copy(alpha = glow.alpha * 0.5f), Color.Transparent), top, far * 0.45f),
                    radius = far * 0.45f,
                    center = top,
                )
                drawCircle(
                    brush = Brush.radialGradient(listOf(accent.copy(alpha = accent.alpha * 0.45f), Color.Transparent), corner, far * 0.42f),
                    radius = far * 0.42f,
                    center = corner,
                )
            }
        },
    )
}

/** The grid's shader, the same as the PC app's (desktop/app/shaders/nova.frag). */
private const val GRID_AGSL = """
uniform float2 size;
uniform float2 origin;
uniform float time;
uniform float cell;
uniform float motion;
uniform float driftPeriod;
uniform float scanPeriod;
uniform float4 lineColor;
uniform float4 glowColor;
uniform float4 accentColor;

float hash(float2 p) {
    return fract(sin(dot(p, float2(127.1, 311.7))) * 43758.5453);
}

float gridLine(float2 p, float s) {
    float2 g = abs(fract(p / s - 0.5) - 0.5) * s;
    float d = min(g.x, g.y);
    return 1.0 - smoothstep(0.0, 1.1, d);
}

half4 main(float2 px) {
    float far = length(size);
    float2 drift = float2(1.0, 0.6) * cell * fract(time / driftPeriod) * motion;
    float2 gp = px + drift;
    float minor = gridLine(gp, cell);
    float major = gridLine(gp, cell * 4.0);

    float2 cellId = floor(gp / (cell * 4.0));
    float2 toNode = abs(fract(gp / (cell * 4.0) + 0.5) - 0.5) * cell * 4.0;
    float node = exp(-dot(toNode, toNode) / 6.0);
    float twinkle = 0.55 + 0.45 * sin(time * 1.7 + hash(cellId) * 6.2831853) * motion;

    float2 uv = px / size - 0.5;
    float vignette = smoothstep(0.15, 0.85, length(uv * float2(1.0, 1.2)));
    float lineA = (minor * 0.55 + major * 1.0) * (0.55 + 0.45 * vignette);

    float scanY = fract(time / scanPeriod) * (size.y + 240.0) - 120.0;
    float scan = exp(-pow((px.y - scanY) / 46.0, 2.0)) * motion;
    float scanCore = exp(-pow((px.y - scanY) / 1.6, 2.0)) * motion;

    float dCyan = distance(px, origin);
    float dViolet = distance(px, size * float2(0.92, 1.05));
    float breathe = 0.85 + 0.15 * sin(time * 0.6) * motion;
    float cyanA = exp(-dCyan / (far * 0.32)) * 0.55 * breathe;
    float violetA = exp(-dViolet / (far * 0.30)) * 0.45 * (1.7 - breathe);

    float lit = lineA + scan * minor * 2.2 + scanCore * 0.35 + node * twinkle * 2.4;
    float3 rgb = lineColor.rgb * lineColor.a * lit
               + glowColor.rgb * glowColor.a * (cyanA + scan * 0.10)
               + accentColor.rgb * accentColor.a * violetA;
    float a = lineColor.a * lit + glowColor.a * (cyanA + scan * 0.10) + accentColor.a * violetA;
    return half4(half3(rgb), half(clamp(a, 0.0, 1.0)));
}
"""
