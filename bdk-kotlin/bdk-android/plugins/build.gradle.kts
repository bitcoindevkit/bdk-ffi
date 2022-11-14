plugins {
    id("java-gradle-plugin")
    `kotlin-dsl`
}

gradlePlugin {
    plugins {
        create("uniFfiAndroidBindings") {
            id = "org.bitcoindevkit.plugins.generate-android-bindings"
            implementationClass = "org.bitcoindevkit.plugins.UniFfiAndroidPlugin"
        }
    }
}
