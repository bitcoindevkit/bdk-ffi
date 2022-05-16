plugins {
    id("com.android.library")
    id("kotlin-android")
    id("maven-publish")
    id("signing")

    // API docs
    id("org.jetbrains.dokka")

    // Custom plugin to generate the native libs and bindings file
    id("org.bitcoindevkit.plugins.generate-android-bindings")
}

android {
    compileSdk = 31

    defaultConfig {
        minSdk = 21
        targetSdk = 31
        // versionCode = 1
        // versionName = "v0.2.2"
        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
        consumerProguardFiles("consumer-rules.pro")
    }

    buildTypes {
        getByName("release") {
            isMinifyEnabled = false
            proguardFiles(file("proguard-android-optimize.txt"), file("proguard-rules.pro"))
        }
    }

    publishing {
        singleVariant("release") {
            withSourcesJar()
            withJavadocJar()
        }
    }
}

dependencies {
    implementation("net.java.dev.jna:jna:5.8.0@aar")
    implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk7")
    implementation("androidx.appcompat:appcompat:1.4.0")
    implementation("androidx.core:core-ktx:1.7.0")
    api("org.slf4j:slf4j-api:1.7.30")

    androidTestImplementation("com.github.tony19:logback-android:2.0.0")
    androidTestImplementation("androidx.test.ext:junit:1.1.3")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.4.0")
    androidTestImplementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.4.1")
}

afterEvaluate {
    publishing {
        publications {
            create<MavenPublication>("maven") {
                groupId = "org.bitcoindevkit"
                artifactId = "bdk-android"
                version = "0.7.0-SNAPSHOT"
                from(components["release"])
                pom {
                    name.set("bdk-android")
                    description.set("Bitcoin Dev Kit Kotlin language bindings.")
                    url.set("https://bitcoindevkit.org")
                    licenses {
                        license {
                            name.set("APACHE 2.0")
                            url.set("https://github.com/bitcoindevkit/bdk/blob/master/LICENSE-APACHE")
                        }
                        license {
                            name.set("MIT")
                            url.set("https://github.com/bitcoindevkit/bdk/blob/master/LICENSE-MIT")
                        }
                    }
                    developers {
                        developer {
                            id.set("notmandatory")
                            name.set("Steve Myers")
                            email.set("notmandatory@noreply.github.org")
                        }
                        developer {
                            id.set("artfuldev")
                            name.set("Sudarsan Balaji")
                            email.set("sudarsan.balaji@artfuldev.com")
                        }
                    }
                    scm {
                        connection.set("scm:git:github.com/bitcoindevkit/bdk-ffi.git")
                        developerConnection.set("scm:git:ssh://github.com/bitcoindevkit/bdk-ffi.git")
                        url.set("https://github.com/bitcoindevkit/bdk-ffi/tree/master")
                    }
                }
            }
        }
    }
}

signing {
    useGpgCmd()
    sign(publishing.publications)
}

tasks.withType<org.jetbrains.dokka.gradle.DokkaTask>().configureEach {
    dokkaSourceSets {
        named("main") {
            moduleName.set("bdk-android")
            moduleVersion.set("0.7.0-SNAPSHOT")
            includes.from("Module.md")
        }
    }
}
