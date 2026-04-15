package com.skyo.ripguard.ui.beranda

import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Card
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import com.skyo.ripguard.R
import com.skyo.ripguard.controller.ChromeController
import com.skyo.ripguard.controller.UseChrome
import com.skyo.ripguard.ui.navbar.Destination

data class MenuItem(
    val title: String,
    val desc: String,
    val route: Destination
)
@Composable
fun BerandaScreen(chrome: ChromeController, topBar: @Composable () -> Unit, navTo: (Destination) -> Unit) {
    UseChrome(chrome, topBar)

    val menuItems = listOf(
        MenuItem(
            "Lokasi Rip Current",
            "Lihat lokasi Rip Current secara realtime dan detail",
            Destination.LOKASI
        ),
        MenuItem(
            "Deteksi Rip Current",
            "Deteksi lokasi Rip Current menggunakan kamera HP",
            Destination.DETEKSI
        ),
        MenuItem(
            "Pengetahuan Rip Current",
            "Ketahui pengertian, ciri-ciri, cara mendeteksi, dan cara melepaskan diri dari Rip Current",
            Destination.EDUKASI
        )
    )

    // example of destination: Destination.LOKASI

    Box(modifier = Modifier.fillMaxSize()) {
        Image(
            painter = painterResource(id = R.drawable.beranda),
            contentDescription = null,
            modifier = Modifier.fillMaxSize(),
            contentScale = ContentScale.Crop
        )

        val backgroundColor = MaterialTheme.colorScheme.background

        Box(
            modifier = Modifier
                .fillMaxSize()
                .background(backgroundColor.copy(alpha = 0.45f))
        )

        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(12.dp)
        ) {
            Column(
                modifier = Modifier.padding(8.dp, 0.dp, 0.dp, 0.dp)
            ) {
                Text(
                    text = "RIPGUARD",
                    fontWeight = FontWeight.Bold,
                    style = MaterialTheme.typography.titleLarge
                )
                Text("Teknologi Cerdas untuk Pesisir yang Lebih Aman")
            }

            menuItems.forEach { (title, desc, dest) ->
                Card(
                    modifier = Modifier.fillMaxWidth(),
                    onClick = {
                        navTo(dest)
                    }
                ) {
                    Column(
                        modifier = Modifier.padding(16.dp)
                    ) {
                        Text(title, fontWeight = FontWeight.Bold, style = MaterialTheme.typography.titleMedium)
                        Text(desc)
                    }
                }
            }
        }
    }
}