#include <assert.h>
#include <stdlib.h>
#include <stdio.h>
#include "bdk_ffi.h"

int main (int argc, char const * const argv[])
{
    // test print_string
    print_string("hello 123");
    
    // test concat_string
    char const * string1 = "string1";
    char const * string2 = "string2";
    char * string3 = concat_string(string1, string2);
    print_string(string3);
    free_string(string3);
    // verify free_string after free_string fails
    ////free_string(string3);
    
    // test print_config with c created config
    Config_t config1 = { .name = "test", .count = 101 };
    print_config(&config1);
    
    // test new_config
    Config_t * config2 = new_config("test test", 202);
    print_config(config2);
    
    // test free_config
    free_config(config2);
    // verify print_config after free_config fails (invalid data)
    ////print_config(config2);
    // verify free_config after free_config fails (double free detected, core dumped)
    ////free_config(config2);
    
    char const * name = "test_wallet";
    char const * desc = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)";
    char const * change = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)";
    //char const * change = NULL;
    
    // test new_wallet
    {
        WalletPtr_t * wallet = new_wallet(name, desc, change);
        
        // test sync_wallet
        sync_wallet(wallet);
        printf("after sync_wallet\n");    
        sync_wallet(wallet);
        printf("after sync_wallet\n");
        
        // test new_address
        char * address1 = new_address(wallet);
        printf("address1: %s\n", address1);
        free_string(address1);
        assert(address1 != NULL);
        char * address2 = new_address(wallet);
        printf("address2: %s\n", address2);
        assert(address2 != NULL);
        free_string(address2);
        
        // test free_wallet
        free_wallet(wallet);
        printf("after free_wallet\n");
        
        // test free_wallet NULL doesn't crash
        free_wallet(NULL);
        
        // verify sync_wallet after sync_wallet fails (double free detected, core dumped)
        ////sync_wallet(&wallet);
    }
        
    return EXIT_SUCCESS;
}
