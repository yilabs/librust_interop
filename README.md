# librust_interop
Rust interop with other programming languages: C, D, ... etc.

Mainly to make high performant, memory safe Rust libraries available to other languages users.

Recently wrapped:

* DashMap: is an implementation of a concurrent associative array/hashmap in Rust.
dashmap-5.3.4 Blazing fast concurrent HashMap for Rust.
https://docs.rs/dashmap/5.3.4/dashmap/struct.DashMap.html

* SegQueue: An unbounded multi-producer multi-consumer queue.
https://docs.rs/crossbeam-queue/0.3.5/crossbeam_queue/struct.SegQueue.html


# Note:
Right now:
* only u64 is used as key type, and value type
* so for value type: only pass integer type or pointer type to the container.

# Pre-req:

* Rust nightly: for https://github.com/dtolnay/cargo-expand
* cbindgen: https://github.com/eqrion/cbindgen
* dpp: https://code.dlang.org/packages/dpp
* dstep: https://code.dlang.org/packages/dstep
