#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef uintptr_t HandleT;

typedef uint64_t ValT;

typedef uint64_t KeyT;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

HandleT dashmap_new(void);

HandleT segqueue_new(void);

ValT dashmap_get(HandleT handle, KeyT key);

ValT dashmap_insert(HandleT handle, KeyT key, ValT val);

uintptr_t dashmap_length(HandleT handle);

void dashmap_keys(HandleT handle, ValT *c_array, uintptr_t length);

void dashmap_values(HandleT handle, ValT *c_array, uintptr_t length);

ValT segqueue_pop(HandleT handle);

bool segqueue_push(HandleT handle, ValT val);

uintptr_t segqueue_length(HandleT handle);

void segqueue_destroy(HandleT handle);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
