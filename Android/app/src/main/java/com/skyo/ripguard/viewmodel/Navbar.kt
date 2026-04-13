package com.skyo.ripguard.viewmodel

import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import androidx.lifecycle.ViewModel
import com.skyo.ripguard.ui.navbar.Destination

class NavbarViewModel : ViewModel() {
    private var _selectedDestination by mutableIntStateOf(Destination.BERANDA.ordinal)

    val selectedDestination: Destination
        get() = Destination.entries[_selectedDestination]

    private var _initialized by mutableStateOf(false)
    val isInitialized: Boolean get() = _initialized

    fun setInitialized() {
        _initialized = true
    }

    fun setSelectedDestination(destination: Destination) {
        _selectedDestination = destination.ordinal
    }
}