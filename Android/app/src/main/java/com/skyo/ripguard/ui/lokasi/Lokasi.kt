package com.skyo.ripguard.ui.lokasi

import android.widget.Toast
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.BottomSheetScaffold
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.FloatingActionButton
import androidx.compose.material3.ModalBottomSheet
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.dp
import androidx.compose.ui.viewinterop.AndroidView
import com.skyo.ripguard.controller.ChromeController
import com.skyo.ripguard.controller.UseChrome
import org.osmdroid.config.Configuration
import org.osmdroid.tileprovider.tilesource.TileSourceFactory
import org.osmdroid.util.GeoPoint
import org.osmdroid.views.CustomZoomButtonsController
import org.osmdroid.views.MapView
import org.osmdroid.views.overlay.Polygon

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun LokasiScreen(chrome: ChromeController, topBar: @Composable () -> Unit) {
    UseChrome(chrome, topBar)

    val context = LocalContext.current
    var showBottomSheet by remember { mutableStateOf(false) }

    val mapView = remember {
        MapView(context).apply {
            Configuration.getInstance().userAgentValue = context.packageName

            setTileSource(TileSourceFactory.MAPNIK)

            controller.setZoom(15.0)
            //  8.0253993°S 110.3287713°E Parangtritis
            controller.setCenter(GeoPoint(-8.0253993, 110.3287713))

            setMultiTouchControls(true)

            zoomController.setVisibility(CustomZoomButtonsController.Visibility.NEVER)
        }
    }

    val polygon = Polygon().apply {
        points = listOf(
            GeoPoint(-8.0253993, 110.3287713),
            GeoPoint(-8.0260000, 110.3295000),
            GeoPoint(-8.0245000, 110.3300000)
        )

        fillColor = 0x5500FF00
        strokeColor = 0xFF00FF00.toInt()
        strokeWidth = 4f

        setOnClickListener { _, _, _ ->
            showBottomSheet = true
            true
        }
    }

    mapView.overlays.add(polygon)

    Box(modifier = Modifier.fillMaxSize()) {
        AndroidView(
            factory = { mapView },
            modifier = Modifier.fillMaxSize()
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

        if (showBottomSheet) {
            ModalBottomSheet (
                onDismissRequest = { showBottomSheet = false }
            ) {
                RipInfoBottomSheet(1, 30.1F) // Dummy
            }
        }
    }
}