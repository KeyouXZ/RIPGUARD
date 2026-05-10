package com.skyo.ripguard

import android.content.Context
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import okhttp3.OkHttpClient
import okhttp3.Request
import org.json.JSONObject
import androidx.core.content.edit

object ConfigManager {
    var BASE_URL: String = "http://ripguard.skyo.my.id:43433"

    private const val PREFS = "ripguard_config"
    private const val KEY_BASE_URL = "base_url"

    suspend fun load(context: Context) = withContext(Dispatchers.IO) {
        try {
            println("Loading BASE_URL...!")
            val prefs = context.getSharedPreferences(PREFS, Context.MODE_PRIVATE)

            // Load cached URL first
            BASE_URL = prefs.getString(KEY_BASE_URL, BASE_URL) ?: BASE_URL

            val url =
                "https://raw.githubusercontent.com/KeyouXZ/RIPGUARD/config/ripguard.json"

            val client = OkHttpClient()

            val request = Request.Builder()
                .url(url)
                .build()

            val response = client.newCall(request).execute()

            if (response.isSuccessful) {
                val body = response.body?.string()

                val json = JSONObject(body!!)
                val remoteUrl = json.getString("base_url")

                BASE_URL = remoteUrl

                // Save to cache
                prefs.edit {
                    putString(KEY_BASE_URL, remoteUrl)
                }
            }
        } catch (e: Exception) {
            e.printStackTrace()
        }
    }
}