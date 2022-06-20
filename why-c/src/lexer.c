#include <ctype.h>
#include <string.h>

#include "lexer.h"
#include "token.h"
#include "utils.h"

Lexer *lexer_init(char *src) {
    Lexer *lexer = calloc(1, sizeof(Lexer));
    validate_ptr(lexer);

    lexer->src = src;
    lexer->src_len = strlen(src);
    lexer->i = 0;
    lexer->c = src[lexer->i];

    return lexer;
}

void lexer_next(Lexer *lexer) {
    if (lexer->i < lexer->src_len && lexer->c != '\0') {
        lexer->c = lexer->src[++lexer->i];
    }
}

void lexer_skip_whitespace(Lexer *lexer) {
    while (is_whitespace(lexer->c)) {
        lexer_next(lexer);
    }
}

Token *lexer_next_with(Lexer *lexer, Token *token) {
    lexer_next(lexer);
    return token;
}

Token *lexer_lex_next(Lexer *lexer) {
    while (lexer->c != '\0') {
        if (lexer->c == '#') {
            lexer_next(lexer);
            return lexer_next_with(lexer, lexer_parse_ident(lexer));
        }
    }

    return token_init(NULL, TOKEN_EOF);
}

char lexer_peek(Lexer *lexer, int offset) {
    return lexer->src[MIN(lexer->i + offset, lexer->src_len)];
}

Token *lexer_parse_ident(Lexer *lexer) {
    char *ident = calloc(1, sizeof(char));

    while (isalnum(lexer->c)) {
        ident = realloc(ident, (strlen(ident) + 2) * sizeof(char));
        strcat(ident, (char[]){lexer->c, 0});
        lexer_next(lexer);
    }

    switch (lexer->c) {
        case '=': {
            if (lexer_peek(lexer, 1) == '>') {
                return lexer_next_with(
                    lexer, token_init("=>", TOKEN_LARGE_R_ARROW)
                );

                return lexer_next_with(lexer, token_init("=", TOKEN_EQ));
            }

            break;
        }

        // case ':': {
        //     lexer_skip_whitespace(lexer);
        //     return lexer_next_with(lexer, lexer_parse_typehint(lexer));
        // }
    }

    return token_init(ident, TOKEN_IDENT);
}
