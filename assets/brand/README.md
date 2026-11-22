# The Panal mark

Panal is based on [NectarLink](https://github.com/thtbee/nectarlink) by thtbee.
Its name and mark are its own, as NectarLink's trademark policy asks of
modified versions ([TRADEMARKS.md](../../TRADEMARKS.md)).

A honeycomb cell (panal, in Spanish), the link between the phone and the PC,
in electric cyan around a violet core, on a deep-space tile with a cyan glow:
the colors of the Nova theme.

| File | What it is |
|---|---|
| `icon.svg` | The full-color app icon: deep-space tile, cyan glow and rim |
| `icon-flat.svg` | The same without the glow and rim, for flat contexts |
| `icon-mono.svg` | The mark alone in white, for one-color uses |
| `icon-512.png` | The app icon at 512 × 512, rendered by `desktop/app/src/mark.rs` |

The apps draw the mark themselves from the same geometry:
`desktop/app/src/mark.rs` (the Windows icon, tray and notifications at every
size), `desktop/app/qml/Icons.qml` (`mark`) and
`android/app/src/main/res/drawable/ic_launcher_*.xml` and `ic_notification.xml`.

Colors (Nova palette, `docs/design/tokens.json`): tile `#070A12`, glow
`#0A3B4C`, cell and rim `#5CE1FF`, core `#B388FF`.
