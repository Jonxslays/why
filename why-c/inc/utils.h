#ifndef _WHY_UTILS_H
#define _WHY_UTILS_H

#include <stdio.h>
#include <stdlib.h>

#include "token.h"

#define MAX(a, b)\
    a > b ? a : b

#define MIN(a, b)\
    a < b ? a : b

void validate_ptr(void *ptr);

void debug_print_token(Token *token);

int is_whitespace(char c);

#endif
