module rust_interop.queue;

import core.stdc.stdint;
import std.array;
import std.stdio;

import liblfdsd;

import rust_interop_h;  // rust exported header -> d file

// wrapper of https://docs.rs/crossbeam-queue/0.3.5/crossbeam_queue/struct.SegQueue.html
// use the same name as in Rust
// this class is shared: otherwise, please use a normal queue
// Crate crossbeam_queue: This crate provides concurrent queues that can be shared among threads:
// SegQueue, an unbounded MPMC queue that allocates small buffers, segments, on demand.


version (all) {  // manual wrap

shared class SegQueue {
  HandleT handle;

  this() {
    handle = segqueue_new();
  }

  bool empty() {
    return 0 == this.length();
  }

  bool full() {
    return false;  // SegQueue is *unbounded* multi-producer multi-consumer queue.
  }

  void push(ulong val) {  // TODO: queue_bmm push void*, but here we use ulong!
    segqueue_push(handle, val);
  }

  ulong pop() {
    return segqueue_pop(handle);
  }

  uintptr_t length() {
    return segqueue_length(handle);
  }
}

} else {

// TODO: refine this!
enum segqueue_decl = liblfdsd.queue_bmm_decl
    .replace("c_queue_bmm*", "HandleT")
    .replace("queue_bmm_new(n)", "queue_bmm_new()")
    .replace("int ok;", "int ok=1;")
    .replace(", &ok", "")
    .replace("void* value", "ulong value")
    .replace("queue_bmm", "segqueue");
mixin(segqueue_decl);

alias SegQueue = segqueue!int;
}

unittest {
  auto q1 = new shared SegQueue();
//assert(q1.handle == 0);
  assert(q1.length == 0);

  auto q2 = new shared SegQueue();
//assert(q2.handle == 1);
  assert(q2.length == 0);
  assert(q1.length == 0);

  // try some push
  auto n = 10;
  foreach (i; 0 .. n) {
    assert(q1.length == i);
    q1.push(i);
  }
  assert(q1.length == 10);
  // writeln(q1.length);

  // now pop
  foreach (i; 0 .. n) {
    auto e = q1.pop();
    // writeln(e);
    assert(e == i);
  }
  assert(q1.length == 0);
  assert(q2.length == 0);

  // test segqueue
//auto sq = new shared(segqueue!int);
}
