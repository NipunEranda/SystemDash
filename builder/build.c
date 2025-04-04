#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <dirent.h>
#include <sys/stat.h>
#include <stdbool.h>

#include "functions.h"

int main(int argc, char *argv[])
{
    if (argv[1])
    {
        if (strcmp(argv[1], "applesilicon") == 0 || strcmp(argv[1], "applex86") == 0 || strcmp(argv[1], "linux") == 0)
        {
            build(argv[1]);
        }
        else if (strcmp(argv[1], "clean") == 0)
        {
            clean(false);
        }
        else
        {
            printf("No matching parameter provided. Please specify 'applesilicon', 'applex86', 'linux', or 'clean'.");
        }
    }
    else
    {
        printf("No parameter provided. Please specify 'applesilicon', 'applex86', 'linux', or 'clean'.");
    }

    return 0;
}