package com.skyo.ripguard

import com.skyo.ripguard.viewmodel.DetectionViewModel
import com.skyo.ripguard.viewmodel.EducationViewModel
import com.skyo.ripguard.viewmodel.NavbarViewModel

object NavbarViewModelSingleton {
    private var instance: NavbarViewModel? = null

    fun get(): NavbarViewModel {
        if (instance == null) {
            instance = NavbarViewModel()
        }
        return instance!!
    }
}

object EducationViewModelSingleton {
    private var instance: EducationViewModel? = null

    fun get(): EducationViewModel {
        if (instance == null) {
            instance = EducationViewModel()
        }
        return instance!!
    }
}

object DetectionViewModelSingleton {
    private var instance: DetectionViewModel? = null

    fun get(): DetectionViewModel {
        if (instance == null) {
            instance = DetectionViewModel()
        }
        return instance!!
    }
}