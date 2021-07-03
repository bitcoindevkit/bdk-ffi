package org.bitcoindevkit.bdk

import java.nio.file.Paths

/**
 * Library test, which will execute on linux host.
 *
 */
class JvmLibTest : LibTest() {

    override fun getTestDataDir(): String {
        //return Files.createTempDirectory("bdk-test").toString()
        return Paths.get(System.getProperty("java.io.tmpdir"), "bdk-test").toString()
    }

}
