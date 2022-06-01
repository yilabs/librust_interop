#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef uintptr_t HandleT;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

HandleT dashmap_new(void);

HandleT segqueue_new(void);

uint64_t dashmap_get(HandleT handle, uint64_t key);

uint64_t dashmap_insert(HandleT handle, uint64_t key, uint64_t val);

uintptr_t dashmap_len(HandleT handle);

uint64_t segqueue_pop(HandleT handle);

void segqueue_push(HandleT handle, uint64_t val);

uintptr_t segqueue_len(HandleT handle);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
