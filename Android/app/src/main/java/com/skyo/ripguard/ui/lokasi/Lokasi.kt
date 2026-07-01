// Copyright (C) 2026 Keyou
// SPDX-License-Identifier: AGPL-3.0-or-later

package com.skyo.ripguard.ui.lokasi

import android.util.Log
import android.widget.Toast
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.BottomSheetScaffold
import androidx.compose.material3.DrawerState
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.FloatingActionButton
import androidx.compose.material3.ModalBottomSheet
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.dp
import androidx.compose.ui.viewinterop.AndroidView
import androidx.navigation.NavController
import com.skyo.ripguard.ConfigManager
import com.skyo.ripguard.controller.ChromeController
import com.skyo.ripguard.controller.UseChrome
import com.skyo.ripguard.model.DetectionRes
import com.skyo.ripguard.viewmodel.LocationViewModel
import kotlinx.coroutines.CoroutineScope
import org.osmdroid.config.Configuration
import org.osmdroid.tileprovider.tilesource.TileSourceFactory
import org.osmdroid.util.GeoPoint
import org.osmdroid.views.CustomZoomButtonsController
import org.osmdroid.views.MapView
import org.osmdroid.views.overlay.Polygon

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun LokasiScreen(chrome: ChromeController, locationViewModel: LocationViewModel, navController: NavController, drawerState: DrawerState, scope: CoroutineScope) {
    UseChrome(chrome, topBar = {
        LocationTopBar(
            navController,
            drawerState,
            scope
        )
    })

    val context = LocalContext.current

    val mapView = remember {
        MapView(context).apply {
            Configuration.getInstance().userAgentValue = context.packageName

            setTileSource(TileSourceFactory.MAPNIK)

            controller.setZoom(16.0)
            //  8.0253993°S 110.3287713°E Parangtritis
            controller.setCenter(GeoPoint(-8.0253993, 110.3287713))

            setMultiTouchControls(true)

            zoomController.setVisibility(CustomZoomButtonsController.Visibility.NEVER)
        }
    }

    val locations by locationViewModel.locations.collectAsState()
    var selectedLocation by remember { mutableStateOf<DetectionRes?>(null) }

    Box(modifier = Modifier.fillMaxSize()) {
        AndroidView(
            factory = { mapView },
            modifier = Modifier.fillMaxSize(),
            update = { map ->
                map.overlays.removeAll { it is Polygon }
                Log.d("MAP", "locations = ${locations.size}")

                locations.forEach { detection ->
                    Log.d("MAP", "${detection.latitude}, ${detection.longitude}")

                    detection.detections.forEach { det ->

                        val polygon = Polygon().apply {
                            val lat = detection.latitude
                            val lon = detection.longitude

                            val size = 0.00005

                            points = listOf(
                                GeoPoint(lat + size, lon - size),
                                GeoPoint(lat + size, lon + size),
                                GeoPoint(lat - size, lon + size),
                                GeoPoint(lat - size, lon - size)
                            )

                            fillColor = 0x5500FF00
                            strokeColor = 0xFF00FF00.toInt()
                            strokeWidth = 4f

                            setOnClickListener { _, _, _ ->
                                selectedLocation = detection
                                true
                            }
                        }

                        map.overlays.add(polygon)
                    }
                }

                map.invalidate()
            }
        )

        Column(
            modifier = Modifier
                .align(Alignment.BottomEnd)
                .padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(12.dp)
        ) {

            FloatingActionButton(
                onClick = { mapView.controller.zoomIn() }
            ) {
                Text("+")
            }

            FloatingActionButton(
                onClick = { mapView.controller.zoomOut() }
            ) {
                Text("−")
            }
        }

        selectedLocation?.let { location ->

            ModalBottomSheet(
                onDismissRequest = { selectedLocation = null }
            ) {
                RipInfoBottomSheet(
                    location.detections.size,
                    location.windSpeed,
                    "${ConfigManager.BASE_URL}/image/${location.id}"
                )
            }
        }
    }
}