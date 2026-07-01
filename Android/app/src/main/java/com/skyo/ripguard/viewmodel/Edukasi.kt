// Copyright (C) 2026 Keyou
// SPDX-License-Identifier: AGPL-3.0-or-later

package com.skyo.ripguard.viewmodel

import androidx.lifecycle.ViewModel
import com.skyo.ripguard.R
import com.skyo.ripguard.model.Education
import com.skyo.ripguard.model.EducationContent

class EducationViewModel : ViewModel() {

    private val educationList = listOf(
        Education(
            id = 0,
            title = "Mengenal Arus Balik",
            contents = listOf(
                EducationContent.Image(R.drawable.rip_current),
                EducationContent.Text("\n" +
                        "   Banyak yang salah kaprah menganggap arus ini sebagai \"ombak besar\". Padahal, rip current adalah aliran air sempit yang bergerak sangat kuat menjauhi pantai. Bayangkan seperti sungai kecil di tengah laut yang justru mengalir ke arah laut lepas, bukan ke arah daratan.\n" +
                        "\n" +
                        "   Arus ini terbentuk karena akumulasi air yang dibawa ombak ke pantai harus kembali ke laut. Air mencari jalan keluar yang paling mudah, biasanya melalui celah di antara gundukan pasir bawah laut, sehingga menciptakan aliran keluar yang sangat deras dan mematikan.")
            )
        ),
        Education(
            id = 1,
            title = "Kenali Tanda Bahaya",
            contents = listOf(
                EducationContent.Text("Tanda tanda yang paling mudah dikenali untuk mengetahui keberadaan Rip current yaitu : \n" +
                        "1. Tidak terbentuknya buih setelah gelombang pecah,\n" +
                        "2. Ombak tidak pecah dan permukaan air terlihat tenang" +
                        "\n"),
                EducationContent.Text("Cara mempelajari keberadaan rip current dengan mata telanjang \n" +
                        "Rip current itu kompleks, dapat dengan cepat berubah bentuk dan lokasi, dan terkadang sulit dilihat. Hal-hal yang perlu diperhatikan adalah:\n" +
                        "1. Air yang lebih dalam dan berwarna gelap,\n" +
                        "2. Lebih sedikit gelombang yang pecah,\n" +
                        "3. Permukaan yang tenang dikelilingi oleh air yang memiliki riak,\n" +
                        "4. Membawa sedimen seperti pasir ke arah laut\n" +
                        "\n" +
                        "Arus balik tidak selalu menunjukkan semua tanda ini sekaligus.\n" +
                        "Sementara tempat yang tak terdapat rip current memiliki tanda tanda sebagai berikut:\n" +
                        "1. Air yang lebih dangkal dan berwarna lebih terang,\n" +
                        "2. Terdapat banyak gelombang yang pecah,\n" +
                        "3. Permukaan nya terdapat banyak riak,")
            )
        ),
        Education(
            id = 2,
            title = "Deteksi Dini di Lokasi",
            contents = listOf(
                EducationContent.Text("Langkah praktis memantau kondisi pantai sebelum berenang \n" +
                        "\n" +
                        "1. Amati bagian pantai jika ada bagian yang memiliki Warna air yang gelap, ombaknya tenang, dan tak ada riak maka jangan berenang di area tersebut kerana area tersebut memiliki arus yang kuat\n" +
                        "2. Tetaplah di area pengawasan saat berenang agar petugas bisa membantu secepat mungkin jika terjadi laka laut\n" +
                        "3. Amati kondisi cuaca, jika terdapat angin kencang dan gelombang tinggi disarankan untuk tidak berenang karena memiliki resiko yang tinggi\n" +
                        "4. Jangan berenang sendirian selalu ajak teman atau keluarga agar saat terjadi hal yang tak diinginkan ada orang yang membantu")
            )
        ),
        Education(
            id = 3,
            title = "Protokol Penyelamatan",
            contents = listOf(
                EducationContent.Text("Langkah-langkah untuk meloloskan diri dari Rip current:\n" +
                        "1. Tetap tenang dan jangan mencoba untuk melawan arus rip current atau berenang ke arah pantai,\n" +
                        "2. Cobalah untuk mengapung\n" +
                        "3. Kamu bisa menggunakan 2 cara yaitu berenang ke samping atau biarkan tubuh mu terseret arus rip current sampai keluar dari arus rip current\n" +
                        "4. Saat merasa lelah cobalah untuk mengapung dan melambaikan tangan untuk meminta tolong\n" +
                        "5. Jika sudah keluar dari arus rip current berenanglah ke tepian" +
                        "\n"),
                EducationContent.Text("Cara untuk menyelamatkan orang yang terseret Rip current:\n" +
                        "1. Jangan coba coba untuk berenang ke arah korban ini hanya menyebabkan kamu ikut terjebak di Rip current\n" +
                        "2. Cobalah untuk mencari bantuan seperti tim penyelamat\n" +
                        "3. Kalau tidak ada cobalah untuk melempar benda yang dapat mengapung ke arah korban untuk membantu korban mengapung")
            )
        )
    )

    fun getEducationById(id: Int): Education? {
        return educationList.find { it.id == id }
    }

    fun getNextEducationTitle(currentId: Int): String? {
        val index = educationList.indexOfFirst { it.id == currentId }
        return educationList.getOrNull(index + 1)?.title
    }

    fun getPrevEducationTitle(currentId: Int): String? {
        val index = educationList.indexOfFirst { it.id == currentId }
        return educationList.getOrNull(index - 1)?.title
    }
}