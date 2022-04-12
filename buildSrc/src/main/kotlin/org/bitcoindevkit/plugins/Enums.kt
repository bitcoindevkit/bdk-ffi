package org.bitcoindevkit.plugins


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
