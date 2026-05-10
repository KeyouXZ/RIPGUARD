package com.skyo.ripguard.ui.deteksi

import androidx.compose.material3.AlertDialog
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import com.skyo.ripguard.ConfigManager
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.RequestBody.Companion.toRequestBody
import java.io.IOException

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
        .url("$${ConfigManager.BASE_URL}/report")
        .post(body)
        .build()

    client.newCall(request).enqueue(object : okhttp3.Callback {
        override fun onFailure(call: okhttp3.Call, e: IOException) {
            e.printStackTrace()
        }

        override fun onResponse(call: okhttp3.Call, response: okhttp3.Response) {
            println("Reported: ${response.body?.string()}")
        }
    })
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ShowErrorAlert(error: IOException, onDismiss: () -> Unit) {
    val message = error.message ?: "Tidak tersedia"

    AlertDialog(
        onDismissRequest = onDismiss,
        title = {
            Text("Error Terdeteksi")
        },
        text = {
            Text(message)
        },
        confirmButton = {
            TextButton(
                onClick = {
                    val fullError = buildString {
                        append(error.message)
                        append("\n\n")
                        append(error.stackTraceToString())
                    }

                    reportError(fullError)

                    onDismiss()
                }
            ) {
                Text(
                    "Laporkan"
                )
            }
        },
        dismissButton = {
            TextButton(
                onClick = {
                    onDismiss()
                }
            ) {
                Text("Abaikan")
            }
        }
    )
}