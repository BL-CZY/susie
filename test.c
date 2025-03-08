#include <stdio.h>
#include <stdlib.h>

char *get_ui() {
  FILE *fptr;

  fptr = fopen("test.json", "r");

  if (!fptr) {
    return "";
  }

  char *content = malloc(100);

  fgets(content, 100, fptr);

  fclose(fptr);

  fptr = 0;

  return content;
}
