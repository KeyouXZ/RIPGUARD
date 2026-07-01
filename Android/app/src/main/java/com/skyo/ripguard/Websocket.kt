// Copyright (C) 2026 Keyou
// SPDX-License-Identifier: AGPL-3.0-or-later

package com.skyo.ripguard

import android.annotation.SuppressLint
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.Service
import android.content.Context
import android.content.Intent
import android.os.Build
import android.os.Handler
import android.os.IBinder
import android.os.Looper
import android.util.Log
import androidx.core.app.NotificationCompat
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.SharedFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asSharedFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.emptyFlow
import kotlinx.coroutines.launch
import okhttp3.OkHttpClient
import okhttp3.Request
import okhttp3.Response
import okhttp3.WebSocket
import okhttp3.WebSocketListener
import java.util.concurrent.TimeUnit

enum class WSStatus { DISCONNECTED, RECONNECTING, CONNECTED }

class PersistentWebSocket(private val url: String) {
    private val client = OkHttpClient.Builder()
        .pingInterval(30, TimeUnit.SECONDS)
        .build()

    private var webSocket: WebSocket? = null
    private val handler = Handler(Looper.getMainLooper())
    private var retryDelay = 1000L

    private val _status = MutableStateFlow(WSStatus.DISCONNECTED)
    val status: StateFlow<WSStatus> = _status.asStateFlow()

    var onMessage: ((String) -> Unit)? = null

    fun connect() {
        _status.value = WSStatus.RECONNECTING

        // Close existing if any to prevent leaks
        webSocket?.cancel()

        val request = Request.Builder().url(url).build()
        webSocket = client.newWebSocket(request, object : WebSocketListener() {
            override fun onOpen(webSocket: WebSocket, response: Response) {
                _status.value = WSStatus.CONNECTED
                retryDelay = 1000L
                Log.d("WS", "Connected")
            }

            override fun onFailure(webSocket: WebSocket, t: Throwable, response: Response?) {
                _status.value = WSStatus.RECONNECTING
                Log.e("WS", "Error: ${t.message}")
                scheduleReconnect()
            }

            override fun onClosed(webSocket: WebSocket, code: Int, reason: String) {
                _status.value = WSStatus.DISCONNECTED
                Log.d("WS", "Closed: $reason")
                scheduleReconnect()
            }

            override fun onMessage(
                webSocket: WebSocket,
                text: String
            ) {
                Log.d("WS", text)
                onMessage?.invoke(text)
            }
        })
    }

    private fun scheduleReconnect() {
        handler.postDelayed({ connect() }, retryDelay)
        retryDelay = minOf(retryDelay * 2, 30000L)
    }

    fun send(text: String) {
        webSocket?.send(text)
    }

    fun close() {
        webSocket?.close(1000, "Service Destroyed")
        client.dispatcher.executorService.shutdown()
    }
}

object WSManager {
    private var ws: PersistentWebSocket? = null
    private val _messages = MutableSharedFlow<String>(extraBufferCapacity = 64)
    val messages: SharedFlow<String> = _messages.asSharedFlow()

    val status: StateFlow<WSStatus>
        get() = ws?.status ?: MutableStateFlow(WSStatus.DISCONNECTED).asStateFlow()

    fun init(context: Context) {
        if (ws == null) {
            val url = ConfigManager.BASE_URL.replace("http", "ws") + "/ws"
            ws = PersistentWebSocket(url)
            ws?.onMessage = { msg ->
                _messages.tryEmit(msg)
            }
            ws?.connect()
        }
    }

    fun send(msg: String) {
        ws?.send(msg)
    }

    @Deprecated("Use messages SharedFlow instead", ReplaceWith("messages"))
    fun setOnMessage(listener: (String) -> Unit) {
        ws?.onMessage = listener
    }

    fun clearOnMessage() {
        ws?.onMessage = { msg ->
            _messages.tryEmit(msg)
        }
    }

    fun close() {
        ws?.close()
        ws = null
    }
}

class WSService : Service() {
    companion object {
        const val CHANNEL_ID = "RipGuardWSChannel"
    }

    override fun onCreate() {
        super.onCreate()
        createNotificationChannel()

        WSManager.init(this)

        startForegroundService()
    }

    override fun onBind(intent: Intent?): IBinder? = null

    override fun onDestroy() {
        WSManager.close()
        super.onDestroy()
    }

    @SuppressLint("ForegroundServiceType")
    private fun startForegroundService() {
        val notification = NotificationCompat.Builder(this, CHANNEL_ID)
            .setContentTitle("RipGuard Active")
            .setContentText("Monitoring enabled")
            .setSmallIcon(R.drawable.ripguard)
            .build()
        startForeground(1, notification)
    }

    private fun createNotificationChannel() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val channel = NotificationChannel(
                CHANNEL_ID,
                "RipGuard WebSocket",
                NotificationManager.IMPORTANCE_LOW
            )
            val manager = getSystemService(NotificationManager::class.java)
            manager.createNotificationChannel(channel)
        }
    }
}