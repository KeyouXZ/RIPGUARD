// Copyright (C) 2026 Keyou
// SPDX-License-Identifier: AGPL-3.0-or-later

package com.skyo.ripguard.viewmodel

import android.util.Log
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.google.gson.Gson
import com.google.gson.reflect.TypeToken
import com.skyo.ripguard.WSManager
import com.skyo.ripguard.model.DelWsEvent
import com.skyo.ripguard.model.DetectionRes
import com.skyo.ripguard.model.WsEvent
import com.skyo.ripguard.model.WsMessage
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import kotlin.collections.emptyList

class LocationViewModel : ViewModel() {
    private val gson = Gson()
    init {
        Log.d("WS", "LocationViewModel created!")
        viewModelScope.launch {
            WSManager.messages.collect { message ->
                handleMessage(message)
            }
        }
    }

    private fun handleMessage(message: String) {
        try {
            Log.d("WS", "WS DATA: $message")

            if (message.isEmpty()) return

            val data = gson.fromJson(message, WsMessage::class.java)

            when (data.event) {
                WsEvent.Init -> {
                    val type = object : TypeToken<List<DetectionRes>>() {}.type
                    val initData: List<DetectionRes> = gson.fromJson(data.data, type)

                    setLocations(initData)
                }

                WsEvent.New -> {
                    val newData = gson.fromJson(data.data, DetectionRes::class.java)

                    addLocation(newData)
                }

                WsEvent.Del -> {
                    val delData = gson.fromJson(data.data, DelWsEvent::class.java)

                    deleteLocationById(delData.id)
                }
            }
        } catch (e: Exception) {
            Log.e("WS", "Error handling message: ${e.message}", e)
        }
    }

    private val _locations = MutableStateFlow<List<DetectionRes>>(emptyList())
    val locations: StateFlow<List<DetectionRes>> = _locations.asStateFlow()

    fun setLocations(locations: List<DetectionRes>) {
        Log.d("VM", "setLocations ${locations.size}")
        _locations.value = locations
    }

    fun addLocation(location: DetectionRes) {
        _locations.value += location
    }

    fun deleteLocationById(id: Int) {
        _locations.value = _locations.value.filter { it.id != id }
    }

    fun clearLocations() {
        _locations.value = emptyList()
    }
}
