#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <dirent.h>
#include <sys/stat.h>
#include <stdbool.h>

#include "functions.h"

const char* apple_silicon = "applesilicon";
const char* apple_x86 = "applex86";
const char* linx = "linux";
const char* windows = "windows";

int main(int argc, char *argv[])
{
    if (argv[1])
    {
        if (strcmp(argv[1], apple_silicon) == 0 || strcmp(argv[1], apple_x86) == 0 || strcmp(argv[1], linx) == 0 || strcmp(argv[1], windows) == 0)
        {
            build(argv[1]);
        }
        else if (strcmp(argv[1], "clean") == 0)
        {
            clean(false);
        }
        else
        {
            printf("No matching parameter provided. Please specify 'applesilicon', 'applex86', 'linux', windows, or 'clean'.");
        }
    }
    else
    {
        printf("No parameter provided. Please specify 'applesilicon', 'applex86', 'linux', windows, or 'clean'.");
    }

    return 0;
}