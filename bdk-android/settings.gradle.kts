rootProject.name = "bdk-android"

include(":lib")
includeBuild("plugins")

pluginManagement {
    repositories {
        gradlePluginPortal()
        google()
    }
}

dependencyResolutionManagement {
    repositories {
        mavenCentral()
        google()
    }
}
