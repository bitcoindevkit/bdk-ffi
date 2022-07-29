plugins {
    id("java-gradle-plugin")
    `kotlin-dsl`
}

gradlePlugin {
    plugins {
        create("uniFfiJvmBindings") {
            id = "org.bitcoindevkit.plugins.generate-jvm-bindings"
            implementationClass = "org.bitcoindevkit.plugins.UniFfiJvmPlugin"
        }
        create("uniFfiAndroidBindings") {
            id = "org.bitcoindevkit.plugins.generate-android-bindings"
            implementationClass = "org.bitcoindevkit.plugins.UniFfiAndroidPlugin"
        }
    }
}
