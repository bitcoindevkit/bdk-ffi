import org.gradle.api.tasks.testing.logging.TestExceptionFormat.*
import org.gradle.api.tasks.testing.logging.TestLogEvent.*
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

// library version is defined in gradle.properties
val libraryVersion: String by project

plugins {
    id("org.jetbrains.kotlin.jvm")
    id("org.gradle.java-library")
    id("org.gradle.maven-publish")
    id("org.gradle.signing")
    id("org.jetbrains.dokka")
    id("org.jetbrains.dokka-javadoc")
}

java {
    sourceCompatibility = JavaVersion.VERSION_11
    targetCompatibility = JavaVersion.VERSION_11
    withSourcesJar()
    withJavadocJar()
}

tasks.withType<org.jetbrains.kotlin.gradle.tasks.KotlinCompile> {
    kotlinOptions {
        jvmTarget = "11"
    }
}

// This block ensures that the tests that require access to a blockchain are not
// run if the -P excludeConnectedTests flag is passed to gradle.
// This ensures our CI runs are not fickle by not requiring access to testnet or signet.
// This is a workaround until we have a proper regtest setup for the CI.
// Note that the command in the CI is ./gradlew test -P excludeConnectedTests
tasks.test {
    if (project.hasProperty("excludeConnectedTests")) {
        exclude("**/LiveElectrumClientTest.class")
        exclude("**/LiveMemoryWalletTest.class")
        exclude("**/LiveTransactionTest.class")
        exclude("**/LiveTxBuilderTest.class")
        exclude("**/LiveWalletTest.class")
        exclude("**/LiveKyotoTest.class")
    }
}

testing {
    suites {
        val test by getting(JvmTestSuite::class) {
            useKotlinTest("1.9.23")
        }
    }
}

tasks.withType<Test> {
    testLogging {
        events(PASSED, SKIPPED, FAILED, STANDARD_OUT, STANDARD_ERROR)
        exceptionFormat = FULL
        showExceptions = true
        showStackTraces = true
        showCauses = true
    }
}

dependencies {
    implementation(platform("org.jetbrains.kotlin:kotlin-bom"))
    implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk7")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.8.1")
    implementation("net.java.dev.jna:jna:5.14.0")
    api("org.slf4j:slf4j-api:1.7.30")

    // testImplementation("org.junit.jupiter:junit-jupiter-api:5.10.1")
    // testRuntimeOnly("org.junit.jupiter:junit-jupiter-engine:5.10.1")
    // testRuntimeOnly("org.junit.vintage:junit-vintage-engine:5.8.2")
    testImplementation("ch.qos.logback:logback-classic:1.2.3")
    testImplementation("ch.qos.logback:logback-core:1.2.3")
}

afterEvaluate {
    publishing {
        publications {
            create<MavenPublication>("maven") {
                groupId = "org.bitcoindevkit"
                artifactId = "bdk-jvm"
                version = libraryVersion

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

    val signingKeyId: String? by project
    val signingKey: String? by project
    val signingPassword: String? by project
    useInMemoryPgpKeys(signingKeyId, signingKey, signingPassword)
    sign(publishing.publications)
}

dokka {
    moduleName.set("bdk-jvm")
    moduleVersion.set(libraryVersion)
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
