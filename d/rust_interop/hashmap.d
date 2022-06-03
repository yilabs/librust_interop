module rust_interop.hashmap;

import core.stdc.stdint;
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

// only use integer type as KeyT, since key will be passed to Rust thru ffi,
// there is no way (or difficult) to call any D side's KeyT.cmp function on the Rust side.
// it's the user's responsibility to make sure the KeyT, ValT can be passed to DashMap
class DashMap(KeyT, ValT) {
  static assert(isIntegral!(KeyT));
  static assert(canBeFFIValType!(ValT));

  private HandleT _handle;

  this() {
    _handle = dashmap_new();
  }

  uintptr_t length() {
    return dashmap_length(_handle);
  }

  // as drop-in replacement of other D hashmap, let's use the D method name convention
  ValT get(KeyT key) {
    ulong val = dashmap_get(_handle, key);
    return cast(ValT)(cast(void*)val);
  }

  /**
   * Supports `aa[key]` syntax.
   */
  ValT opIndex(KeyT key) {
    return this.get(key);
  }

  /**
   * Supports $(B aa[key] = value;) syntax.
   */
  void opIndexAssign(ValT val, const KeyT key) {
    dashmap_insert(_handle, key, cast(ulong)val);
  }
}

};

mixin(DashMapDecl);

unittest {
  int n = 10;

//auto hs = new DashMap!(int, Small);
  auto hS = new DashMap!(int, SmallPtr);
//auto sS = new DashMap!(Small, SmallPtr);  // `isIntegral!(Small)` is false
  // try struct* SmallPtr
  assert(hS.length == 0);
  foreach (i; 0 .. n) {
    SmallPtr sp = new Small(i * i);
    hS[i] = sp;  // insert
  }
  assert(hS.length == 10);
  foreach (i; 0 .. n) {
    SmallPtr sp = hS[i];  // get
    assert(sp.val == (i * i));
  }
  assert(hS.length == 10);

  auto hm = new DashMap!(int, int);
  assert(hm.length == 0);
  foreach (i; 0 .. n) {
    hm[i] = (i * i);
  }
  assert(hm.length == 10);
  foreach (i; 0 .. n) {
    assert(hm[i] == (i * i));
  }
  assert(hm.length == 10);
}

