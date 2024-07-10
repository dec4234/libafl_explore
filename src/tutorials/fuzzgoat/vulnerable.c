#include <stdio.h>
#include "string.h"

int main(int argc, char** argv) {
    printf("\n\nStarting\n\n");

    if(argc != 2) {
        return -1;
    }

    // Note that argv[0] is ./testing
    if(strcmp("testing", argv[1]) == 0) {
        printf("\n\nDetected\n\n");
    }

    return 0;
}