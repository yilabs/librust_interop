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
}

unittest {
  {
  SegQueue queue = new SegQueue();
  assert(queue.handle !is null);
  }
  {
  SegQueue queue = new SegQueue();
  assert(queue.handle !is null);
  }
}
