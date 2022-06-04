# librust_interop
Rust interop with other programming languages: C, D, ... etc.

Mainly to make high performant, memory safe Rust libraries available to other languages users.

Recently wrapped:

* DashMap: is an implementation of a concurrent associative array/hashmap in Rust.
dashmap-5.3.4 Blazing fast concurrent HashMap for Rust.
https://docs.rs/dashmap/5.3.4/dashmap/struct.DashMap.html

* SegQueue: An unbounded multi-producer multi-consumer queue.
https://docs.rs/crossbeam-queue/0.3.5/crossbeam_queue/struct.SegQueue.html


## Note:
Right now:
* only u64 is used as key type, and value type
* so for value type: only pass integer type or pointer type to the container.


We use the same memory management method as liblfdsd:
https://github.com/mw66/liblfdsd#memory-management-remember-this-is-a-thin-wrapper-lib-in-d

Let Rust's be Rust's, and let D's be D's, i.e.

* Rust manage Rust's memory (the container), and
* D manage D's memory (the objects)

The only thing interfacing between Rust and D is the simple uintptr_t (void*) as value, so to use this D library:

all primitive types (whose .sizeof upto pointer size on the target machine) | class (pointer)'s value are stored as value of uintptr_t
everything else, i.e. all (fat) objects' address are stored as value of uintptr_t

The only extra requirement on the D side is to keep reference to those fat objects to avoid it being GC-ed before being pop-ed.

(Just as: don't push a stack var into any-type-of queue, and pop it after the stack is gone -- this is the responsibility of the programmer, not the container.)

That's all.

And please remember to keep a reference on the D side of the fat objects you put in the container:

## Pre-req:

* Rust nightly: for https://github.com/dtolnay/cargo-expand
* cbindgen: https://github.com/eqrion/cbindgen
* dpp: https://code.dlang.org/packages/dpp
* dstep: https://code.dlang.org/packages/dstep
