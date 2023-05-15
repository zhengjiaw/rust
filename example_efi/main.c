#include <stdio.h>

extern int my_rust_function();
extern int my_function();

int main()
{
    int result = my_rust_function();
    printf("%d\n", result);

    int res = my_function();
    printf("%d\n", res);

    return 0;
}
