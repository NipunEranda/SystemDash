#include "functions.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <dirent.h>
#include <sys/stat.h>
#include <stdbool.h>

const char *prev_build = "dist";
const char *rust_build = "target";
const char *node_modules = "./frontend/node_modules";
const char *node_build = "./frontend/dist";

const char *apple_arm = "applearm";
const char *apple_x86 = "applex86";
const char *linx = "linux";
const char *windows = "windows";

int remove_directory(const char *path)
{
    DIR *dir = opendir(path); // Open the directory
    struct dirent *entry;
    char full_path[1024]; // To store the full path of files/subdirectories

    if (dir == NULL)
    {
        return -1;
    }

    // Loop over the directory contents
    while ((entry = readdir(dir)) != NULL)
    {
        // Skip the "." and ".." entries
        if (strcmp(entry->d_name, ".") == 0 || strcmp(entry->d_name, "..") == 0)
        {
            continue;
        }

        // Build the full path for each item inside the directory
        snprintf(full_path, sizeof(full_path), "%s/%s", path, entry->d_name);

        // If it's a directory, recursively call remove_directory to remove its contents
        if (entry->d_type == DT_DIR)
        {
            remove_directory(full_path); // Recursively remove subdirectories
        }
        else
        {
            if (remove(full_path) != 0)
            { // Remove the file
                perror("Error removing file");
            }
        }
    }

    closedir(dir); // Close the directory

    // Remove the directory itself after it is empty
    if (rmdir(path) == 0)
    {
        printf("Directory '%s' removed successfully.\n", path);
        return 0;
    }
    else
    {
        perror("Error removing directory");
        return -1;
    }
}

// Function to copy a file from source to destination
int copy_file(const char *source, const char *destination)
{
    FILE *src = fopen(source, "rb");
    FILE *dest = fopen(destination, "wb");

    if (!src || !dest)
    {
        perror("Error opening files");
        return -1;
    }

    char buffer[1024];
    size_t n;

    while ((n = fread(buffer, 1, sizeof(buffer), src)) > 0)
    {
        fwrite(buffer, 1, n, dest);
    }

    fclose(src);
    fclose(dest);
    return 0;
}

// Function to copy the contents of a directory to a new location
int copy_directory(const char *source_dir, const char *destination_dir)
{
    DIR *dir = opendir(source_dir);
    if (dir == NULL)
    {
        perror("Error opening directory");
        return -1;
    }

    struct dirent *entry;
    char source_path[1024];
    char dest_path[1024];

    // Create the destination directory
    if (mkdir(destination_dir, 0755) != 0)
    {
        perror("Error creating directory");
        closedir(dir);
        return -1;
    }

    while ((entry = readdir(dir)) != NULL)
    {
        // Skip "." and ".."
        if (strcmp(entry->d_name, ".") == 0 || strcmp(entry->d_name, "..") == 0)
        {
            continue;
        }

        // Build full source and destination paths
        snprintf(source_path, sizeof(source_path), "%s/%s", source_dir, entry->d_name);
        snprintf(dest_path, sizeof(dest_path), "%s/%s", destination_dir, entry->d_name);

        // Check if it's a directory or file and act accordingly
        if (entry->d_type == DT_DIR)
        {
            // Recursively copy subdirectories
            if (copy_directory(source_path, dest_path) != 0)
            {
                closedir(dir);
                return -1;
            }
        }
        else if (entry->d_type == DT_REG)
        {
            // Copy regular files
            if (copy_file(source_path, dest_path) != 0)
            {
                closedir(dir);
                return -1;
            }
        }
    }

    closedir(dir);
    return 0;
}

// Function to move a directory
int move_directory(const char *source_dir, const char *destination_dir)
{
    // Step 1: Copy the contents from source to destination
    if (copy_directory(source_dir, destination_dir) != 0)
    {
        printf("Error copying directory contents.\n");
        return -1;
    }

    // Step 2: Delete the source directory and its contents
    if (remove_directory(source_dir) != 0)
    {
        printf("Error deleting source directory.\n");
        return -1;
    }

    printf("Directory moved successfully.\n");
    return 0;
}

// Function to move a file
int move_file(const char *source, const char *destination)
{
    // Try to rename (move) the file
    if (rename(source, destination) != 0)
    {
        perror("Error moving file");
        return -1; // Return -1 on failure
    }
    printf("File moved successfully.\n");
    return 0; // Return 0 on success
}

void clean(bool keep_build)
{
    printf("Cleaning up...\n");

    if (keep_build == false)
        remove_directory(prev_build);
    remove_directory(rust_build);
    remove_directory(node_modules);
    remove_directory(node_build);
}

void build_frontend()
{
    system("mkdir dist");
    system("cd frontend && npm i && npm run build");
    move_directory("./frontend/dist", "./dist/static");
}

char *get_target(const char *systemType)
{
    if (strcmp(systemType, apple_arm) == 0)
    {
        return "aarch64-apple-darwin";
    }
    else if (strcmp(systemType, apple_x86) == 0)
    {
        return "x86_64-apple-darwin";
    }
    else if (strcmp(systemType, linx) == 0)
    {
        return "x86_64-unknown-linux-gnu";
    }
    else if (strcmp(systemType, windows) == 0)
    {
        return "x86_64-pc-windows-gnu";
    }
    else
    {
        return "0";
    }
}

char *get_executable_name(const char *systemType, char *name)
{
    char *executable_name = malloc(100 * sizeof(char));
    if (executable_name == NULL)
    {
        perror("Error allocating memory");
        return NULL;
    }

    if ((strcmp(systemType, apple_arm) == 0) || (strcmp(systemType, apple_x86) == 0) || (strcmp(systemType, linx) == 0))
    {
        sprintf(executable_name, "%s", name);
    }
    else
    {
        sprintf(executable_name, "%s.exe", name);
    }

    return executable_name;
}

void build(const char *systemType)
{
    int status;
    char *target = "";
    if ((strcmp(systemType, apple_arm) == 0) || (strcmp(systemType, apple_x86) == 0) || (strcmp(systemType, linx) == 0))
    {
        char build_command[100];
        sprintf(build_command, "rustup target add %s && cargo build --release --target %s", get_target(systemType), get_target(systemType));

        status = system(build_command);
    }
    else
    {
        char build_command[100];
        sprintf(build_command, "rustup target add %s && cargo build --release --target %s", get_target(systemType), get_target(systemType));

        status = system(build_command);
        status = 0;
    }

    if (status == -1)
    {
        perror("Error executing command");
    }
    else
    {
        char move_command_source[100];
        char move_command_destination[100];
        char *executable_name = get_executable_name(systemType, "SystemDash");
        if (executable_name == NULL)
        {
            fprintf(stderr, "Failed to get executable name.\n");
            return;
        }

        sprintf(move_command_source, "target/%s/release/%s", get_target(systemType), executable_name);
        sprintf(move_command_destination, "./dist/%s", executable_name);

        build_frontend();
        move_file(move_command_source, move_command_destination);
        clean(true);
        free(executable_name); // Free the allocated memory
        printf("\nBuild completed successfully.\n");
    }
}
