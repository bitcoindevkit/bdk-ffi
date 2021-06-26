#include <assert.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include "bdk_ffi.h"

int main (int argc, char const * const argv[])
{   
    // test new wallet error
    { 
        BlockchainConfig_t *bc_config = new_electrum_config("ssl://electrum.blockstream.info:60002", NULL, 5, 30);
        //DatabaseConfig_t *db_config = new_sled_config("/home/steve/.bdk", "test_wallet");
        DatabaseConfig_t *db_config = new_memory_config();
        
        WalletResult_t *wallet_result = new_wallet_result("bad", "bad", bc_config, db_config);
        assert(wallet_result != NULL);
        free_blockchain_config(bc_config);
        free_database_config(db_config);
        char *wallet_err = get_wallet_err(wallet_result);
        assert(wallet_err != NULL);
        assert( 0 == strcmp(wallet_err,"Descriptor") );
        //printf("wallet err: %s\n", wallet_err);   
        WalletRef_t *wallet_ref = get_wallet_ok(wallet_result);
        assert(wallet_ref == NULL);
        free_string(wallet_err);
        free_wallet_result(wallet_result);

    }
    
    // test new wallet
    {
        char const *desc = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)";
        char const *change = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)";
    
        BlockchainConfig_t *bc_config = new_electrum_config("ssl://electrum.blockstream.info:60002", NULL, 5, 30);
        //DatabaseConfig_t *db_config = new_sled_config("/home/steve/.bdk", "test_wallet");
        DatabaseConfig_t *db_config = new_memory_config();
    
        WalletResult_t *wallet_result = new_wallet_result(desc, change, bc_config, db_config);
        assert(wallet_result != NULL);
        free_blockchain_config(bc_config);
        free_database_config(db_config);
        char *wallet_err = get_wallet_err(wallet_result);
        assert(wallet_err == NULL);
        WalletRef_t *wallet_ref = get_wallet_ok(wallet_result);
        assert(wallet_ref != NULL);
        
        // test sync_wallet
        VoidResult_t *sync_result = sync_wallet(wallet_ref);    
        free_void_result(sync_result);    
        
        // test new_address
        StringResult_t *address_result1 = new_address(wallet_ref);
        char *address1 = get_string_ok(address_result1);
        //printf("address1: %s\n", address1);       
        assert( 0 == strcmp(address1,"tb1qgkhp034fyxeta00h0nne9tzfm0vsxq4prduzxp"));
        free_string(address1);
        free_string_result(address_result1);
        
        StringResult_t *address_result2 = new_address(wallet_ref);
        char *address2 = get_string_ok(address_result2);
        //printf("address2: %s\n", address2);
        assert(0 == strcmp(address2,"tb1qd6u9q327sru2ljvwzdtfrdg36sapax7udz97wf"));
        free_string(address2);
        free_string_result(address_result2);
        
        free_wallet_ref(wallet_ref);
        
        // test free_wallet
        free_wallet_result(wallet_result);
        
        // test free_wallet NULL doesn't crash
        free_wallet_result(NULL);
        
        // verify free_wallet after free_wallet fails (core dumped)
        ////free_wallet_result(wallet_result);
        
        // verify sync_wallet after free_wallet fails (core dumped)
        ////VoidResult_t sync_result2 = sync_wallet(wallet_result);   

    }
        
    return EXIT_SUCCESS;
}
