package com.skyo.ripguard.model

sealed class EducationContent {
    data class Text(val value: String) : EducationContent()
    data class Image(val resId: Int) : EducationContent()
}

data class Education(
    val id: Int,
    val title: String,
    val contents: List<EducationContent>
)
