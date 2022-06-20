#include <stdlib.h>

#include "token.h"
#include "utils.h"

Token *token_init(char *value, int type) {
    Token *token = calloc(1, sizeof(Token));
    validate_ptr(token);

    token->value = value;
    token->type = type;

    debug_print_token(token);
    return token;
}
