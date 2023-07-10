#include <stdio.h>
#include <dlfcn.h>

// extern "C" {
//     int callApiEndpointFromCpp(int x);
// }

int main() {
    void* lib = dlopen("./libbinance.so", RTLD_NOW);
    if (lib) {
        typedef int (*Func_callApiEndpointFromCpp)(const char*);
        Func_callApiEndpointFromCpp callApiEndpointFromCpp = (Func_callApiEndpointFromCpp) dlsym(lib, "callApiEndpointFromCpp");



        
        if (callApiEndpointFromCpp) {
            unsigned int result = callApiEndpointFromCpp("new_order");
            result = callApiEndpointFromCpp("exchange_info");
            printf("Result : %d\n", result);
        } else {
            printf("Error: symbol not found\n");
        }














        dlclose(lib);
    } else {
        printf("Error: failed to load library\n");
    }
}