package com.skyo.ripguard

import android.content.Context
import android.content.Intent
import android.os.Bundle
import android.widget.Toast
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.MutableState
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.lifecycle.lifecycleScope
import com.skyo.ripguard.ui.navbar.NavDrawer
import com.skyo.ripguard.ui.theme.RIPGUARDTheme
import com.skyo.ripguard.utility.requestNotificationPermission
import kotlinx.coroutines.launch

@Composable
fun AppWithToast(context: Context, startTimestamp: Long, intentState: MutableState<Intent?>) {
    val navViewModel = remember { NavbarViewModelSingleton.get() }

    LaunchedEffect(Unit) {
        if (!navViewModel.isInitialized) {
            Toast.makeText(
                context,
                "App started in ${System.currentTimeMillis() - startTimestamp}ms",
                Toast.LENGTH_SHORT
            ).show()

            navViewModel.setInitialized()
        }
    }

    NavDrawer(intentState.value ?: Intent(), navViewModel)
}

class MainActivity : ComponentActivity() {
    private val _intentState = mutableStateOf<Intent?>(null)

    override fun onCreate(savedInstanceState: Bundle?) {
        val startTime = System.currentTimeMillis()

        super.onCreate(savedInstanceState)
        enableEdgeToEdge()

        lifecycleScope.launch {
            requestNotificationPermission(this@MainActivity, this@MainActivity)
        }

        setContent {
            RIPGUARDTheme {
                AppWithToast(this, startTime, _intentState)
            }
        }
    }
}