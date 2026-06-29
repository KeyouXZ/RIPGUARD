package com.skyo.ripguard.model

import com.google.gson.JsonElement
import com.google.gson.annotations.SerializedName
import kotlinx.serialization.Serializable

data class BoundingBox(
    val x1: Float,
    val y1: Float,
    val x2: Float,
    val y2: Float
)

data class Detection(
    val bbox: BoundingBox,
    val confidence: Float,
)

data class Result(
    val detections: List<Detection>,
    val image: String,
)

@Serializable
enum class WsEvent {
    @SerializedName("init")
    Init,
    @SerializedName("new")
    New,
    @SerializedName("del")
    Del
}

data class DelWsEvent(
    val id: Int
)

@Serializable
data class DetectionRes(
    val id: Int,
    val latitude: Float,
    val longitude: Float,
    val detections: List<Detection>,
    @SerializedName("image_path")
    val imagePath: String? = null,
    @SerializedName("wind_speed")
    val windSpeed: Float,
    @SerializedName("created_at")
    val createdAt: String,
)

data class WsMessage(
    val event: WsEvent,
    val data: JsonElement
)

data class DetectionResult(
    val success: Boolean,
    val size: Int,
    val result: Result
)

enum class DetectionState {
    Idle,
    Capturing,
    Uploading,
    Processing,
    Success,
    Failed
}