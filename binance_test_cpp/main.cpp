#include <stdio.h>
#include <dlfcn.h>


// Globals
typedef char* (*FUNC_RustFromCpp)(const char*);
typedef char* (*FUNC_CppFromRust)(const char*);
typedef int (*FUNC_InitFromCpp)(FUNC_CppFromRust);

FUNC_RustFromCpp gFunc_rustFromCpp = NULL;
FUNC_InitFromCpp gFunc_initFromCpp = NULL;
void* gHandler_LibBinance = NULL;

char* cppFromRust(const char* s) {
    printf("Received String From Rust : %s \n", s);
    return NULL;
}

int initRust() {
    gHandler_LibBinance = dlopen("./libbinance.so", RTLD_NOW);
    if (gHandler_LibBinance) {
        gFunc_initFromCpp = (FUNC_InitFromCpp) dlsym(gHandler_LibBinance, "initFromCpp");
        if (gFunc_initFromCpp) {
            printf("Success to load (%s)\n", "initFromCpp");
        } else {
            printf("Error : symbol not found (%s) \n", "initFromCpp");
            return -1;
        }

        gFunc_rustFromCpp = (FUNC_RustFromCpp) dlsym(gHandler_LibBinance, "rustFromCpp");
        if (gFunc_rustFromCpp) {
            printf("Success to load (%s)\n", "rustFromCpp");

        } else {
            printf("Error: symbol not found (%s)\n", "rustFromCpp");
            return -1;
        }
    } else {
        printf("Error: failed to load library\n");
        return -1;
    }
    return 0;
}

int exitRust() {
    dlclose(gHandler_LibBinance);
    return 0;
}

int main() {
    if (initRust()) {
        printf("Fail to init rust library\n");
        return -1;
    }

    gFunc_initFromCpp(cppFromRust);
    char* result = NULL;
    result = gFunc_rustFromCpp("new_order");
    result = gFunc_rustFromCpp("exchange_info");

    exitRust();
}

