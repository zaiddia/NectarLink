// SPDX-License-Identifier: GPL-3.0-or-later
import QtQuick

// The Nova theme's holographic grid (shaders/nova.frag), behind the whole
// app: a grid that drifts slowly, a scan line sweeping down the window, a
// cyan halo rising from the logo and a violet one in the far corner.
//
// Light on resources, like Hive's comb: one fragment shader, no textures,
// and a clock that follows Theme's (it ticks only while the window is in
// front and motion is allowed). `animate` false draws a still grid with no
// drift, scan or twinkle (reduced motion, battery saver, the user's setting).
ShaderEffect {
    id: nova

    property bool animate: true
    property real time: 0
    property real cell: 32
    property real strength: 1
    property point origin: Qt.point(width / 6, height / 8)
    property real driftSeconds: 18.0
    property real scanSeconds: 7.0
    property color lineColor
    property color glowColor
    property color accentColor

    readonly property size itemSize: Qt.size(width, height)
    readonly property real motion: animate ? 1 : 0
    readonly property real driftPeriod: driftSeconds
    readonly property real scanPeriod: scanSeconds

    blending: true
    fragmentShader: "qrc:/shaders/nova.frag.qsb"
}
