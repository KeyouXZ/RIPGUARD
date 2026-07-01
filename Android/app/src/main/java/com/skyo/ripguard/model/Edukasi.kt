// Copyright (C) 2026 KeyouXZ
// SPDX-License-Identifier: AGPL-3.0-or-later

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
