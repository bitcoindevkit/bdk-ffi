import org.gradle.api.tasks.testing.logging.TestExceptionFormat.*
import org.gradle.api.tasks.testing.logging.TestLogEvent.*

plugins {
    id("org.jetbrains.kotlin.jvm")
    id("java-library")
    id("maven-publish")
    id("signing")

    // API docs
    id("org.jetbrains.dokka")

    // Custom plugin to generate the native libs and bindings file
    id("org.bitcoindevkit.plugins.generate-jvm-bindings")
}

java {
    sourceCompatibility = JavaVersion.VERSION_1_8
    targetCompatibility = JavaVersion.VERSION_1_8
    withSourcesJar()
    withJavadocJar()
}

tasks.withType<Test> {
    useJUnitPlatform()

    testLogging {
        events(PASSED, SKIPPED, FAILED, STANDARD_OUT, STANDARD_ERROR)
        exceptionFormat = FULL
        showExceptions = true
        showCauses = true
        showStackTraces = true
    }
}

dependencies {
    implementation(platform("org.jetbrains.kotlin:kotlin-bom"))
    implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk7")
    implementation("net.java.dev.jna:jna:5.8.0")
    api("org.slf4j:slf4j-api:1.7.30")
    testImplementation("junit:junit:4.13.2")
    testRuntimeOnly("org.junit.vintage:junit-vintage-engine:5.8.2")
    testImplementation("ch.qos.logback:logback-classic:1.2.3")
    testImplementation("ch.qos.logback:logback-core:1.2.3")
}

afterEvaluate {
    publishing {
        publications {
            create<MavenPublication>("maven") {

                groupId = "org.bitcoindevkit"
                artifactId = "bdk-jvm"
                version = "0.7.0-SNAPSHOT"

                from(components["java"])

                pom {
                    name.set("bdk-jvm")
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
            moduleName.set("bdk-jvm")
            moduleVersion.set("0.7.0-SNAPSHOT")
            includes.from("Module.md")
        }
    }
}
