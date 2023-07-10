

#include <stdio.h>

extern "C"
void callApiEndpointFromRust(char* str) {
    printf("Received from Rust : %s\n", str);
}