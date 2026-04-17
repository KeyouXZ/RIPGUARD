package com.skyo.ripguard.ui.deteksi

import androidx.compose.foundation.layout.Box
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import com.skyo.ripguard.controller.ChromeController
import com.skyo.ripguard.controller.UseChrome
import com.skyo.ripguard.utility.CameraPermissionRequester

@Composable
fun DeteksiScreen(chromeController: ChromeController, topBar: @Composable () -> Unit) {
    UseChrome(chromeController, topBar)

    Box(Modifier) {
        CameraPermissionRequester("Camera permission required to scan rip current!") {
            CameraLogic()
        }
    }
}