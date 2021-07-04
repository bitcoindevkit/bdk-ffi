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
        
        // new wallet with bad descriptor 
        FfiResult_OpaqueWallet_ptr_t wallet_result = new_wallet_result("bad","bad",bc_config,db_config);  
        assert(wallet_result.err == FFI_ERROR_DESCRIPTOR);
        assert(wallet_result.ok == NULL);
        
        free_blockchain_config(bc_config);
        free_database_config(db_config);  
        
        free_wallet_result(wallet_result);
    }
    
    // test new wallet
    {
        char const *desc = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)";
        char const *change = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)";
    
        BlockchainConfig_t *bc_config = new_electrum_config("ssl://electrum.blockstream.info:60002", NULL, 5, 30);
        DatabaseConfig_t *db_config = new_memory_config();
    
        // new wallet
        FfiResult_OpaqueWallet_ptr_t wallet_result = new_wallet_result(desc,change,bc_config,db_config);
        // printf("wallet_result.err = %d\n", wallet_result.err));
        assert(wallet_result.err == FFI_ERROR_NONE);    
        assert(wallet_result.ok != NULL);
        
        free_blockchain_config(bc_config);
        free_database_config(db_config);
        
        OpaqueWallet_t *wallet = wallet_result.ok;
        
        // sync wallet
        FfiResultVoid_t sync_result = sync_wallet(wallet);   
        assert(sync_result.err == FFI_ERROR_NONE);    
        free_void_result(sync_result);
        
        // new address
        FfiResult_char_ptr_t address1_result = new_address(wallet);
        assert(address1_result.ok != NULL);
        assert(address1_result.err == FFI_ERROR_NONE);
        // printf("address1 = %s\n", *address1_result.ok);
        assert( 0 == strcmp(address1_result.ok,"tb1qgkhp034fyxeta00h0nne9tzfm0vsxq4prduzxp"));
        free_string_result(address1_result);
        
        FfiResult_char_ptr_t address2_result = new_address(wallet);
        assert(address2_result.ok != NULL);
        assert(address2_result.err == FFI_ERROR_NONE);
        // printf("address2 = %s\n", *address2_result.ok);
        assert( 0 == strcmp(address2_result.ok,"tb1qd6u9q327sru2ljvwzdtfrdg36sapax7udz97wf"));
        free_string_result(address2_result);
        
        // free_wallet
        free_wallet_result(wallet_result);
        
        // verify free_wallet after free_wallet fails (core dumped)
        //// free_wallet_result(wallet_result);
        
        // verify sync_wallet after free_wallet fails (core dumped)
        //// FfiResultVoid_t sync_result2 = sync_wallet(wallet);    
    }
    
    // test get unspent utxos
    {
        char const *desc = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)";
        char const *change = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)";
    
        BlockchainConfig_t *bc_config = new_electrum_config("ssl://electrum.blockstream.info:60002", NULL, 5, 30);
        DatabaseConfig_t *db_config = new_memory_config();
    
        // new wallet
        FfiResult_OpaqueWallet_ptr_t wallet_result = new_wallet_result(desc,change,bc_config,db_config);
        assert(wallet_result.err == FFI_ERROR_NONE);    
        assert(wallet_result.ok != NULL);
        
        free_blockchain_config(bc_config);
        free_database_config(db_config);
        
        OpaqueWallet_t *wallet = wallet_result.ok;
        
        // sync wallet
        FfiResultVoid_t sync_result = sync_wallet(wallet);   
        assert(sync_result.err == FFI_ERROR_NONE);    
        free_void_result(sync_result);
        
        // list unspent
        FfiResult_Vec_LocalUtxo_t unspent_result = list_unspent(wallet);
        assert(unspent_result.ok.len == 7);
        assert(unspent_result.err == FFI_ERROR_NONE);
        
        LocalUtxo_t * unspent_ptr = unspent_result.ok.ptr;                       
        for (int i = 0; i < unspent_result.ok.len; i++) {            
            // printf("%d: outpoint.txid: %s\n", i, unspent_ptr[i].outpoint.txid);
            assert(unspent_ptr[i].outpoint.txid != NULL);
            // printf("%d: outpoint.vout: %d\n", i, unspent_ptr[i].outpoint.vout);
            assert(unspent_ptr[i].outpoint.vout >= 0);
            // printf("%d: txout.value: %ld\n", i, unspent_ptr[i].txout.value);
            assert(unspent_ptr[i].txout.value > 0);
            // printf("%d: txout.script_pubkey: %s\n", i, unspent_ptr[i].txout.script_pubkey);
            assert(unspent_ptr[i].txout.script_pubkey != NULL);
            // printf("%d: keychain: %d\n", i, unspent_ptr[i].keychain);
            assert(unspent_ptr[i].keychain >= 0);
        }
        
        free_veclocalutxo_result(unspent_result);          
        free_wallet_result(wallet_result);
    }
    
    // test balance 
    {
        char const *desc = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)";
        char const *change = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)";
    
        BlockchainConfig_t *bc_config = new_electrum_config("ssl://electrum.blockstream.info:60002", NULL, 5, 30);
        DatabaseConfig_t *db_config = new_memory_config();
    
        // new wallet
        FfiResult_OpaqueWallet_ptr_t wallet_result = new_wallet_result(desc,change,bc_config,db_config);
        assert(wallet_result.err == FFI_ERROR_NONE);    
        assert(wallet_result.ok != NULL);
        
        free_blockchain_config(bc_config);
        free_database_config(db_config);
        
        OpaqueWallet_t *wallet = wallet_result.ok;
        
        // sync wallet
        FfiResultVoid_t sync_result = sync_wallet(wallet);   
        assert(sync_result.err == FFI_ERROR_NONE);    
        free_void_result(sync_result);
        
        // get balance
        FfiResult_uint64_t balance_result = balance(wallet);
        //printf("balance.err = %d\n", (balance_result.err));
        assert(balance_result.err == FFI_ERROR_NONE);          
        //printf("balance.ok = %ld\n", balance_result.ok);                
        assert(balance_result.ok > 0);  
        
        // free balance and wallet results
        free_uint64_result(balance_result);
        free_wallet_result(wallet_result);
    }
        
    return EXIT_SUCCESS;
}
