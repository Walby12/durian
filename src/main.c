#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

enum Tokens {
    ADD,
    SUB,
    PUSH,
    POP
};

/* Structure that is meant to desciribe the virtual machine */
struct Durian {
  int index;
  int line;
  int pc;
};

/* Function that takes a string a count and a pointer to the vm and returns an array of Tokens 
 * NOTE: The count is only useful for printing the token array */
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
        printf("Unrecognized string: %s at token: %d at line: %d", str, dur->index+1, dur->line+1);
        return NULL;
      }
    } else if (c == '\n') {
      dur->line++;
      dur->pc++;
      dur->index = 0;
    } else {
      dur->pc++;
    }
  }

  *count = t;
  return toks;
}

/* TODO: Add file reading thru the arguments passed */
int main() {
  struct Durian dur = {0, 0, 0};

  int count;
  enum Tokens *toks = tokenize("+ - push pop", &count, &dur);

  for (int i = 0; i < count; i++) {
      printf("%d ", toks[i]);
  }
  printf("\n");

  free(toks);
  return 0;
}
