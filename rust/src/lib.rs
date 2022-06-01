use std::ffi::{c_void};

use dashmap::DashMap;
use crossbeam_queue::SegQueue;

type CVoidPtr = *mut c_void;  // this is `void *` in C

type HashMapT = DashMap<u64, u64>;  // DashMap that stores u64 value from D side
type QueueT = SegQueue<u64>;

/* NOTE: all the exported functions use C naming convention.
*/
// https://stackoverflow.com/a/24191977
macro_rules! cast_rust_obj_to_c_void_ptr { ($obj:ident, $c_void_ptr:ident) => {
  let $c_void_ptr: CVoidPtr = &mut $obj as *mut _ as CVoidPtr;
}; }

// NOTE: this c_void_ptr *is* originally a Rust object!
macro_rules! cast_c_void_ptr_back_to_rust { ($obj:ident, $otype:ty, $c_void_ptr:ident) => {
  // unsafe is needed because we dereference a raw pointer here
  #[allow(unused_unsafe)]
  let $obj: &mut $otype = unsafe { &mut *($c_void_ptr as *mut $otype) };
}; }



// return a CVoidPtr
macro_rules! create_function { ($func_name:ident, $ctype:ty) => {

#[no_mangle]
pub unsafe extern "C" fn $func_name() -> CVoidPtr {
  let mut obj = <$ctype>::new();
  cast_rust_obj_to_c_void_ptr!(obj, c_void_ptr);
  return c_void_ptr;
}

}; }

create_function!( dashmap_new, HashMapT);
create_function!(segqueue_new,   QueueT);

#[no_mangle]
pub unsafe extern "C" fn dashmap_get(handle:CVoidPtr, key:u64) -> u64 {
  cast_c_void_ptr_back_to_rust!(obj, HashMapT, handle);
  *(obj.get(&key).unwrap())
}

// return the old val
#[no_mangle]
pub unsafe extern "C" fn dashmap_insert(handle:CVoidPtr, key:u64, val:u64) -> u64 {
  cast_c_void_ptr_back_to_rust!(obj, HashMapT, handle);
  obj.insert(key, val).unwrap()
}


#[no_mangle]
pub unsafe extern "C" fn segqueue_pop(handle:CVoidPtr) -> u64 {
  cast_c_void_ptr_back_to_rust!(obj, QueueT, handle);
  obj.pop().unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn segqueue_push(handle:CVoidPtr, val:u64) {
  cast_c_void_ptr_back_to_rust!(obj, QueueT, handle);
  obj.push(val)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
