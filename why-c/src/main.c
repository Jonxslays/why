#include "utils.h"
#include "why.h"
#include <stdio.h>
#include <stdlib.h>


int main(int argc, char **argv) {
    if (argc < 2) {
        printf("Missing required argument: the file to compile.\n");
        exit(1);
    }

    long bytes;
    char *buffer;
    FILE *file;

    // Open the file
    file = fopen(argv[1], "r");
    validate_ptr(file);

    // See how big the file is
    fseek(file, 0L, SEEK_END);
    bytes = ftell(file);
    fseek(file, 0L, SEEK_SET);

    // Allocate the required memory
    buffer = calloc(bytes, sizeof(char));
    validate_ptr(buffer);

    // Read the file into the buffer
    fread(buffer, sizeof(char), bytes, file);
    fclose(file);

    // Compile the program
    why_compile(buffer);

    // Bye!
    free(buffer);
    return 0;
}
