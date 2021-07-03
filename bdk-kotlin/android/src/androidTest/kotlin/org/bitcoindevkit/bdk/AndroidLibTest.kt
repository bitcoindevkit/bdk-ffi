package org.bitcoindevkit.bdk

import android.app.Application
import android.content.Context.MODE_PRIVATE
import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.runner.RunWith

/**
 * Instrumented test, which will execute on an Android device.
 *
 * See [testing documentation](http://d.android.com/tools/testing).
 */
@RunWith(AndroidJUnit4::class)
class AndroidLibTest : LibTest() {
    override fun getTestDataDir(): String {
        val context = ApplicationProvider.getApplicationContext<Application>()
        return context.getDir("bdk-test", MODE_PRIVATE).toString()
    }

}
