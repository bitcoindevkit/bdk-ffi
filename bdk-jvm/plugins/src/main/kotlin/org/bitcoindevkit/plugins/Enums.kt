package org.bitcoindevkit.plugins


val operatingSystem: OS = when {
    System.getProperty("os.name").contains("mac", ignoreCase = true) -> OS.MAC
    System.getProperty("os.name").contains("linux", ignoreCase = true) -> OS.LINUX
    System.getProperty("os.name").contains("windows", ignoreCase = true) -> OS.WINDOWS
    else -> OS.OTHER
}

enum class OS {
    MAC,
    LINUX,
    WINDOWS,
    OTHER,
}
