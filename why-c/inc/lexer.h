#ifndef _WHY_LEXER_H
#define _WHY_LEXER_H

#include <stdlib.h>

#include "token.h"

typedef struct {
    char *src;
    size_t src_len;
    char c;
    unsigned int i;
} Lexer;

Lexer *lexer_init(char *src);

char lexer_peek(Lexer *lexer, int offset);

void lexer_next(Lexer *lexer);

void lexer_skip_whitespace(Lexer *lexer);

Token *lexer_lex_next(Lexer *lexer);

Token *lexer_next_with(Lexer *lexer, Token *token);

Token *lexer_parse_ident(Lexer *lexer);

Token *lexer_parse_typehint(Lexer *lexer);

#endif
