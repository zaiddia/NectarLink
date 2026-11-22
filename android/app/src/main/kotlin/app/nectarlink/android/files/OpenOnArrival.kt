// SPDX-License-Identifier: GPL-3.0-or-later
package app.nectarlink.android.files

import android.content.Context
import androidx.core.content.edit

/**
 * Whether a document a PC hands off (Send to phone and open) opens by itself
 * as soon as it arrives. Off by default: it's saved in Downloads either way,
 * and its notification opens it with a tap, so a PC can't open files on
 * this phone without you.
 */
object OpenOnArrival {
    private const val PREFS = "open_on_arrival"
    private const val KEY = "enabled"

    fun isOn(context: Context): Boolean =
        context.applicationContext.getSharedPreferences(PREFS, Context.MODE_PRIVATE).getBoolean(KEY, false)

    fun set(context: Context, on: Boolean) {
        context.applicationContext.getSharedPreferences(PREFS, Context.MODE_PRIVATE).edit { putBoolean(KEY, on) }
    }
}
