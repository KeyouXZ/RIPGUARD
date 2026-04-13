package com.skyo.ripguard.ui.navbar

import androidx.annotation.DrawableRes
import com.skyo.ripguard.R

enum class Destination(
    val route: String,
    @get:DrawableRes val icon: Int,
    val contentDescription: String,
    val label: String,
) {
    BERANDA(
        route = "beranda",
        icon = R.drawable.ic_o_home,
        contentDescription = "Beranda",
        label = "Beranda"
    ),
    LOKASI(
        route = "lokasi",
        icon = R.drawable.ic_location_on,
        contentDescription = "Lokasi",
        label = "Lokasi",
    ),
    DETEKSI(
        route = "deteksi",
        icon = R.drawable.ic_o_camera,
        contentDescription = "Deteksi",
        label = "Deteksi",
    ),
    EDUKASI(
        route = "edukasi",
        icon = R.drawable.ic_o_book,
        contentDescription = "Zona Edukasi",
        label = "Zona Edukasi",
    ),
}