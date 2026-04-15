package com.skyo.ripguard.ui.edukasi

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Card
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import com.skyo.ripguard.controller.ChromeController
import com.skyo.ripguard.controller.UseChrome

@Composable
fun EdukasiScreen(chrome: ChromeController, navController: NavController, topBar: @Composable () -> Unit) {
    UseChrome(chrome, topBar)

    val menuItems = listOf(
        "Mengenal Arus Balik" to "Pahami apa itu Rip Current dan mengapa ia sangat berbahaya.",
        "Kenali Tanda Bahaya" to "Pelajari cara membedakan area air tenang yang berbahaya dengan mata telanjang.",
        "Deteksi Dini di Lokasi" to "Langkah praktis memantau kondisi pantai sebelum kamu memutuskan untuk berenang.",
        "Protokol Penyelamatan" to "Panduan langkah demi langkah untuk meloloskan diri jika kamu terjebak dalam arus."
    )

    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp)
    ) {
        menuItems.forEachIndexed { index, (title, desc) ->
            Card(
                modifier = Modifier.fillMaxWidth(),
                onClick = {
                    navController.navigate("edukasi_id/$index")
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