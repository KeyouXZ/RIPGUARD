package com.skyo.ripguard.ui.edukasi

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.unit.dp
import com.skyo.ripguard.controller.ChromeController
import com.skyo.ripguard.controller.UseChrome
import com.skyo.ripguard.model.Education
import com.skyo.ripguard.model.EducationContent

@Composable
fun EduScreen(chrome: ChromeController, edu: Education?, topBar: @Composable () -> Unit, fab: @Composable () -> Unit) {
    UseChrome(chrome, topBar, fab)

    if (edu != null) {
        Column(
            modifier = Modifier.padding(8.dp).fillMaxWidth()
        ) {
            edu.contents.forEach { content ->
                when (content) {
                    is EducationContent.Text -> {
                        Text(content.value)
                    }

                    is EducationContent.Image -> {
                        Image(
                            painter = painterResource(id = content.resId),
                            contentDescription = null,
                            modifier = Modifier.fillMaxWidth(),
                            contentScale = ContentScale.Crop
                        )
                    }
                }
            }
        }
    } else {
        Text("Data not found")
    }
}