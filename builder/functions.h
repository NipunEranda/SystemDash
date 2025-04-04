#ifndef FUNCTIONS_H
#define FUNCTIONS_H


#include <stdbool.h>

int remove_directory(const char *path);
int copy_file(const char *source, const char *destination);
int copy_directory(const char *source_dir, const char *destination_dir);
int move_directory(const char *source_dir, const char *destination_dir);
int move_file(const char *source, const char *destination);

void clean(bool keep_build);
void build_frontend();
char *get_target(const char *systemType);
char *get_executable_name(char *name);

void build(const char *systemType);

#endif  // FUNCTIONS_H