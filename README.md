## Build Commands

Compile builder

    gcc builder/build.c -o build

Then build the application using

    Apple Silicon: 
        
        ./build applesilicon

    Apple Intel:
        
        ./build applex86

    Linux:
        
        ./build linux

Clean previous build

    ./build clean