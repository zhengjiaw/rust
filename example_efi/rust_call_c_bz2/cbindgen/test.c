// gcc -o test test.c -L target/release
#include "bindings.h"
#include <stdio.h>

int main() {
    const char* world = hello_world();
    printf("%s\n", world);

    const char* name = "OpenAI";
    const char* greeting = hello(name);
    printf("%s\n", greeting);

    // 释放在 Rust 中分配的内存
    free_str(greeting);

    return 0;
}
