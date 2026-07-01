// Copyright (C) 2026 Keyou
// SPDX-License-Identifier: AGPL-3.0-or-later

package com.skyo.ripguard.ui.deteksi

import android.annotation.SuppressLint
import android.util.Log
import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Card
import androidx.compose.material3.LinearProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.asImageBitmap
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.Dialog
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import com.skyo.ripguard.ConfigManager
import com.skyo.ripguard.model.DetectionResult
import com.skyo.ripguard.model.DetectionState
import com.skyo.ripguard.viewmodel.DetectionViewModel
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.RequestBody.Companion.toRequestBody
import java.io.IOException

@SuppressLint("DefaultLocale")
@Composable
fun DetectionDialog(
    detectionViewModel: DetectionViewModel,
    data: DetectionResult?,
    error: IOException?,
    onDismiss: () -> Unit
) {
    val state by detectionViewModel.currentState.collectAsStateWithLifecycle()
    val canDismiss = state == DetectionState.Success ||
            state == DetectionState.Failed
    var isReporting by remember { mutableStateOf(false) }

    if (state != DetectionState.Idle && state != DetectionState.Capturing) {
        Dialog(
            onDismissRequest = {
                if (canDismiss) onDismiss()
            }
        ) {
            Card {
                Column(
                    modifier = Modifier
                        .padding(16.dp, 10.dp, 10.dp, 10.dp)
                        .fillMaxWidth(),
                ) {
                    when (state) {
                        DetectionState.Uploading,
                        DetectionState.Processing -> {
                            Column(
                                modifier = Modifier.fillMaxWidth(),
                                horizontalAlignment = Alignment.CenterHorizontally
                            ) {
                                Text(
                                    text = "Menganalisis Arus!",
                                    style = MaterialTheme.typography.titleMedium,
                                    fontWeight = FontWeight.Bold
                                )

                                Spacer(Modifier.height(12.dp))

                                LinearProgressIndicator(
                                    modifier = Modifier.fillMaxWidth(0.9f),
                                    color = MaterialTheme.colorScheme.secondary,
                                    trackColor = MaterialTheme.colorScheme.surfaceVariant,
                                )
                            }
                        }

                        DetectionState.Failed -> {
                            Text(
                                text = "Gagal Mendeteksi Rip Current",
                                style = MaterialTheme.typography.titleLarge,
                                fontWeight = FontWeight.Bold
                            )

                            Spacer(Modifier.height(8.dp))

                            Text("Terjadi kesalahan saat analisis gambar.")

                            Spacer(Modifier.height(12.dp))

                            TextButton(
                                onClick = {
                                    if (isReporting) return@TextButton

                                    isReporting = true

                                    val safeError = error ?: IOException("Unknown error")
                                    val message = safeError.message ?: "Tidak tersedia"
                                    val stackTrace = safeError.stackTraceToString()

                                    val fullError = buildString {
                                        append(message)
                                        append("\n\n")
                                        append(stackTrace)
                                    }

                                    reportError(fullError)

                                    isReporting = false
                                    onDismiss()
                                }
                            ) {
                                Text("Laporkan Error")
                            }
                        }

                        else -> {
                            if (data == null) return@Card

                            val base64 = data.result.image
                            val bitmap = remember(base64) {
                                base64ToBitmap(base64)
                            }

                            val bboxCount = data.result.detections.size
                            val detections = data.result.detections

                            val avgConfidence = if (detections.isNotEmpty()) {
                                detections.map { it.confidence }.average().toFloat() * 100
                            } else {
                                0f
                            }
                            val formatted = String.format("%.2f", avgConfidence)

                            Text(
                                text = if (bboxCount > 0) {
                                    "RIP Current Terdeteksi"
                                } else {
                                    "Rip Current Tidak Terdeteksi"
                                },
                                style = MaterialTheme.typography.titleLarge,
                                fontWeight = FontWeight.Bold,
                                overflow = TextOverflow.Ellipsis
                            )

                            Text(
                                text = if (bboxCount > 0) {
                                    "Tingkat keyakinan rata-rata: $formatted%"
                                } else {
                                    "Kamu aman! Tetap waspada!"
                                }
                            )

                            Spacer(Modifier.height(5.dp))

                            bitmap?.let {
                                Image(
                                    bitmap = bitmap.asImageBitmap(),
                                    contentDescription = null,
                                    alignment = Alignment.Center,
                                    modifier = Modifier
                                        .fillMaxWidth()
                                        .clip(RoundedCornerShape(16.dp)),
                                    contentScale = ContentScale.Crop
                                )
                            }
                        }
                    }
                }
            }
        }
    }
}

fun reportError(error: String) {
    val client = okhttp3.OkHttpClient()

    val json = """
        {
            "message": "$error",
            "platform": "android",
            "source": "ripguard-deteksi"
        }
    """.trimIndent()

    val body = json.toRequestBody("application/json".toMediaType())

    val request = okhttp3.Request.Builder()
        .url("${ConfigManager.BASE_URL}/report")
        .post(body)
        .build()

    client.newCall(request).enqueue(object : okhttp3.Callback {
        override fun onFailure(call: okhttp3.Call, e: IOException) {
            e.printStackTrace()
        }

        override fun onResponse(call: okhttp3.Call, response: okhttp3.Response) {
            if (!response.isSuccessful) {
                return
            }
            Log.i("REPORT", "Reported: ${response.body?.string()}")
        }
    })
}