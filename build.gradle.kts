buildscript {
    repositories {
        google()
        mavenCentral()
    }
    dependencies {
        classpath("com.android.tools.build:gradle:7.1.2")
        classpath("org.jetbrains.kotlin:kotlin-gradle-plugin:1.6.10")
    }
}

plugins {
    id("signing")
    id("maven-publish")
    id("io.github.gradle-nexus.publish-plugin") version "1.1.0"
}

// signing {
//     val signingKeyId: String? by project
//     val signingKey: String? by project
//     val signingPassword: String? by project
//     useInMemoryPgpKeys(signingKeyId, signingKey, signingPassword)
//     sign(publishing.publications)
// }

// does this need to be defined here? Not sure
// it used to be defined in the nexusPublishing block but is not required
// I think the group ID is defined in the specific publishing blocks in the respective build.gradle.kts
group = "org.bitcoindevkit"

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

allprojects {
    repositories {
        google()
        mavenCentral()
    }
}
