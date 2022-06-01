#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef void *CVoidPtr;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

CVoidPtr dashmap_new(void);

CVoidPtr segqueue_new(void);

uint64_t dashmap_get(CVoidPtr handle, uint64_t key);

uint64_t dashmap_insert(CVoidPtr handle, uint64_t key, uint64_t val);

uint64_t segqueue_pop(CVoidPtr handle);

void segqueue_push(CVoidPtr handle, uint64_t val);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
