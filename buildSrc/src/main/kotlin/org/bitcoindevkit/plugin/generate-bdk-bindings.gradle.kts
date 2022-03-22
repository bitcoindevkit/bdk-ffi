package org.bitcoindevkit.plugin

val operatingSystem: OS = when {
    System.getProperty("os.name").contains("mac", ignoreCase = true) -> OS.MAC
    System.getProperty("os.name").contains("linux", ignoreCase = true) -> OS.LINUX
    else -> OS.OTHER
}
val architecture: Arch = when (System.getProperty("os.arch")) {
    "x86_64" -> Arch.X86_64
    "aarch64" -> Arch.AARCH64
    else -> Arch.OTHER
}

tasks.register<Exec>("buildJvmBinaries") {
    group = "Bitcoindevkit"
    description = "Build the JVM native binaries for the bitcoindevkit"

    workingDir("${project.projectDir}/../bdk-ffi")
    val cargoArgs: MutableList<String> = mutableListOf("build", "--release", "--target")

    if (operatingSystem == OS.MAC && architecture == Arch.X86_64) {
        println("Building the JVM native libs for macOS x86_64")
        cargoArgs.add("x86_64-apple-darwin")
    } else if (operatingSystem == OS.MAC && architecture == Arch.AARCH64) {
        println("Building the JVM native libs for macOS x86_64")
        cargoArgs.add("aarch64-apple-darwin")
    } else if (operatingSystem == OS.LINUX) {
        println("Building the JVM native libs for Linux x86_64")
        cargoArgs.add("x86_64-unknown-linux-gnu")
    }

    executable("cargo")
    args(cargoArgs)
}

tasks.register<Copy>("moveNativeLibs") {
    group = "Bitcoindevkit"
    description = "Move the native libraries in the bdk-jvm project"

    var targetDir = ""
    var resDir = ""
    if (operatingSystem == OS.MAC && architecture == Arch.X86_64) {
        targetDir = "x86_64-apple-darwin"
        resDir = "darwin-x86-64"
    } else if (operatingSystem == OS.MAC && architecture == Arch.AARCH64) {
        targetDir = "aarch64-apple-darwin"
        resDir = "darwin-aarch64"
    } else if (operatingSystem == OS.LINUX) {
        targetDir = "x86_64-unknown-linux-gnu"
        resDir = "linux-x86-64"
    }

    from("${project.projectDir}/../bdk-ffi/target/$targetDir/release/libbdkffi.dylib")
    into("${project.projectDir}/../jvm/src/main/resources/$resDir/")
}

tasks.register<Exec>("generateBindings") {
    group = "Bitcoindevkit"
    description = "Building the bindings file for the bitcoindevkit"

    workingDir("${project.projectDir}/../bdk-ffi")
    executable("uniffi-bindgen")
    args("generate", "./src/bdk.udl", "--no-format", "--out-dir", "../jvm/src/main/kotlin", "--language", "kotlin")
}

enum class Arch {
    AARCH64,
    X86_64,
    OTHER,
}

enum class OS {
    MAC,
    LINUX,
    OTHER,
}
