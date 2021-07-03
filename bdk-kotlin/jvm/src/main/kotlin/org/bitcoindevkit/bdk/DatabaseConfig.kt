package org.bitcoindevkit.bdk

import org.slf4j.Logger
import org.slf4j.LoggerFactory

abstract class DatabaseConfig() : LibBase() {
    private val log: Logger = LoggerFactory.getLogger(DatabaseConfig::class.java)
    abstract val databaseConfigT: LibJna.DatabaseConfig_t

    protected fun finalize() {
        libJna.free_database_config(databaseConfigT)
        log.debug("$databaseConfigT freed")
    }
}

class MemoryConfig() : DatabaseConfig() {

    private val log: Logger = LoggerFactory.getLogger(MemoryConfig::class.java)
    override val databaseConfigT = libJna.new_memory_config()
}

class SledConfig(path: String, treeName: String) : DatabaseConfig() {

    private val log: Logger = LoggerFactory.getLogger(SledConfig::class.java)
    override val databaseConfigT = libJna.new_sled_config(path, treeName)
}