#include "rust_fn.h"
#include<stdio.h>

int main() {
    char* str = "Hello, world!";
    printf("%s\n", str);

    call_from_c();
}