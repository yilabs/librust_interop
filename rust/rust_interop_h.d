import core.stdc.config;

extern (C):

alias HandleT = c_ulong;

// __cplusplus

HandleT dashmap_new ();

HandleT segqueue_new ();

ulong dashmap_get (HandleT handle, ulong key);

ulong dashmap_insert (HandleT handle, ulong key, ulong val);

ulong segqueue_pop (HandleT handle);

void segqueue_push (HandleT handle, ulong val);

// extern "C"
// __cplusplus
