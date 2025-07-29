plugins {
    id("com.android.library").version("8.3.1").apply(false)
    id("org.jetbrains.kotlin.android").version("2.1.10").apply(false)
    id("org.gradle.maven-publish")
    id("org.gradle.signing")
    id("org.jetbrains.dokka").version("2.0.0").apply(false)
    id("org.jetbrains.dokka-javadoc").version("2.0.0").apply(false)
}

// library version is defined in gradle.properties
val libraryVersion: String by project
group = "org.bitcoindevkit"
version = libraryVersion
