package com.skyo.ripguard.model

data class BoundingBox(
    val x1: Float,
    val y1: Float,
    val x2: Float,
    val y2: Float
)

data class Detection(
    val bbox: BoundingBox,
    val confidence: Float
)

data class Result(
    val detections: List<Detection>,
    val image: String,
)

data class DetectionResult(
    val success: Boolean,
    val size: Int,
    val result: Result
)