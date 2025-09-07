#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <stdbool.h>

enum Tokens {
  ADD,
  SUB,
  PUSH,
  POP
};

struct Durian {
  int index;
  int line;
  int pc;
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
      while (isalpha((unsigned char)src[dur->pc]) || j == len) {
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
  char src[1024];

  bool cond = false;

  if (argc < 2) {
    printf("Error you did not pass enough arguments\n");
  } else if (argc == 2) {
    FILE *f = fopen(argv[1], "r");
    cond = true;
    if (f == NULL) {
      printf("Error could not open file\n"); 
      return 0;
    } else {
      fgets(src, 1024, f);
      fclose(f);
    }
  } else {
    printf("You passed to many args\n");
  }

  if (cond) {
    struct Durian dur = {0, 0, 0};
  
    int count;
    enum Tokens *toks = tokenize(src, &count, &dur);
    if (toks == NULL) return 0; 
    for (int i = 0; i < count; i++) {
      printf("%d ", toks[i]);
    }
    printf("\n");

    free(toks);
  }
  return 0;
}
