package com.skyo.ripguard.ui.navbar

import android.annotation.SuppressLint
import android.app.Activity
import android.app.Application
import android.content.Intent
import android.widget.Toast
import androidx.activity.compose.BackHandler
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material3.CenterAlignedTopAppBar
import androidx.compose.material3.DrawerState
import androidx.compose.material3.DrawerValue
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.FabPosition
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.ModalDrawerSheet
import androidx.compose.material3.ModalNavigationDrawer
import androidx.compose.material3.NavigationDrawerItem
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.rememberDrawerState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.rememberNavController
import androidx.navigation.navArgument
import com.skyo.ripguard.EducationViewModelSingleton
import com.skyo.ripguard.R
import com.skyo.ripguard.controller.ChromeController
import com.skyo.ripguard.controller.UseChrome
import com.skyo.ripguard.ui.beranda.BerandaScreen
import com.skyo.ripguard.ui.edukasi.EduScreen
import com.skyo.ripguard.ui.edukasi.EducationFAB
import com.skyo.ripguard.ui.edukasi.EdukasiScreen
import com.skyo.ripguard.viewmodel.NavbarViewModel
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.launch

@Composable
fun DummyScreen(chrome: ChromeController, text: String, topBar: @Composable () -> Unit) {
    UseChrome(chrome, topBar)

    Box {
        Text(text)
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun DefaultTopBar(
    navController: NavController,
    drawerState: DrawerState,
    scope: CoroutineScope,
    text: String = "Skyo"
) {
    val navBackStackEntry by navController.currentBackStackEntryAsState()
    val currentRoute = navBackStackEntry?.destination?.route
    val currentDestination = Destination.entries.find { it.route == currentRoute }

    CenterAlignedTopAppBar(
        title = {
            Text(currentDestination?.label ?: text)
        },
        navigationIcon = {
            IconButton(onClick = {
                scope.launch {
                    if (drawerState.isClosed) drawerState.open()
                    else drawerState.close()
                }
            }) {
                Icon(
                    painter = painterResource(R.drawable.ic_menu),
                    contentDescription = "Menu"
                )
            }
        }
    )
}

@Composable
fun BackPressWithConfirmation() {
    val context = LocalContext.current
    val backPressedOnce = remember { mutableStateOf(false) }
    val scope = rememberCoroutineScope()

    BackHandler {
        if (backPressedOnce.value) {
            (context as? Activity)?.finish()
        } else {
            backPressedOnce.value = true
            Toast.makeText(context, "Press back again to exit", Toast.LENGTH_SHORT).show()

            scope.launch {
                kotlinx.coroutines.delay(2000)
                backPressedOnce.value = false
            }
        }
    }
}

@SuppressLint("ViewModelConstructorInComposable")
@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun NavDrawer(
    intent: Intent,
    navViewModel: NavbarViewModel
) {
    val navController = rememberNavController()

    BackPressWithConfirmation()

    val drawerState = rememberDrawerState(initialValue = DrawerValue.Closed)
    val scope = rememberCoroutineScope()

    val context = LocalContext.current.applicationContext as Application

    val educationViewModel = remember { EducationViewModelSingleton.get() }

    val selectedDestination = navViewModel.selectedDestination

    val chrome = remember { ChromeController() }

    ModalNavigationDrawer(
        drawerState = drawerState,
        gesturesEnabled = drawerState.isOpen,
        drawerContent = {
            ModalDrawerSheet(
                modifier = Modifier.fillMaxWidth(0.75f)
            ) {
                LazyColumn(
                    modifier = Modifier.padding(horizontal = 16.dp)
                ) {
                    item {
                        Spacer(Modifier.height(12.dp))
                        Text(
                            "RIPGUARD - Pantai Aman, Hati Tenang",
                            modifier = Modifier.padding(14.dp),
                            style = MaterialTheme.typography.titleMedium,
                            fontWeight = FontWeight.Bold
                        )

                        Column {
                            Destination.entries.forEach { destination ->
                                val selected = selectedDestination == destination

                                NavigationDrawerItem(
                                    label = { Text(destination.label) },
                                    icon = {
                                        Icon(
                                            painterResource(destination.icon),
                                            contentDescription = destination.contentDescription
                                        )
                                    },
                                    selected = selected,
                                    onClick = {
                                        if (navController.currentDestination?.route != destination.route) {
                                            navController.navigate(destination.route) {
                                                popUpTo(0) { inclusive = true }
                                                launchSingleTop = true
                                            }
                                            navViewModel.setSelectedDestination(destination)
                                        }

                                        scope.launch { drawerState.close() }
                                    }
                                )
                            }
                        }
                    }
                }
            }
        }
    ) {
        Scaffold(
            topBar = { chrome.topBar?.invoke() },
            floatingActionButton = { chrome.fab?.invoke() },
            floatingActionButtonPosition = chrome.fabPosition ?: FabPosition.End,
            bottomBar = { chrome.bottomBar?.invoke() }
        ) { innerPadding ->
            NavHost(
                navController,
                startDestination = Destination.BERANDA.route,
                modifier = Modifier.padding(innerPadding)
            ) {
                composable("beranda") {
                    BerandaScreen(chrome, @Composable {
                        DefaultTopBar(
                            navController,
                            drawerState,
                            scope
                        )
                    }, {
                        navController.navigate(it.route) {
                            popUpTo(0) { inclusive = true }
                            launchSingleTop = true
                        }
                        navViewModel.setSelectedDestination(it)
                    })
                }
                composable("lokasi") {
                    DummyScreen(chrome, "Lokasi", @Composable {
                        DefaultTopBar(
                            navController,
                            drawerState,
                            scope
                        )
                    })
                }
                composable("deteksi") {
                    DummyScreen(chrome, "Deteksi", @Composable {
                        DefaultTopBar(
                            navController,
                            drawerState,
                            scope
                        )
                    })
                }
                composable("edukasi") {
                    EdukasiScreen(chrome, navController, @Composable {
                        DefaultTopBar(
                            navController,
                            drawerState,
                            scope
                        )
                    })
                }

                composable(
                    route = "edukasi_id/{id}",
                    arguments = listOf(navArgument("id") { type = NavType.IntType })
                ) {
                    val id = it.arguments?.getInt("id")

                    val edu = educationViewModel.getEducationById(id!!)
                    val nextText = educationViewModel.getNextEducationTitle(id)
                    val prevText = educationViewModel.getPrevEducationTitle(id)

                    EduScreen(
                        chrome, edu,
                        @Composable {
                            DefaultTopBar(
                                navController,
                                drawerState,
                                scope,
                                edu!!.title
                            )
                        },
                        @Composable {
                            EducationFAB(navController, id, prevText, nextText)
                        })
                }
            }
        }
    }
}
