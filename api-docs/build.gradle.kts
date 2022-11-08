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
            moduleVersion.set("0.11.0")
            includes.from("Module1.md")
            samples.from("src/test/kotlin/org/bitcoindevkit/Samples.kt")
        }
    }
}

// tasks.withType<org.jetbrains.dokka.gradle.DokkaTask>().configureEach {
//     dokkaSourceSets {
//         named("main") {
//             moduleName.set("bdk-jvm")
//             moduleVersion.set("0.11.0")
//             includes.from("Module2.md")
//             samples.from("src/test/kotlin/org/bitcoindevkit/Samples.kt")
//         }
//     }
// }
