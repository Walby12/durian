#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <stdbool.h>

enum Tokens {
  ADD,
  SUB,
  PUSH,
  POP,
  INT
};

struct Durian {
  int index;
  int line;
  int pc;
  int values;
  int ints[1024];
};

enum Tokens* tokenize(const char *src, int *count, struct Durian *dur) {
  int len = strlen(src);
  enum Tokens *toks = malloc(len * sizeof(enum Tokens));
  if (!toks) return NULL;

  int t = 0;
  
  while (src[dur->pc] != '\0') {
    char c = src[dur->pc];

    if (c == '+') {
      toks[t++] = ADD;
      dur->pc++;
      dur->index++;
    } else if (c == '-') {
      toks[t++] = SUB;
      dur->pc++;
      dur->index++;
    } else if (isalpha((unsigned char)c)) {
      char str[1024];
      int j = 0;
      while (isalpha((unsigned char)src[dur->pc]) && j < len) {
        str[j++] = src[dur->pc++];
      }
      str[j] = '\0';

      if (strcmp(str, "push") == 0) {
        toks[t++] = PUSH;
        dur->index++;
      }
      else if (strcmp(str, "pop") == 0) {
        toks[t++] = POP;
        dur->index++;
      } else {
        printf("Unrecognized string: %s (token: %d ,line: %d)\n", str, dur->index+1, dur->line+1);
        return NULL;
      }
    } else if (c == '\n') {
      dur->line++;
      dur->pc++;
      dur->index = 0;
    } else if (isdigit((unsigned char)c)) {
      char str[1024];
      int j = 0;
      while (isdigit((unsigned char)src[dur->pc]) && j < len) {
        str[j++] = src[dur->pc++];
      }
      str[j] = '\0';

      toks[t++] = INT;
      dur->ints[dur->values++] = atoi(str);
      dur->index++;
    } else {
      if (!isspace(c)) {
        printf("Unrecognized char: %c (token number: %d ,line: %d)\n", c, dur->index+1, dur->line+1);
        return NULL;
      } else {
        dur->pc++;
      }
    }
  }

  *count = t;
  return toks;
}

int main(int argc, char *argv[]) {
  bool cond = false;
  char *src = NULL;
  if (argc < 2) {
    printf("Error you did not pass enough arguments\n");
  } else if (argc == 2) {
    FILE *f = fopen(argv[1], "rb");
    cond = true;
    if (f == NULL) {
      printf("Error could not open file\n"); 
      return 0;
    } else {
      fseek(f, 0, SEEK_END);
      long size = ftell(f);
      rewind(f);

      src = malloc(size + 1);
      if (src == NULL) {
        printf("ERROR: ran out of memory\n");
        fclose(f);
        return 1;
      }

      size_t bytes_read = fread(src, 1, size, f);
      src[bytes_read] = '\0';

      fclose(f);
    }
  } else {
    printf("You passed too many args\n");
  }

  if (cond && src != NULL) {
    struct Durian dur = {0};
  
    int count;
    enum Tokens *toks = tokenize(src, &count, &dur);
    if (toks == NULL) {
      free(src);
      return 0; 
    }

    for (int i = 0; i < count; i++) {
      printf("%d ", toks[i]);
    }
    printf("\n");

    for (int j = 0; j < dur.values; j++) {
      printf("%d ", dur.ints[j]);
    }
    printf("\n");

    free(toks);
    free(src);
  }

  return 0;
}
