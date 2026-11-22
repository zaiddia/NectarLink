// SPDX-License-Identifier: GPL-3.0-or-later
import QtQuick
import app.nectarlink

// Text in one of the design's type roles: displayLarge, displaySmall,
// headline, title, body, bodySmall, label, caption, code.
Text {
    id: txt
    property string role: "body"
    // Secondary text uses the variant color.
    property bool muted: false

    readonly property var spec: Theme.typography[role] || Theme.typography.body
    readonly property bool isDisplay: role === "displayLarge" || role === "displaySmall" || role === "headline"
    // Graphite labels are mono, uppercase and tracked (docs/design/README.md);
    // Nova's labels are upper case and tracked too, in Figtree (its captions
    // stay mixed case, since they run longer).
    // Nova: false keeps a long label (a settings section) in mixed case.
    property bool novaCaps: true
    readonly property bool graphiteLabel: (Theme.graphite && (role === "label" || role === "caption"))
                                          || (Theme.nova && novaCaps && role === "label")

    // Pixel size; set this (not font.pixelSize) to resize, so the role's
    // tracking scales with it.
    property real size: Math.round(graphiteLabel ? Theme.labelStyle.size : spec.size)
    // Set this (not font.weight) to change the weight: Figtree is a variable
    // font, and its weight axis is set to match, so in-between weights such
    // as 650 render exactly instead of snapping to a named style.
    property int weight: graphiteLabel ? Theme.labelStyle.weight : (isDisplay && Theme.graphite ? 400 : spec.weight)

    readonly property string family: role === "code" || (graphiteLabel && !Theme.nova)
        ? Theme.fontMono : (isDisplay ? Theme.fontDisplay : Theme.fontUi)

    color: muted ? Theme.surfaceContentVariant : Theme.surfaceContent
    font.family: family
    font.pixelSize: Math.max(1, Math.round(size))
    font.weight: weight
    font.variableAxes: family === Theme.fontUi ? { "wght": weight } : {}
    font.letterSpacing: (graphiteLabel ? Theme.labelStyle.tracking : spec.tracking) * size
    font.capitalization: graphiteLabel ? Font.AllUppercase : Font.MixedCase
    // Token line heights are CSS-style (a multiple of the font size), not
    // a multiple of the font's own line spacing.
    lineHeight: Math.round(Math.max(1, Math.round(size)) * spec.lineHeight)
    lineHeightMode: Text.FixedHeight
    verticalAlignment: Text.AlignVCenter
    textFormat: Text.PlainText
    elide: Text.ElideRight
}
