#include <stdio.h>
#include "string.h"

int main123(int argc, char** argv) {
    printf("\n\nStarting\n\n");

    if(argc < 3) {
        return -1;
    }

    char* filename = NULL;

    // Note that argv[0] is ./testing
    if(strcmp("testing", argv[1]) == 0) {
        filename = argv[2];
    } else {
        return 0; // nothing else to do
    }

    printf("\n\nReading file: %.25s\n\n", filename);

    FILE *file;

    if(fopen_s(&file, filename, "r") != 0) {
        printf("\n\nCould not open file\n\n");
        return -1;
    }

    return 0;
}

int test() {
    return 5;
}