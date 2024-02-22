#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct InMemoryKeyStore {
  uint8_t *keys;
  uintptr_t len;
  uintptr_t cap;
} InMemoryKeyStore;

struct InMemoryKeyStore new_keystore(void);

void add(struct InMemoryKeyStore *ks, uint8_t value);
