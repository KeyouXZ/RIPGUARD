// Copyright (C) 2026 Keyou
// SPDX-License-Identifier: AGPL-3.0-or-later

package com.skyo.ripguard.ui.deteksi

import android.graphics.Bitmap
import android.graphics.BitmapFactory
import android.util.Log
import com.skyo.ripguard.ConfigManager
import com.skyo.ripguard.model.DetectionResult
import com.skyo.ripguard.viewmodel.DetectionViewModel
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.OkHttpClient
import okhttp3.RequestBody.Companion.toRequestBody
import java.io.ByteArrayOutputStream
import java.io.IOException
import java.util.concurrent.TimeUnit


fun Bitmap.toJpegBytes(quality: Int = 95): ByteArray {
    val stream = ByteArrayOutputStream()
    this.compress(Bitmap.CompressFormat.JPEG, quality, stream)
    return stream.toByteArray()
}

fun base64ToBitmap(base64: String): Bitmap? {
    return try {
        val decodedBytes = android.util.Base64.decode(base64, android.util.Base64.DEFAULT)
        BitmapFactory.decodeByteArray(decodedBytes, 0, decodedBytes.size)
    } catch (e: Exception) {
        null
    }
}

fun scanImage(detectionViewModel: DetectionViewModel, image: Bitmap, zoom: Float, onResult: (DetectionResult) -> Unit, onError: (IOException) -> Unit) {
    detectionViewModel.setProcessing()
    // Convert into JPEG Bytes
    val jpegBytes = image.toJpegBytes()

    val client = OkHttpClient.Builder()
        .connectTimeout(5, TimeUnit.SECONDS)
        .readTimeout(10, TimeUnit.SECONDS)
        .writeTimeout(10, TimeUnit.SECONDS)
        .callTimeout(15, TimeUnit.SECONDS)
        .build()

    val requestBody = jpegBytes.toRequestBody("image/jpeg".toMediaType(), 0, jpegBytes.size)

    val multipartBody = okhttp3.MultipartBody.Builder()
        .setType(okhttp3.MultipartBody.FORM)
        .addFormDataPart(
            "file",
            "scan.jpg",
            requestBody
        )
        .build()

    detectionViewModel.setUploading()
    val request = okhttp3.Request.Builder()
        .url("${ConfigManager.BASE_URL}/detect")
        .post(multipartBody)
        .build()

    client.newCall(request).enqueue(object : okhttp3.Callback {
        override fun onFailure(call: okhttp3.Call, e: IOException) {
            Log.e("SCANNER", "Fail to call the API")
            e.printStackTrace()

            CoroutineScope(Dispatchers.Main).launch {
                detectionViewModel.setFailed()

                onError(e)
            }
        }

        override fun onResponse(call: okhttp3.Call, response: okhttp3.Response) {
            Log.i("SCANNER", "Got response from the API")
            if (!response.isSuccessful) {
                CoroutineScope(Dispatchers.Main).launch {
                    detectionViewModel.setFailed()
                    onError(IOException("HTTP ${response.code}"))
                }
                return
            }

            val body = response.body?.string() ?: return

            val gson = com.google.gson.Gson()
            val result = gson.fromJson(body, DetectionResult::class.java)


            CoroutineScope(Dispatchers.Main).launch {
                detectionViewModel.setSuccess()

                onResult(result)
            }
        }
    })
}
