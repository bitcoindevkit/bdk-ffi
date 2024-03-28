rootProject.name = "bdk-jvm"

include(":lib")
includeBuild("plugins")

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
