#ifndef _WHY_TOKEN_H
#define _WHY_TOKEN_H

typedef struct {
    enum {
        TOKEN_IDENT,
        TOKEN_STRING,
        TOKEN_INT,
        TOKEN_UINT,
        TOKEN_FLOAT,
        TOKEN_UFLOAT,
        TOKEN_EQ,
        TOKEN_LPAREN,
        TOKEN_RPAREN,
        TOKEN_HASH,
        TOKEN_DOLLAR,
        TOKEN_COLON,
        TOKEN_COMMA,
        TOKEN_SEMI,
        TOKEN_AT,
        TOKEN_SMALL_R_ARROW,
        TOKEN_LARGE_R_ARROW,
        TOKEN_LT,
        TOKEN_GT,
        TOKEN_LTE,
        TOKEN_GTE,
        TOKEN_NE,
        TOKEN_EQEQ,
        TOKEN_KEYWORD,
        TOKEN_LBRACE,
        TOKEN_RBRACE,
        TOKEN_EXCLAMATION,
        TOKEN_LBRACKET,
        TOKEN_RBRACKET,
        TOKEN_BAR,
        TOKEN_QUESTION_MARK,
        TOKEN_PERCENT,
        TOKEN_TYPEHINT,
        TOKEN_EOF,
    } type;
    char *value;
} Token;

Token *token_init(char *value, int type);

#endif
