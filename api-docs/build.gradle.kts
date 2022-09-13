import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
    kotlin("jvm") version "1.7.10"

    // API docs
    id("org.jetbrains.dokka") version "1.7.10"
}

repositories {
    mavenCentral()
}

dependencies {
    testImplementation(kotlin("test"))
}

tasks.test {
    useJUnitPlatform()
}

tasks.withType<KotlinCompile> {
    kotlinOptions.jvmTarget = "1.8"
}

tasks.withType<org.jetbrains.dokka.gradle.DokkaTask>().configureEach {
    dokkaSourceSets {
        named("main") {
            moduleName.set("bdk-android")
            moduleVersion.set("0.9.0")
            includes.from("Module.md")
            samples.from("src/main/kotlin/org/bitcoindevkit/Samples.kt")
        }
    }
}
