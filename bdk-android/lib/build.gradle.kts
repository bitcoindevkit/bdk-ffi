import com.vanniktech.maven.publish.AndroidSingleVariantLibrary
import com.vanniktech.maven.publish.JavadocJar
import com.vanniktech.maven.publish.SourcesJar
import org.jetbrains.kotlin.gradle.dsl.JvmTarget
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
    id("com.android.library")
    id("org.jetbrains.kotlin.android")
    id("org.jetbrains.dokka")
    id("com.vanniktech.maven.publish")
}

group = "org.bitcoindevkit"
version = "2.3.1"

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

mavenPublishing {
    coordinates(
        groupId = group.toString(),
        artifactId = "bdk-android",
        version = version.toString()
    )

    pom {
        name.set("bdk-android")
        description.set("Bitcoin Dev Kit language bindings for Android.")
        url.set("https://bitcoindevkit.org")
        inceptionYear.set("2021")
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
            url.set("https://github.com/bitcoindevkit/bdk-ffi/")
            connection.set("scm:git:github.com/bitcoindevkit/bdk-ffi.git")
            developerConnection.set("scm:git:ssh://github.com/bitcoindevkit/bdk-ffi.git")
        }
    }

    configure(
        AndroidSingleVariantLibrary(
            javadocJar = JavadocJar.Dokka("dokkaGeneratePublicationHtml"),
            sourcesJar = SourcesJar.Sources(),
            variant = "release",
        )
    )

    publishToMavenCentral()
    signAllPublications()
}

dokka {
    moduleName.set("bdk-android")
    moduleVersion.set(version.toString())
    dokkaSourceSets.main {
        includes.from("../docs/DOKKA_LANDING.md")
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
