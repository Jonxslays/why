#include "lexer.h"
#include "why.h"

void compile_why_source(char *src) {
    Lexer *lexer = lexer_init(src);
    free(lexer);
}
