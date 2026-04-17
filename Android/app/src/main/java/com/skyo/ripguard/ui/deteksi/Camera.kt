package com.skyo.ripguard.ui.deteksi

import android.annotation.SuppressLint
import androidx.annotation.OptIn
import androidx.camera.core.AspectRatio
import androidx.camera.core.CameraSelector
import androidx.camera.core.ExperimentalGetImage
import androidx.camera.core.ImageAnalysis
import androidx.camera.core.Preview
import androidx.camera.extensions.ExtensionMode
import androidx.camera.extensions.ExtensionsManager
import androidx.camera.lifecycle.ProcessCameraProvider
import androidx.camera.view.PreviewView
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.gestures.detectTransformGestures
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableFloatStateOf
import androidx.compose.runtime.mutableLongStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.CornerRadius
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.viewinterop.AndroidView
import androidx.core.content.ContextCompat
import androidx.lifecycle.compose.LocalLifecycleOwner

@SuppressLint("LocalContextResourcesRead")
@OptIn(ExperimentalGetImage::class)
@Composable
fun CameraLogic() {
    val lifecycleOwner = LocalLifecycleOwner.current

    var cameraRef by remember { mutableStateOf<androidx.camera.core.Camera?>(null) }
    var scale by remember { mutableFloatStateOf(1f) }

    var lineOffset by remember { mutableFloatStateOf(0f) }
    val context = LocalContext.current
    val primaryColor = MaterialTheme.colorScheme.primary
    var isScanning by remember { mutableStateOf(true) }

    var lastAnalysisTime by remember { mutableLongStateOf(0L) }
    val scanDelayMs = 500L

    Box(
        Modifier
            .fillMaxSize()
            .pointerInput(Unit) {
                detectTransformGestures { _, _, zoomRatio, _ ->
                    cameraRef?.let { cam ->
                        val zoomState = cam.cameraInfo.zoomState.value
                        val minZoom = zoomState?.minZoomRatio ?: 1f
                        val maxZoom = zoomState?.maxZoomRatio ?: 5f

                        val newScale = (scale * zoomRatio).coerceIn(minZoom, maxZoom)

                        scale = newScale
                        cam.cameraControl.setZoomRatio(newScale)
                    }
                }
            }
    ) {
        AndroidView(
            factory = { ctx ->
                val previewView = PreviewView(ctx)
                val cameraProviderFuture = ProcessCameraProvider.getInstance(context)

                cameraProviderFuture.addListener({
                    val cameraProvider = cameraProviderFuture.get()

                    val preview = Preview.Builder().setTargetAspectRatio(AspectRatio.RATIO_16_9).build().also {
                        it.surfaceProvider = previewView.surfaceProvider
                    }

                    val extensionsManagerFuture = ExtensionsManager.getInstanceAsync(context, cameraProvider)
                    val extensionsManager = extensionsManagerFuture.get()

                    val baseSelector = CameraSelector.DEFAULT_BACK_CAMERA
                    val hdrCameraSelector = if (extensionsManager.isExtensionAvailable(baseSelector, ExtensionMode.HDR)) {
                        extensionsManager.getExtensionEnabledCameraSelector(baseSelector, ExtensionMode.HDR)
                    } else {
                        baseSelector
                    }

                    val imageAnalyzer = ImageAnalysis.Builder()
                        .setBackpressureStrategy(ImageAnalysis.STRATEGY_KEEP_ONLY_LATEST)
                        .setTargetResolution(android.util.Size(1280, 720))
                        .build()
                        .also { analysis ->
                            analysis.setAnalyzer(ContextCompat.getMainExecutor(context)) { imageProxy ->
                                val currentTime = System.currentTimeMillis()

                                if (!isScanning || currentTime - lastAnalysisTime < scanDelayMs) {
                                    imageProxy.close()
                                    return@setAnalyzer
                                }

                                val mediaImage = imageProxy.image
                                if (mediaImage != null) {
                                    // DO LOGIC HERE
                                } else {
                                    imageProxy.close()
                                }
                            }
                        }

                    try {
                        cameraProvider.unbindAll()
                        cameraRef = cameraProvider.bindToLifecycle(
                            lifecycleOwner,
                            hdrCameraSelector,
                            preview,
                            imageAnalyzer
                        )
                    } catch (exc: Exception) {
                        exc.printStackTrace()
                    }
                }, ContextCompat.getMainExecutor(context))

                previewView
            },
            modifier = Modifier.fillMaxSize(),
        )

        Canvas(modifier = Modifier.fillMaxSize()) {
            val boxSize = size.width * 0.7f
            val left = (size.width - boxSize) / 2
            val top = (size.height - boxSize) / 2
            val right = left + boxSize

            drawRect(color = androidx.compose.ui.graphics.Color(0x88000000))

            drawRoundRect(
                color = androidx.compose.ui.graphics.Color.Transparent,
                topLeft = androidx.compose.ui.geometry.Offset(left, top),
                size = androidx.compose.ui.geometry.Size(boxSize, boxSize),
                blendMode = androidx.compose.ui.graphics.BlendMode.Clear,
                cornerRadius = CornerRadius(16f, 16f)
            )

            drawRoundRect(
                color = primaryColor.copy(0.8f),
                topLeft = androidx.compose.ui.geometry.Offset(left, top),
                size = androidx.compose.ui.geometry.Size(boxSize, boxSize),
                style = androidx.compose.ui.graphics.drawscope.Stroke(width = 4f),
                cornerRadius = CornerRadius(16f, 16f)
            )

            drawLine(
                color = primaryColor,
                start = androidx.compose.ui.geometry.Offset(left, top + lineOffset),
                end = androidx.compose.ui.geometry.Offset(right, top + lineOffset),
                strokeWidth = 6f
            )
        }

        LaunchedEffect(Unit) {
            while (true) {
                lineOffset += 5f
                if (lineOffset > (0.6f * context.resources.displayMetrics.widthPixels))
                    lineOffset = 0f
                kotlinx.coroutines.delay(16L)
            }
        }
    }
}