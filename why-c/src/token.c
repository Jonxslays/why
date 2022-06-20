#include "token.h"
#include "utils.h"
#include <stdlib.h>


Token *token_init(char *value, int type) {
    Token *token = calloc(1, sizeof(Token));
    validate_ptr(token);

    token->value = value;
    token->type = type;

    debug_print_token(token);
    return token;
}

char *token_to_string(char *buffer, Token *token) {
    sprintf(buffer, "Token(type: %d, value: %s)", token->type, token->value);
    return buffer;
}
