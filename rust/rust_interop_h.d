extern (C):

alias CVoidPtr = void*;

// __cplusplus

CVoidPtr dashmap_new ();

CVoidPtr segqueue_new ();

ulong dashmap_get (CVoidPtr handle, ulong key);

ulong dashmap_insert (CVoidPtr handle, ulong key, ulong val);

ulong segqueue_pop (CVoidPtr handle);

void segqueue_push (CVoidPtr handle, ulong val);

// extern "C"
// __cplusplus
