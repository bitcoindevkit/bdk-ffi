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
    }
}
