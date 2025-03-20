plugins {
    id("org.jetbrains.kotlin.jvm").version("2.1.10").apply(false)
    id("org.gradle.java-library")
    id("org.gradle.maven-publish")
    id("org.gradle.signing")
    id("io.github.gradle-nexus.publish-plugin") version "1.1.0"
    id("org.jetbrains.dokka").version("2.0.0").apply(false)
    id("org.jetbrains.dokka-javadoc").version("2.0.0").apply(false)
}

// library version is defined in gradle.properties
val libraryVersion: String by project

// These properties are required here so that the nexus publish-plugin
// finds a staging profile with the correct group (group is otherwise set as "")
// and knows whether to publish to a SNAPSHOT repository or not
// https://github.com/gradle-nexus/publish-plugin#applying-the-plugin
group = "org.bitcoindevkit"
version = libraryVersion

nexusPublishing {
    repositories {
        create("sonatype") {
            nexusUrl.set(uri("https://s01.oss.sonatype.org/service/local/"))
            snapshotRepositoryUrl.set(uri("https://s01.oss.sonatype.org/content/repositories/snapshots/"))

            val ossrhUsername: String? by project
            val ossrhPassword: String? by project
            username.set(ossrhUsername)
            password.set(ossrhPassword)
        }
    }
}
