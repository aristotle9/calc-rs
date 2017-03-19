#include<stdio.h>
#include<stdint.h>
#include<stdlib.h>
#include<string.h>

void hello_world();
void* createI32Vec();
int32_t pushI32Vec(void*, int32_t);
void dropI32Vec(void**);//会把指针归零

enum _CalcNodeType {
    Add = 0,
    Sub = 1,
    Mul = 2,
    Div = 3,
    Minus = 4,
    Num = 5
};

typedef enum _CalcNodeType CalcNodeType;
struct _CalcNode;
typedef struct _CalcNode CalcNode;

struct _CalcNode {
    CalcNodeType node_type;
    CalcNode* left;
    CalcNode* right;
    const char* str_val;
};

int eval_node(CalcNode* node);
void parseCalcNode(const char* p_str, int32_t str_len, int32_t* p_code, CalcNode** p_node, char** p_err);
void freeCalcNode(CalcNode**);

int main() {
    // hello_world();
    // void* vec = createI32Vec();
    // printf("%x\n", vec);
    // pushI32Vec(NULL, 1);//return 0
    // pushI32Vec(vec, 2);
    // pushI32Vec(vec, 3);
    // printf("%x\n", vec);
    // dropI32Vec(&vec);
    // printf("%x\n", vec);

    printf("%d\n", sizeof(CalcNodeType));

    const char* expr_str = "(1 + 2/) * 3";
    int expr_len = strlen(expr_str);
    int code = 0;
    char* err = NULL;
    CalcNode* pnode = NULL;
    parseCalcNode(expr_str, expr_len, &code, &pnode, &err);
    if (code == 0) { //ok
        printf("eval: %d\n", eval_node(pnode));
        freeCalcNode(&pnode);
    } else {
        printf("%s\n", err);
        free(err);
    }
    return 0;
    }

int eval_node(CalcNode* node) {
    CalcNodeType t = node->node_type;
    int left = 0;
    int right = 0;
    if (node->left != NULL) {
        left = eval_node(node->left);
    }
    if (node->right != NULL) {
        right = eval_node(node->right);
    }
    switch(t) {
        case Add:
            return left + right;
        case Sub:
            return left - right;
        case Mul:
            return left * right;
        case Div:
            if (right == 0) {
                printf("div by 0\n");
                return 0;
            }
            return left / right;
        case Minus:
            return - left;
        case Num:
            return atoi(node->str_val);
    }
    return 0;
}