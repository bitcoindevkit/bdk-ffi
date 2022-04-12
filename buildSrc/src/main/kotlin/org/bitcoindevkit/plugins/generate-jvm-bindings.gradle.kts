package org.bitcoindevkit.plugins

// register a task of type Exec called buildJvmBinary
// which will run something like
// cargo build --release --target aarch64-apple-darwin
val buildJvmBinary by tasks.register<Exec>("buildJvmBinary") {

    workingDir("${project.projectDir}/../bdk-ffi")
    val cargoArgs: MutableList<String> = mutableListOf("build", "--release", "--target")

    if (operatingSystem == OS.MAC && architecture == Arch.X86_64) {
        cargoArgs.add("x86_64-apple-darwin")
    } else if (operatingSystem == OS.MAC && architecture == Arch.AARCH64) {
        cargoArgs.add("aarch64-apple-darwin")
    } else if (operatingSystem == OS.LINUX) {
        cargoArgs.add("x86_64-unknown-linux-gnu")
    }

    executable("cargo")
    args(cargoArgs)

    doLast {
        println("Native library for bdk-jvm on ${cargoArgs.last()} successfully built")
    }
}

// move the native libs build by cargo from bdk-ffi/target/.../release/
// to their place in the bdk-jvm library
val moveNativeJvmLib by tasks.register<Copy>("moveNativeJvmLib") {

    dependsOn(buildJvmBinary)

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

    doLast {
        println("$targetDir native binaries for JVM moved to ./jvm/src/main/resources/$resDir/")
    }
}

// generate the bindings using the bdk-ffi-bindgen tool
// created in the bdk-ffi submodule
val generateJvmBindings by tasks.register<Exec>("generateJvmBindings") {

    dependsOn(moveNativeJvmLib)

    workingDir("${project.projectDir}/../bdk-ffi")
    executable("cargo")
    args("run", "--package", "bdk-ffi-bindgen", "--", "--language", "kotlin", "--out-dir", "../jvm/src/main/kotlin")

    doLast {
        println("JVM bindings file successfully created")
    }
}

// create an aggregate task which will run the 3 required tasks to build the JVM libs in order
// the task will also appear in the printout of the ./gradlew tasks task with group and description
tasks.register("buildJvmLib") {
    group = "Bitcoindevkit"
    description = "Aggregate task to build JVM library"

    dependsOn(
        buildJvmBinary,
        moveNativeJvmLib,
        generateJvmBindings
    )
}
