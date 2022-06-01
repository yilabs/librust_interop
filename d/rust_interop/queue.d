module rust_interop.queue;

import core.stdc.stdint;
// link to rslib.a

import rust_interop_h;  // rust exported header -> d file

// wrapper of https://docs.rs/crossbeam-queue/0.3.5/crossbeam_queue/struct.SegQueue.html
// use the same name as in Rust
class SegQueue {
  CVoidPtr handle;
  this() {
    handle = segqueue_new();
  }

  uintptr_t length() {
    return dashmap_len(handle);
  }
}

unittest {
  SegQueue q1 = new SegQueue();
  assert(q1.handle !is null);

  SegQueue q2 = new SegQueue();
  assert(q2.handle !is null);
  assert(q2.length  == 0);

  assert(q1.length  == 0);
}
