package com.skyo.ripguard.viewmodel

import android.graphics.Bitmap
import androidx.lifecycle.ViewModel
import com.skyo.ripguard.model.DetectionState
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow

class DetectionViewModel : ViewModel() {

    private val _currentState = MutableStateFlow(DetectionState.Idle)
    val currentState: StateFlow<DetectionState> = _currentState.asStateFlow()

    private var recentImageResult: Bitmap? = null

    fun setImage(bitmap: Bitmap) {
        recentImageResult = bitmap
    }

    fun startScan() {
        _currentState.value = DetectionState.Capturing
    }

    fun setUploading() {
        _currentState.value = DetectionState.Uploading
    }

    fun setProcessing() {
        _currentState.value = DetectionState.Processing
    }

    fun setSuccess() {
        _currentState.value = DetectionState.Success
    }

    fun setFailed() {
        _currentState.value = DetectionState.Failed
    }

    fun reset() {
        _currentState.value = DetectionState.Idle
        recentImageResult = null
    }
}