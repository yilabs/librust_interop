import core.stdc.config;
import core.stdc.stdint;

extern (C):

alias HandleT = c_ulong;

// __cplusplus

HandleT dashmap_new ();

HandleT segqueue_new ();

ulong dashmap_get (HandleT handle, ulong key);

ulong dashmap_insert (HandleT handle, ulong key, ulong val);

uintptr_t dashmap_length (HandleT handle);

ulong segqueue_pop (HandleT handle);

void segqueue_push (HandleT handle, ulong val);

uintptr_t segqueue_length (HandleT handle);

void segqueue_destroy (HandleT handle);

// extern "C"
// __cplusplus
