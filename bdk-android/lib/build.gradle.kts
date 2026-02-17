import org.jetbrains.kotlin.gradle.dsl.JvmTarget
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
    id("com.android.library")
    id("org.jetbrains.kotlin.android")
    id("org.gradle.maven-publish")
    id("org.gradle.signing")
    id("org.jetbrains.dokka")
    id("org.jetbrains.dokka-javadoc")
}

group = "org.bitcoindevkit"
version = "2.4.0-SNAPSHOT"

android {
    namespace = group.toString()
    compileSdk = 34

    defaultConfig {
        minSdk = 24
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

tasks.withType<KotlinCompile> {
    compilerOptions {
        jvmTarget.set(JvmTarget.JVM_17)
    }
}

java {
    toolchain {
        languageVersion.set(JavaLanguageVersion.of(17))
    }
}

dependencies {
    implementation("net.java.dev.jna:jna:5.14.0@aar")
    implementation("androidx.appcompat:appcompat:1.4.0")
    implementation("androidx.core:core-ktx:1.7.0")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.8.1")
    api("org.slf4j:slf4j-api:1.7.30")

    androidTestImplementation("com.github.tony19:logback-android:2.0.0")
    androidTestImplementation("androidx.test.ext:junit:1.3.0")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.4.0")
    androidTestImplementation("org.jetbrains.kotlin:kotlin-test:1.6.10")
    androidTestImplementation("org.jetbrains.kotlin:kotlin-test-junit:1.6.10")
}

afterEvaluate {
    publishing {
        publications {
            create<MavenPublication>("maven") {
                groupId = group.toString()
                artifactId = "bdk-android"
                version = version.toString()

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
                            id.set("bdkdevelopers")
                            name.set("Bitcoin Dev Kit Developers")
                            email.set("dev@bitcoindevkit.org")
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
    if (project.hasProperty("localBuild")) {
        isRequired = false
    }
    sign(publishing.publications)
}

dokka {
    moduleName.set("bdk-android")
    moduleVersion.set(version.toString())
    dokkaSourceSets.main {
        includes.from("README.md")
        sourceLink {
            localDirectory.set(file("src/main/kotlin"))
            remoteUrl("https://bitcoindevkit.org/")
            remoteLineSuffix.set("#L")
        }
    }
    pluginsConfiguration.html {
        // customStyleSheets.from("styles.css")
        // customAssets.from("logo.svg")
        footerMessage.set("(c) Bitcoin Dev Kit Developers")
    }
}
