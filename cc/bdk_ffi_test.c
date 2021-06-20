#include <assert.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include "bdk_ffi.h"

int main (int argc, char const * const argv[])
{   
    // test new wallet error
    { 
        WalletResult_t *wallet_result = new_wallet_result("bad", "bad", NULL);
        assert(wallet_result != NULL);
        char *wallet_error = get_wallet_err(wallet_result);
        assert(wallet_error != NULL);
        //printf("wallet error: %s\n", wallet_error);         
        free_string(wallet_error);
        free_wallet_result(wallet_result);
    }
    
    // test new wallet
    {
        char const *name = "test_wallet";
        char const *desc = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)";
        char const *change = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)";
    
        WalletResult_t *wallet_result = new_wallet_result(name, desc, change);
        assert(wallet_result != NULL);
        
        // test sync_wallet
        VoidResult_t *sync_result = sync_wallet(wallet_result);    
        free_void_result(sync_result);    
        
        // test new_address
        StringResult_t *address_result1 = new_address(wallet_result);
        char *address1 = get_string_ok(address_result1);
        //printf("address1: %s\n", address1);       
        assert( 0 == strcmp(address1,"tb1qgkhp034fyxeta00h0nne9tzfm0vsxq4prduzxp"));
        free_string(address1);
        free_string_result(address_result1);
        
        StringResult_t *address_result2 = new_address(wallet_result);
        char *address2 = get_string_ok(address_result2);
        //printf("address2: %s\n", address2);
        assert(0 == strcmp(address2,"tb1qd6u9q327sru2ljvwzdtfrdg36sapax7udz97wf"));
        free_string(address2);
        free_string_result(address_result2);
        
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
