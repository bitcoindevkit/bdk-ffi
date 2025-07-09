rootProject.name = "bdk-jvm"

include(":lib")
include(":examples")

pluginManagement {
    repositories {
        gradlePluginPortal()
    }
}

dependencyResolutionManagement {
    repositories {
        mavenCentral()
    }
}
