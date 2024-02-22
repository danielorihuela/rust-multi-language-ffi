#include <stdio.h>
#include "bindings.h"

int main() {
   InMemoryKeyStore ks = new_keystore();

   add(&ks, 42);
   add(&ks, 84);
   add(&ks, 146);

   printf("length = %lu\n", ks.len);
   printf("capacity = %lu\n\n", ks.cap);

   for(int i = 0; i < 10; i++) {
      printf("value at %i = %i\n", i, ks.keys[i]);
   }

   return 0;
}