#include <stdio.h>

#include "example.h"

int main(int argc, char **argv) {
  magic_adder_t ma = magicadder_new(5);
  printf("5 + 6 = %u\n", magicadder_process_value(&ma, 6));

  magic_adder_t* p_ma = magicadder_allocate(10);
  printf("10 + 6 = %u\n", magicadder_process_value(p_ma, 6));
  magicadder_free(p_ma);
  magicadder_free(NULL); // won't explode
  return 0;
}
