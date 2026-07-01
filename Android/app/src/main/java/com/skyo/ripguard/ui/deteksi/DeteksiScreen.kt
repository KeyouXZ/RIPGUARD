// Copyright (C) 2026 KeyouXZ
// SPDX-License-Identifier: AGPL-3.0-or-later

package com.skyo.ripguard.ui.deteksi

import androidx.compose.foundation.layout.Box
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import com.skyo.ripguard.controller.ChromeController
import com.skyo.ripguard.controller.UseChrome
import com.skyo.ripguard.model.DetectionResult
import com.skyo.ripguard.utility.CameraPermissionRequester
import com.skyo.ripguard.viewmodel.DetectionViewModel
import java.io.IOException

@Composable
fun DeteksiScreen(
    chromeController: ChromeController,
    detectionViewModel: DetectionViewModel,
    topBar: @Composable () -> Unit
) {
    UseChrome(chromeController, topBar)

    var data by remember { mutableStateOf<DetectionResult?>(null) }
    var error by remember { mutableStateOf<IOException?>(null) }

    Box(Modifier) {
        CameraPermissionRequester("Camera permission required to scan rip current!") {
            CameraLogic(
                detectionViewModel,
                { res ->
                    data = res
                },
                { e ->
                    error = e
                }
            )
        }

        DetectionDialog(detectionViewModel, data, error) {
            detectionViewModel.reset()
        }
    }
}
