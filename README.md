## Build Commands

Compile builder

    gcc builder/build.c -o build

Then build the application using

    Apple Silicon: 
        
        ./builder applesilicon

    Apple Intel:
        
        ./builder applex86

    Linux:
        
        ./builder linux

Clean previous build

    ./build clean