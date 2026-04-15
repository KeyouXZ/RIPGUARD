package com.skyo.ripguard.ui.edukasi

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.layout.widthIn
import androidx.compose.material3.FloatingActionButton
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import com.skyo.ripguard.R

@Composable
fun EducationFAB(navController: NavController, currentIndex: Int, prevText: String?, nextText: String?) {

    Row(
        modifier = Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.SpaceBetween
    ) {
        // Prev
        Box(
            modifier = Modifier.weight(1f),
            contentAlignment = Alignment.CenterStart
        ) {
            if (prevText != null) {
                FloatingActionButton(
                    onClick = {
                        navController.navigate("edukasi_id/" + (currentIndex - 1)) {
                            launchSingleTop = true
                        }
                    },
                    modifier = Modifier.padding(start = 32.dp)
                ) {
                    Row(modifier = Modifier.padding(horizontal = 8.dp).widthIn(max = 160.dp)) {
                        Icon(
                            painter = painterResource(R.drawable.ic_backward),
                            contentDescription = "Prev"
                        )
                        Spacer(Modifier.width(4.dp))
                        Text(
                            text = prevText,
                            maxLines = 1,
                            overflow = TextOverflow.Ellipsis
                        )
                    }
                }
            }
        }

        Box(
            modifier = Modifier.weight(1f),
            contentAlignment = Alignment.CenterEnd
        ) {
            if (nextText != null) {
                FloatingActionButton(
                    onClick = {
                        navController.navigate("edukasi_id/" + (currentIndex + 1)) {
                            launchSingleTop = true
                        }
                    }
                ) {
                    Row(modifier = Modifier.padding(horizontal = 8.dp).widthIn(max = 160.dp)) {
                        Text(
                            text = nextText,
                            maxLines = 1,
                            overflow = TextOverflow.Ellipsis
                        )
                        Spacer(Modifier.width(4.dp))
                        Icon(
                            painter = painterResource(R.drawable.ic_forward),
                            contentDescription = "Next"
                        )
                    }
                }
            }
        }
    }
}