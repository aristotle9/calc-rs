#include<stdio.h>
#include<stdint.h>

void hello_world();
void* createI32Vec();
int32_t pushI32Vec(void*, int32_t);
void dropI32Vec(void**);//会把指针归零

int main() {
    hello_world();
    void* vec = createI32Vec();
    printf("%x\n", vec);
    pushI32Vec(NULL, 1);//return 0
    pushI32Vec(vec, 2);
    pushI32Vec(vec, 3);
    printf("%x\n", vec);
    dropI32Vec(&vec);
    printf("%x\n", vec);
    return 0;
}