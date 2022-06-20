#include "lexer.h"
#include "token.h"
#include "utils.h"
#include <ctype.h>
#include <string.h>


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

void lexer_skip_comment(Lexer *lexer, short int *multiline) {
    char next = lexer_peek(lexer, 1);

    if (*multiline) {
        // If its multiline we need to find the */ to end the comment
        while (lexer->c != '*') {
            lexer_next(lexer);
            next = lexer_peek(lexer, 1);
        }

        if (next == '/') {
            // Skip forward 2 chars and set multiline to false
            lexer_next(lexer);
            lexer_next(lexer);
            *multiline = 0;
        } else if (next == '\0') {
            // We started a comment and never closed it
            fprintf(stderr, "Error: Multiline comment was never closed\n");
            free(lexer);
            exit(1);
        } else {
            // There was * but no / afterwards
            lexer_next(lexer);
            lexer_skip_comment(lexer, multiline);
        }

    } else if (next == '/') {
        // Single line comment, ignore the rest of the line
        while (!is_newline(lexer->c)) {
            lexer_next(lexer);
        }

        lexer_next(lexer);

    } else if (next == '*') {
        // This is a multi line comment, recurse
        lexer_next(lexer);
        lexer_next(lexer);
        *multiline = 1;

        lexer_skip_comment(lexer, multiline);

    } else {
        // This is a straight up invalid token
        fprintf(stderr, "Invalid token at character %d\n", lexer->i);
        free(lexer);
        exit(1);
    }
}

Token *lexer_next_with(Lexer *lexer, Token *token) {
    lexer_next(lexer);
    return token;
}

Token *lexer_parse_assignment_expr(Lexer *lexer) {
    if (lexer_peek(lexer, 1) == '>') {
        return lexer_next_with(
            lexer, token_init("=>", TOKEN_LARGE_R_ARROW)
        );
    }

    if (lexer_peek(lexer, 1) == '=') {
        // This is an EQEQ
        return lexer_next_with(lexer, token_init("==", TOKEN_EQEQ));
    }

    return lexer_next_with(lexer, token_init("=", TOKEN_EQ));
}

// Token *lexer_expect(Lexer *lexer, int type) {

//     if (!== type) {
//         fprintf("Expected token: %s, but got %s\n");
//         exit(1);
//     }
// }

Token *lexer_lex_next(Lexer *lexer) {
    short int multiline_comment = 0;

    while (lexer->c != '\0') {
        if (lexer->c == '#') {
            lexer_next_with(lexer, token_init("#", TOKEN_HASH));
            lexer_next_with(lexer, lexer_parse_ident(lexer));
            continue;
        }

        if (lexer->c == '=') {
            lexer_next_with(lexer, lexer_parse_assignment_expr(lexer));
            continue;
        }

        if (lexer->c != ';' && isalnum(lexer->c)) {
            lexer_next_with(lexer, lexer_parse_ident(lexer));
            continue;
        }

        if (lexer->c == ';') {
            lexer_next_with(lexer, token_init(";", TOKEN_SEMI));
            continue;
        }

        if (lexer->c == '/' || lexer->c == '*') {
            if (lexer->c == '*' && !multiline_comment) {
                fprintf(stderr, "Invalid token at character %d\n", lexer->i);
                free(lexer);
                exit(1);
            }

            lexer_skip_comment(lexer, &multiline_comment);
        }

        lexer_skip_whitespace(lexer);
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

    return token_init(ident, TOKEN_IDENT);
}
