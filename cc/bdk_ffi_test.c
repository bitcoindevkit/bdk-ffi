#include <assert.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include "bdk_ffi.h"

int main (int argc, char const * const argv[])
{   
    char const * name = "test_wallet";
    char const * desc = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)";
    char const * change = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)";
    
    // test new_wallet
    {
        WalletPtr_t * wallet = new_wallet(name, desc, change);
        assert(wallet != NULL);
        
        // test sync_wallet
        sync_wallet(wallet);        
        
        // test new_address
        char * address1 = new_address(wallet);
        //printf("address1: %s\n", address1);       
        assert( 0 == strcmp(address1,"tb1qgkhp034fyxeta00h0nne9tzfm0vsxq4prduzxp"));
        free_string(address1);
        
        char * address2 = new_address(wallet);
        //printf("address2: %s\n", address2);
        assert(0 == strcmp(address2,"tb1qd6u9q327sru2ljvwzdtfrdg36sapax7udz97wf"));
        free_string(address2);
        
        // test free_wallet
        free_wallet(wallet);
        
        // test free_wallet NULL doesn't crash
        free_wallet(NULL);
        
        // verify sync_wallet after sync_wallet fails (double free detected, core dumped)
        ////sync_wallet(&wallet);
    }
        
    return EXIT_SUCCESS;
}
