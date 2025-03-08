#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char *get_name() { return "TestExtension"; }

char *get_ui() {
  FILE *file;
  char *buffer;
  long file_size;

  file = fopen("test.json", "r");

  if (file == NULL) {
    return "";
  }

  fseek(file, 0, SEEK_END);
  file_size = ftell(file);
  rewind(file);

  buffer = (char *)malloc(file_size + 1);
  if (buffer == NULL) {
    fclose(file);
    return "";
  }

  size_t result = fread(buffer, 1, file_size, file);
  if (result != file_size) {
    free(buffer);
    fclose(file);
    return "";
  }

  buffer[file_size] = '\0';

  fclose(file);
  return buffer;
}
