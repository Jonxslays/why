#include "token.h"
#include "utils.h"
#include <stdio.h>
#include <stdlib.h>


void validate_ptr(void *ptr) {
    if (ptr == NULL) {
        printf("Received a null pointer, exiting...");
        exit(1);
    }
}

void debug_print_token(Token *token) {
    printf("Token(type: %d, value: %s)\n", token->type, token->value);
}

int is_whitespace(char c) {
    return c == 13 || c == 10 || c == ' ' || c == '\t';
}
