import core.stdc.config;
import core.stdc.stdint;

extern (C):

alias HandleT = c_ulong;

alias KeyT = c_ulong;

alias ValT = c_ulong;

// __cplusplus

HandleT dashmap_new ();

HandleT segqueue_new ();

bool dashmap_contains_key (HandleT handle, KeyT key);

ValT dashmap_get (HandleT handle, KeyT key);

ValT dashmap_insert (HandleT handle, KeyT key, ValT val);

uintptr_t dashmap_length (HandleT handle);

void dashmap_keys (HandleT handle, ValT* c_array, uintptr_t length);

void dashmap_values (HandleT handle, ValT* c_array, uintptr_t length);

ValT segqueue_pop (HandleT handle);

bool segqueue_push (HandleT handle, ValT val);

uintptr_t segqueue_length (HandleT handle);

void segqueue_destroy (HandleT handle);

// extern "C"
// __cplusplus
