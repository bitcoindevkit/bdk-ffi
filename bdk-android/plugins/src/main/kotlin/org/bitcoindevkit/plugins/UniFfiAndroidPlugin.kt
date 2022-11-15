package org.bitcoindevkit.plugins

import org.gradle.api.Plugin
import org.gradle.api.Project
import org.gradle.api.tasks.Copy
import org.gradle.api.tasks.Exec
import org.gradle.kotlin.dsl.environment
import org.gradle.kotlin.dsl.getValue
import org.gradle.kotlin.dsl.provideDelegate
import org.gradle.kotlin.dsl.register

internal class UniFfiAndroidPlugin : Plugin<Project> {
    override fun apply(target: Project): Unit = target.run {
        val llvmArchPath = when (operatingSystem) {
            OS.MAC -> "darwin-x86_64"
            OS.LINUX -> "linux-x86_64"
            OS.OTHER -> throw Error("Cannot build Android library from current architecture")
        }

        // arm64-v8a is the most popular hardware architecture for Android
        val buildAndroidAarch64Binary by tasks.register<Exec>("buildAndroidAarch64Binary") {

            workingDir("${projectDir}/../../bdk-ffi")
            val cargoArgs: MutableList<String> =
                mutableListOf("build", "--profile", "release-smaller", "--target", "aarch64-linux-android")

            executable("cargo")
            args(cargoArgs)

            // if ANDROID_NDK_ROOT is not set then set it to github actions default
            if (System.getenv("ANDROID_NDK_ROOT") == null) {
                environment(
                    Pair("ANDROID_NDK_ROOT", "${System.getenv("ANDROID_SDK_ROOT")}/ndk-bundle")
                )
            }

            environment(
                // add build toolchain to PATH
                Pair("PATH",
                    "${System.getenv("PATH")}:${System.getenv("ANDROID_NDK_ROOT")}/toolchains/llvm/prebuilt/$llvmArchPath/bin"),

                Pair("CFLAGS", "-D__ANDROID_API__=21"),
                Pair("CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER", "aarch64-linux-android21-clang"),
                Pair("CC", "aarch64-linux-android21-clang")
            )

            doLast {
                println("Native library for bdk-android on aarch64 built successfully")
            }
        }

        // the x86_64 version of the library is mostly used by emulators
        val buildAndroidX86_64Binary by tasks.register<Exec>("buildAndroidX86_64Binary") {

            workingDir("${project.projectDir}/../../bdk-ffi")
            val cargoArgs: MutableList<String> =
                mutableListOf("build", "--profile", "release-smaller", "--target", "x86_64-linux-android")

            executable("cargo")
            args(cargoArgs)

            // if ANDROID_NDK_ROOT is not set then set it to github actions default
            if (System.getenv("ANDROID_NDK_ROOT") == null) {
                environment(
                    Pair("ANDROID_NDK_ROOT", "${System.getenv("ANDROID_SDK_ROOT")}/ndk-bundle")
                )
            }

            environment(
                // add build toolchain to PATH
                Pair("PATH",
                    "${System.getenv("PATH")}:${System.getenv("ANDROID_NDK_ROOT")}/toolchains/llvm/prebuilt/$llvmArchPath/bin"),

                Pair("CFLAGS", "-D__ANDROID_API__=21"),
                Pair("CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER", "x86_64-linux-android21-clang"),
                Pair("CC", "x86_64-linux-android21-clang")
            )

            doLast {
                println("Native library for bdk-android on x86_64 built successfully")
            }
        }

        // armeabi-v7a version of the library for older 32-bit Android hardware
        val buildAndroidArmv7Binary by tasks.register<Exec>("buildAndroidArmv7Binary") {

            workingDir("${project.projectDir}/../../bdk-ffi")
            val cargoArgs: MutableList<String> =
                mutableListOf("build", "--profile", "release-smaller", "--target", "armv7-linux-androideabi")

            executable("cargo")
            args(cargoArgs)

            // if ANDROID_NDK_ROOT is not set then set it to github actions default
            if (System.getenv("ANDROID_NDK_ROOT") == null) {
                environment(
                    Pair("ANDROID_NDK_ROOT", "${System.getenv("ANDROID_SDK_ROOT")}/ndk-bundle")
                )
            }

            environment(
                // add build toolchain to PATH
                Pair("PATH",
                    "${System.getenv("PATH")}:${System.getenv("ANDROID_NDK_ROOT")}/toolchains/llvm/prebuilt/$llvmArchPath/bin"),

                Pair("CFLAGS", "-D__ANDROID_API__=21"),
                Pair("CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER",
                    "armv7a-linux-androideabi21-clang"),
                Pair("CC", "armv7a-linux-androideabi21-clang")
            )

            doLast {
                println("Native library for bdk-android on armv7 built successfully")
            }
        }

        // move the native libs build by cargo from target/<architecture>/release/
        // to their place in the bdk-android library
        // the task only copies the available binaries built using the buildAndroid<architecture>Binary tasks
        val moveNativeAndroidLibs by tasks.register<Copy>("moveNativeAndroidLibs") {

            dependsOn(buildAndroidAarch64Binary)

            into("${project.projectDir}/../lib/src/main/jniLibs/")

            into("arm64-v8a") {
                from("${project.projectDir}/../../target/aarch64-linux-android/release-smaller/libbdkffi.so")
            }

            into("x86_64") {
                from("${project.projectDir}/../../target/x86_64-linux-android/release-smaller/libbdkffi.so")
            }

            into("armeabi-v7a") {
                from("${project.projectDir}/../../target/armv7-linux-androideabi/release-smaller/libbdkffi.so")
            }

            doLast {
                println("Native binaries for Android moved to ./lib/src/main/jniLibs/")
            }
        }

        // generate the bindings using the bdk-ffi-bindgen tool located in the bdk-ffi submodule
        val generateAndroidBindings by tasks.register<Exec>("generateAndroidBindings") {
            dependsOn(moveNativeAndroidLibs)

            workingDir("${project.projectDir}/../../bdk-ffi")
            executable("cargo")
            args(
                "run",
                "--package",
                "bdk-ffi-bindgen",
                "--",
                "--language",
                "kotlin",
                "--out-dir",
                "../bdk-android/lib/src/main/kotlin"
            )

            doLast {
                println("Android bindings file successfully created")
            }
        }

        // create an aggregate task which will run the required tasks to build the Android libs in order
        // the task will also appear in the printout of the ./gradlew tasks task with group and description
        tasks.register("buildAndroidLib") {
            group = "Bitcoindevkit"
            description = "Aggregate task to build Android library"

            dependsOn(
                buildAndroidAarch64Binary,
                buildAndroidX86_64Binary,
                buildAndroidArmv7Binary,
                moveNativeAndroidLibs,
                generateAndroidBindings
            )
        }
    }
}
