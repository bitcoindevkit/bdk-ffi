#include <stdlib.h>
#include <stdio.h>
#include "bdk_ffi.h"

int main (int argc, char const * const argv[])
{

    char const * string1 = "string1";
    char const * string2 = "string2";
    char * string3 = concat_string(string1, string2);
    
    print_string(string3);
    print_string(string3);
    free_string(string3);
    //free_string(string3);

    //Point_t a = new_point(84,45);
    //Point_t b = new_point(0,39);
    //Point_t m = mid_point(a, b);
    //print_point(m);
    
    char const * name = "test_wallet";
    char const * desc = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)";
    char const * change = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)";
    
    //printf("wallet name: %s\n", name);
    //printf("descriptor: %s\n", desc);
    //printf("change descriptor: %s\n", change);
    WalletPtr_t * wallet = new_wallet(name, desc, change);
    
    sync_wallet(&wallet);    
    sync_wallet(&wallet);
    
    char const * address1 = new_address(&wallet);
    printf("address1: %s\n", address1);
    char const * address2 = new_address(&wallet);
    printf("address: %s\n", address2);
    
    //free_wallet(wallet);
    //sync_wallet(&wallet);
        
    return EXIT_SUCCESS;
}
