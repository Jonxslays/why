#include "why.h"
#include "lexer.h"
#include "token.h"
#include "utils.h"
#include <string.h>
#include <stdlib.h>


void why_compile(char *src) {
    Lexer *lexer = lexer_init(src);
    Token *final_token = lexer_lex_next(lexer);
    // debug_print_token(eof_token);

    if (final_token->type != TOKEN_EOF) {
        Token *eof_token = token_init(NULL, TOKEN_EOF);

        char received[(strlen(final_token->value) + 256) * sizeof(char)];
        char expected[256 * sizeof(char)];

        token_to_string(received, final_token);
        token_to_string(expected, eof_token);

        fprintf(stderr, "Expected EOF: %s, but got: %s\n", expected, received);
    }

    free(lexer);
}
