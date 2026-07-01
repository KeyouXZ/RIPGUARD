// Copyright (C) 2026 Keyou
// SPDX-License-Identifier: AGPL-3.0-or-later

package com.skyo.ripguard.ui.lokasi

import androidx.compose.material3.CenterAlignedTopAppBar
import androidx.compose.material3.DrawerState
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.res.painterResource
import androidx.navigation.NavController
import androidx.navigation.compose.currentBackStackEntryAsState
import com.skyo.ripguard.R
import com.skyo.ripguard.ui.navbar.Destination
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.launch
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.size
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import com.skyo.ripguard.WSManager
import com.skyo.ripguard.WSStatus

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun LocationTopBar(
    navController: NavController,
    drawerState: DrawerState,
    scope: CoroutineScope,
) {
    val wsStatus by WSManager.status.collectAsStateWithLifecycle()

    val indicatorColor = when (wsStatus) {
        WSStatus.CONNECTED -> Color.Green
        WSStatus.RECONNECTING -> Color(0xFFFFA500) // Orange
        WSStatus.DISCONNECTED -> Color.Red
    }

    val navBackStackEntry by navController.currentBackStackEntryAsState()
    val currentRoute = navBackStackEntry?.destination?.route
    val currentDestination = Destination.entries.find { it.route == currentRoute }

    CenterAlignedTopAppBar(
        title = {
            Text(currentDestination?.label ?: "Lokasi")
        },
        navigationIcon = {
            IconButton(onClick = {
                scope.launch {
                    if (drawerState.isClosed) drawerState.open()
                    else drawerState.close()
                }
            }) {
                Icon(
                    painter = painterResource(R.drawable.ic_menu),
                    contentDescription = "Menu"
                )
            }
        },
        actions = {
            IconButton(onClick = {}) {
                Box {
                    Icon(
                        painter = painterResource(R.drawable.cloud),
                        contentDescription = "Connection Status"
                    )

                    Box(
                        modifier = Modifier
                            .size(8.dp)
                            .align(Alignment.TopEnd)
                            .background(indicatorColor, shape = androidx.compose.foundation.shape.CircleShape)
                    )
                }
            }
        }
    )
}