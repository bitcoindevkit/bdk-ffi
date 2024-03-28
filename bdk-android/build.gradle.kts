plugins {
    id("com.android.library").version("8.3.1").apply(false)
    id("org.jetbrains.kotlin.android").version("1.9.23").apply(false)
    id("org.gradle.maven-publish")
    id("org.gradle.signing")
    id("org.bitcoindevkit.plugins.generate-android-bindings").apply(false)
    id("io.github.gradle-nexus.publish-plugin").version("1.1.0").apply(true)
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
