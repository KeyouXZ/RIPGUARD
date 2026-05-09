package com.skyo.ripguard.ui.deteksi

import android.graphics.Bitmap
import android.graphics.BitmapFactory
import android.media.Image
import android.os.Build
import androidx.annotation.RequiresApi
import com.skyo.ripguard.BASE_URL
import com.skyo.ripguard.model.DetectionResult
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.RequestBody.Companion.toRequestBody
import java.io.IOException
import kotlin.io.encoding.Base64

fun Image.toJpegBytes(rotationDegrees: Int): ByteArray {
    val yPlane = planes[0]
    val uPlane = planes[1]
    val vPlane = planes[2]

    val yBuffer = yPlane.buffer
    val uBuffer = uPlane.buffer
    val vBuffer = vPlane.buffer

    val yRowStride = yPlane.rowStride
    val uvRowStride = uPlane.rowStride
    val uvPixelStride = uPlane.pixelStride

    // Manually build a clean NV21 byte array respecting strides
    val nv21 = ByteArray(width * height * 3 / 2)

    // Copy Y plane row by row
    for (row in 0 until height) {
        yBuffer.position(row * yRowStride)
        yBuffer.get(nv21, row * width, width)
    }

    // Copy VU interleaved (NV21) row by row
    val uvHeight = height / 2
    val uvWidth = width / 2
    for (row in 0 until uvHeight) {
        for (col in 0 until uvWidth) {
            val vPos = row * uvRowStride + col * uvPixelStride
            val uPos = row * uvRowStride + col * uvPixelStride
            val nv21Pos = width * height + row * width + col * 2

            vBuffer.position(vPos)
            nv21[nv21Pos] = vBuffer.get()

            uBuffer.position(uPos)
            nv21[nv21Pos + 1] = uBuffer.get()
        }
    }

    val yuvImage = android.graphics.YuvImage(
        nv21, android.graphics.ImageFormat.NV21, width, height, null
    )

    val out = java.io.ByteArrayOutputStream()
    yuvImage.compressToJpeg(android.graphics.Rect(0, 0, width, height), 95, out)

    // Apply rotation if needed
    if (rotationDegrees == 0) return out.toByteArray()

    val bitmap = BitmapFactory.decodeByteArray(out.toByteArray(), 0, out.size())
    val matrix = android.graphics.Matrix().apply { postRotate(rotationDegrees.toFloat()) }
    val rotated = Bitmap.createBitmap(bitmap, 0, 0, bitmap.width, bitmap.height, matrix, true)

    val rotatedOut = java.io.ByteArrayOutputStream()
    rotated.compress(Bitmap.CompressFormat.JPEG, 95, rotatedOut)

    // Clean up
    bitmap.recycle()
    rotated.recycle()

    return rotatedOut.toByteArray()
}

fun base64ToBitmap(base64: String): Bitmap? {
    return try {
        val decodedBytes = android.util.Base64.decode(base64, android.util.Base64.DEFAULT)
        BitmapFactory.decodeByteArray(decodedBytes, 0, decodedBytes.size)
    } catch (e: Exception) {
        null
    }
}

fun scanImage(image: Image, rotationDegrees: Int, onResult: (DetectionResult) -> Unit, onError: (IOException) -> Unit) {
    val jpegBytes = image.toJpegBytes(rotationDegrees)

    val client = okhttp3.OkHttpClient()

    val requestBody = jpegBytes.toRequestBody("image/jpeg".toMediaType(), 0, jpegBytes.size)

    val multipartBody = okhttp3.MultipartBody.Builder()
        .setType(okhttp3.MultipartBody.FORM)
        .addFormDataPart(
            "file",
            "scan.jpg",
            requestBody
        )
        .build()

    val request = okhttp3.Request.Builder()
        .url("$BASE_URL/detect")
        .post(multipartBody)
        .build()

    client.newCall(request).enqueue(object : okhttp3.Callback {
        override fun onFailure(call: okhttp3.Call, e: IOException) {
            e.printStackTrace()

            onError(e)
        }

        override fun onResponse(call: okhttp3.Call, response: okhttp3.Response) {
            val body = response.body?.string() ?: return

            val gson = com.google.gson.Gson()
            val result = gson.fromJson(body, DetectionResult::class.java)

            onResult(result)
        }
    })
}
