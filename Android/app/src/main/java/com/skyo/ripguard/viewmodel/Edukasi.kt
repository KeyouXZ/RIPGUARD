package com.skyo.ripguard.viewmodel

import androidx.lifecycle.ViewModel
import com.skyo.ripguard.model.Education
import com.skyo.ripguard.model.EducationContent

class EducationViewModel : ViewModel() {

    private val educationList = listOf(
        Education(
            id = 0,
            title = "Mengenal Arus Balik",
            contents = listOf(
                EducationContent.Text("   Banyak yang salah kaprah menganggap arus ini sebagai \"ombak besar\". Padahal, rip current adalah aliran air sempit yang bergerak sangat kuat menjauhi pantai. Bayangkan seperti sungai kecil di tengah laut yang justru mengalir ke arah laut lepas, bukan ke arah daratan.\n" +
                        "\n" +
                        "   Arus ini terbentuk karena akumulasi air yang dibawa ombak ke pantai harus kembali ke laut. Air mencari jalan keluar yang paling mudah, biasanya melalui celah di antara gundukan pasir bawah laut, sehingga menciptakan aliran keluar yang sangat deras dan mematikan.")
            )
        ),
        Education(
            id = 1,
            title = "One",
            contents = listOf(
                EducationContent.Text("Text for ID 1")
            )
        ),
        Education(
            id = 2,
            title = "Two",
            contents = listOf(
                EducationContent.Text("Text for ID 2")
            )
        ),
        Education(
            id = 3,
            title = "Three",
            contents = listOf(
                EducationContent.Text("Text for ID 3")
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