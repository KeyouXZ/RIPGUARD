package com.skyo.ripguard.ui.deteksi

import android.widget.Toast
import androidx.compose.foundation.layout.Box
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import com.skyo.ripguard.controller.ChromeController
import com.skyo.ripguard.controller.UseChrome
import com.skyo.ripguard.model.DetectionResult
import com.skyo.ripguard.utility.CameraPermissionRequester
import java.io.IOException

@Composable
fun DeteksiScreen(chromeController: ChromeController, topBar: @Composable () -> Unit) {
    UseChrome(chromeController, topBar)

    var showDialog by remember { mutableStateOf(false) }
    var showAlert by remember { mutableStateOf(false) }
    var data by remember { mutableStateOf<DetectionResult?>(null) }
    var error by remember { mutableStateOf<IOException?>(null) }

    Box(Modifier) {
        CameraPermissionRequester("Camera permission required to scan rip current!") {
            CameraLogic(
                { res ->
                    showDialog = true
                    data = res
                },
                { e ->
                    showAlert = true
                    error = e
                }
            )
        }

        data?.let {
            if (showDialog) {
                SuccessDialog(it) {
                    showDialog = false
                }
            }
        }

        error?.let {
            if (showAlert) {
                ShowErrorAlert(it) {
                    showAlert = false
                }
            }
        }
    }
}