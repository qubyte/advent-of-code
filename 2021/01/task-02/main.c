#include <stdio.h>
#include <stdlib.h>

int main(void) {
    char str[8];
    int increases = 0;
    unsigned long int depths[] = {0, 0, 0};
    int j;

    for (j = 0; fgets(str, sizeof str, stdin) != NULL; j++) {
        unsigned long int depth = strtoul(str, NULL, 10);

        if (j >= 3 && depth > depths[0]) {
            increases++;
        }

        depths[0] = depths[1];
        depths[1] = depths[2];
        depths[2] = depth;
    }

    printf("%d\n", increases);

    return 0;
}
