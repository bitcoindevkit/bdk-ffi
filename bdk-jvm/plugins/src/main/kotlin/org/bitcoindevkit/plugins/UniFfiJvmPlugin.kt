package org.bitcoindevkit.plugins

import org.gradle.api.DefaultTask
import org.gradle.api.Plugin
import org.gradle.api.Project
import org.gradle.api.tasks.Exec
import org.gradle.kotlin.dsl.getValue
import org.gradle.kotlin.dsl.provideDelegate
import org.gradle.kotlin.dsl.register

// TODO 18: Migrate hard coded strings to constants all in the same location so they're at least easy
//          to find and reason about.
internal class UniFfiJvmPlugin : Plugin<Project> {
    override fun apply(target: Project): Unit = target.run {

        // register a task called buildJvmBinaries which will run something like
        // cargo build --release --target aarch64-apple-darwin
        val buildJvmBinaries by tasks.register<DefaultTask>("buildJvmBinaries") {
            if (operatingSystem == OS.MAC) {
                exec {
                    workingDir("${project.projectDir}/../../bdk-ffi")
                    executable("cargo")
                    val cargoArgs: List<String> = listOf("build", "--profile", "release-smaller", "--target", "x86_64-apple-darwin")
                    args(cargoArgs)
                }
                exec {
                    workingDir("${project.projectDir}/../../bdk-ffi")
                    executable("cargo")
                    val cargoArgs: List<String> = listOf("build", "--profile", "release-smaller", "--target", "aarch64-apple-darwin")
                    args(cargoArgs)
                }
            } else if (operatingSystem == OS.LINUX) {
                exec {
                    workingDir("${project.projectDir}/../../bdk-ffi")
                    executable("cargo")
                    val cargoArgs: List<String> = listOf("build", "--profile", "release-smaller", "--target", "x86_64-unknown-linux-gnu")
                    args(cargoArgs)
                }
            } else if (operatingSystem == OS.WINDOWS) {
                exec {
                    workingDir("${project.projectDir}/../../bdk-ffi")
                    executable("cargo")
                    val cargoArgs: List<String> = listOf("build", "--profile", "release-smaller", "--target", "x86_64-pc-windows-msvc")
                    args(cargoArgs)
                }
            }
        }

        // move the native libs build by cargo from target/.../release/
        // to their place in the bdk-jvm library
        val moveNativeJvmLibs by tasks.register<DefaultTask>("moveNativeJvmLibs") {

            // dependsOn(buildJvmBinaryX86_64MacOS, buildJvmBinaryAarch64MacOS, buildJvmBinaryLinux)
            dependsOn(buildJvmBinaries)

            data class CopyMetadata(val targetDir: String, val resDir: String, val ext: String)
            val libsToCopy: MutableList<CopyMetadata> = mutableListOf()

            if (operatingSystem == OS.MAC) {
                libsToCopy.add(
                    CopyMetadata(
                        targetDir = "aarch64-apple-darwin",
                        resDir = "darwin-aarch64",
                        ext = "dylib"
                    )
                )
                libsToCopy.add(
                    CopyMetadata(
                        targetDir = "x86_64-apple-darwin",
                        resDir = "darwin-x86-64",
                        ext = "dylib"
                    )
                )
            } else if (operatingSystem == OS.LINUX) {
                libsToCopy.add(
                    CopyMetadata(
                        targetDir = "x86_64-unknown-linux-gnu",
                        resDir = "linux-x86-64",
                        ext = "so"
                    )
                )
            } else if (operatingSystem == OS.WINDOWS) {
                libsToCopy.add(
                    CopyMetadata(
                        targetDir = "x86_64-pc-windows-msvc",
                        resDir = "win32-x86-64",
                        ext = "dll"
                    )
                )
            }
            val libName = when (operatingSystem) {
                OS.WINDOWS -> "bdkffi"
                else       -> "libbdkffi"
            }

            libsToCopy.forEach {
                doFirst {
                    copy {
                        with(it) {
                            from("${project.projectDir}/../../bdk-ffi/target/${this.targetDir}/release-smaller/${libName}.${this.ext}")
                            into("${project.projectDir}/../../bdk-jvm/lib/src/main/resources/${this.resDir}/")
                        }
                    }
                }
            }
        }

        // generate the bindings using the bdk-ffi-bindgen tool created in the bdk-ffi submodule
        val generateJvmBindings by tasks.register<Exec>("generateJvmBindings") {

            dependsOn(moveNativeJvmLibs)

            // TODO 2: Is the Windows name the correct one?
            // TODO 3: This will not work on mac Intel (x86_64 architecture)
            val libraryPath = when (operatingSystem) {
                OS.LINUX   -> "./target/x86_64-unknown-linux-gnu/release-smaller/libbdkffi.so"
                OS.MAC     -> "./target/aarch64-apple-darwin/release-smaller/libbdkffi.dylib"
                OS.WINDOWS -> "./target/x86_64-pc-windows-msvc/release-smaller/bdkffi.dll"
                else       -> throw Exception("Unsupported OS")
            }

            workingDir("${project.projectDir}/../../bdk-ffi/")
            val cargoArgs: List<String> = listOf("run", "--bin", "uniffi-bindgen", "generate", "--library", libraryPath, "--language", "kotlin", "--out-dir", "../bdk-jvm/lib/src/main/kotlin/", "--no-format")

            // The code above was for the migration to uniffi 0.24.3 using the --library flag
            // The code below works with uniffi 0.23.0
            // workingDir("${project.projectDir}/../../bdk-ffi/")
            // val cargoArgs: List<String> = listOf("run", "--bin", "uniffi-bindgen", "generate", "src/bdk.udl", "--language", "kotlin", "--out-dir", "../bdk-jvm/lib/src/main/kotlin", "--no-format")
            executable("cargo")
            args(cargoArgs)

            doLast {
                println("JVM bindings file successfully created")
            }
        }

        // we need an aggregate task which will run the 3 required tasks to build the JVM libs in order
        // the task will also appear in the printout of the ./gradlew tasks task with a group and description
        tasks.register("buildJvmLib") {
            group = "Bitcoindevkit"
            description = "Aggregate task to build JVM library"

            dependsOn(
                buildJvmBinaries,
                moveNativeJvmLibs,
                generateJvmBindings
            )
        }
    }
}
