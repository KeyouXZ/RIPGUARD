package com.skyo.ripguard.controller

import androidx.compose.material3.FabPosition
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue

class ChromeController {
    var topBar by mutableStateOf<(@Composable () -> Unit)?>(null)
    var fab by mutableStateOf<(@Composable () -> Unit)?>(null)
    var fabPosition by mutableStateOf<FabPosition?>(null)
    var bottomBar by mutableStateOf<(@Composable () -> Unit)?>(null)
}

@Composable
fun UseChrome(
    chrome: ChromeController,
    topBar: (@Composable () -> Unit)? = null,
    fab: (@Composable () -> Unit)? = null,
    fabPosition: FabPosition? = null,
    bottomBar: (@Composable () -> Unit)? = null
) {
    chrome.topBar = topBar
    chrome.fab = fab
    chrome.fabPosition = fabPosition
    chrome.bottomBar = bottomBar
}
