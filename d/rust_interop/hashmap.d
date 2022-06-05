module rust_interop.hashmap;

import core.stdc.stdint;
import std.algorithm.iteration;
import std.array;
import std.stdio;
import std.traits;

import rust_interop_h;  // rust exported header -> d file


// types whose .sizeof < uintptr_t.sizeof, can be pass over FFI to C or Rust
bool canBeFFIValType(T)() {
  static if (T.sizeof <= uintptr_t.sizeof) {
    return true;
  } else {
    return false;
  }
}

version(unittest) {
  struct Small { int val; }
  struct Exact { uintptr_t up; }
  struct Big   { uintptr_t up; byte b; }
  alias SmallPtr = Small*;
  alias ExactPtr = Exact*;
  alias   BigPtr =   Big*;

}
unittest {
  assert(Small.sizeof == 4);

  assert(!canBeFFIValType!(int[]));  // .ptr + .len
  assert( canBeFFIValType!(int));
  assert( canBeFFIValType!(uint));
  assert( canBeFFIValType!(size_t));

  assert( canBeFFIValType!(Small));
  assert( canBeFFIValType!(Exact));
  assert(!canBeFFIValType!(  Big));

  assert(canBeFFIValType!(SmallPtr));
  assert(canBeFFIValType!(ExactPtr));
  assert(canBeFFIValType!(  BigPtr));
}

enum DashMapDecl = q{

// using the D naming convention
// https://github.com/dlang-community/containers/blob/master/src/containers/hashmap.d

// only use integer type as KT, since key will be passed to Rust thru ffi,
// there is no way (or difficult) to call any D side's KT.cmp function on the Rust side.
// it's the user's responsibility to make sure the KeyT, ValT can be passed to DashMap
// this class is shared: otherwise, please use a normal hashmap
shared class DashMap(KT, VT) {
  static assert(isIntegral!(KT));
  static assert(is(KT == KeyT));  // TODO: right now, only ulong key is supported in Rust
  static assert(canBeFFIValType!(VT));

  private HandleT _handle;

  this() {
    _handle = dashmap_new();
  }

  uintptr_t length() {
    return dashmap_length(_handle);
  }

  bool containsKey(KT key) {
    return dashmap_contains_key(_handle, key);
  }

  // NOTE: right now, since we will directly .unwrap() in Rust, the caller need to make sure that dashmap_contains_key()
  // as drop-in replacement of other D hashmap, let's use the D method name convention
  VT get(KT key) {
    ValT val = dashmap_get(_handle, key);
    return cast(VT)(cast(void*)val);
  }

  VT getOrAdd(KT key, VT delegate() gen) {
    VT val;
    if (!containsKey(key)) {  // this is a FFI call! but there is not way to optimize since on the original Rust function only return bool (instead of None(ValT))
      synchronized(this) {
        // NOTE: does this sync(this) defeat the purpose (using lock-free / high performant Rust hashmap) for a insert-heavy usage?
        // since this sync(this) op will always happen on the D side
        val = gen();
        this.opIndexAssign(val, key);
      }
    } else {
      val = this.get(key);
    }
    return val;
  }

  /**
   * Supports `aa[key]` syntax.
   */
  VT opIndex(KT key) {
    return this.get(key);
  }

  /**
   * Supports $(B aa[key] = value;) syntax.
   */
  void opIndexAssign(VT val, const KT key) {
    dashmap_insert(_handle, key, cast(ValT)(cast(void*)val));
  }

  KT[] keys() {
    synchronized(this) {  // the 2 call: dashmap_length, and dashmap_keys has to be in the same sync block
      auto len = this.length();
      // alloc C array to be passed to Rust
      ValT[] ks = new ValT[len];
      dashmap_keys(_handle, ks.ptr, ks.length);
      return ks;
    }
  }

  VT[] values() {
    synchronized(this) {  // the 2 call: dashmap_length, and dashmap_values has to be in the same sync block
      auto len = this.length();
      // alloc C array to be passed to Rust
      ValT[] vs = new ValT[len];
      dashmap_values(_handle, vs.ptr, vs.length);

      // convert back to the result type
      static if(is(VT == ValT)) {
        VT[] rs = vs;
      } else {
        VT[] rs = new VT[len];
        foreach (i, e; vs) {
          rs[i] = cast(VT)(cast(void*)e);
        }
      }

      return rs;
    }
  }
}

};

mixin(DashMapDecl);

unittest {
  int n = 10;

//auto hs = new DashMap!(int, Small);
  auto hS = new shared DashMap!(KeyT, SmallPtr);
//auto sS = new DashMap!(Small, SmallPtr);  // `isIntegral!(Small)` is false
  // try struct* SmallPtr
  assert(hS.length == 0);
  SmallPtr[] dSideRefHolder = new SmallPtr[n];
  foreach (i; 0 .. n) {
    SmallPtr sp = new Small(i * i);
    dSideRefHolder[i] = sp;
    hS[i] = sp;  // insert
  }
  assert(hS.length == 10);
  foreach (i; 0 .. n) {
    SmallPtr sp = hS[i];  // get
    assert(sp.val == (i * i));
  }
  assert(hS.length == 10);
  writeln(hS.keys());
  writeln(map!(s => *s)(hS.values()));  // write the struct!

  auto hm = new shared DashMap!(KeyT, int);
  assert(hm.length == 0);
  foreach (i; 0 .. n) {
    hm[i] = (i * i);
  }
  assert(hm.length == 10);
  foreach (i; 0 .. n) {
    assert(hm[i] == (i * i));
  }
  assert(hm.length == 10);
  writeln(hm.keys());
  writeln(hm.values());
}

