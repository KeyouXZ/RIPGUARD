package com.skyo.ripguard.ui.deteksi

import android.annotation.SuppressLint
import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Card
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.asImageBitmap
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.Dialog
import com.skyo.ripguard.model.DetectionResult

@SuppressLint("DefaultLocale")
@Composable
fun SuccessDialog(data: DetectionResult, onDismiss: () -> Unit) {
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

    Dialog(
        onDismiss
    ) {
        Card {
            Column(
                modifier = Modifier
                    .padding(16.dp, 10.dp, 10.dp,10.dp)
                    .fillMaxWidth(),
            ) {
                Text(
                    text = if (bboxCount > 0) { "RIP Current Terdeteksi"} else { "Rip Current Tidak Terdeteksi"},
                    style = MaterialTheme.typography.titleLarge,
                    fontWeight = FontWeight.Bold,
                    overflow = TextOverflow.Ellipsis
                )

                Text(
                    text = if (bboxCount > 0) { "Tingkat keyakinan rata-rata: $formatted%"} else { "Kamu aman! Tetap waspada!"}
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