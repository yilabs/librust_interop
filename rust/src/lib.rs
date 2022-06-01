use std::ffi::{c_void};

use dashmap::DashMap;
use crossbeam_queue::SegQueue;

type CVoidPtr = *mut c_void;  // this is `void *` in C

type HashMapT = DashMap<u64, u64>;  // DashMap that stores u64 value from D side
type QueueT = SegQueue<u64>;

/* NOTE: all the exported functions use C naming convention.
*/


// return a CVoidPtr
macro_rules! create_function { ($func_name:ident, $ctype:ty) => {

#[no_mangle]
pub unsafe extern "C" fn $func_name() -> CVoidPtr {
  let mut obj = <$ctype>::new();
  let obj_ptr: CVoidPtr = &mut obj as *mut _ as CVoidPtr;
  return obj_ptr;
}

}; }

create_function!( dashmap_new, HashMapT);
create_function!(segqueue_new,   QueueT);

macro_rules! cast_back_to_rust { ($obj:ident, $c_void_ptr:ident, $otype:ty) => {
  // https://stackoverflow.com/a/24191977
  // unsafe is needed because we dereference a raw pointer here
  #[allow(unused_unsafe)]
  let $obj: &mut $otype = unsafe { &mut *($c_void_ptr as *mut $otype) };
}; }

#[no_mangle]
pub unsafe extern "C" fn dashmap_get(handle:CVoidPtr, key:u64) -> u64 {
  cast_back_to_rust!(obj, handle, HashMapT);
  *(obj.get(&key).unwrap())
}

// return the old val
#[no_mangle]
pub unsafe extern "C" fn dashmap_insert(handle:CVoidPtr, key:u64, val:u64) -> u64 {
  cast_back_to_rust!(obj, handle, HashMapT);
  obj.insert(key, val).unwrap()
}


#[no_mangle]
pub unsafe extern "C" fn segqueue_pop(handle:CVoidPtr) -> u64 {
  cast_back_to_rust!(obj, handle, QueueT);
  obj.pop().unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn segqueue_push(handle:CVoidPtr, val:u64) {
  cast_back_to_rust!(obj, handle, QueueT);
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
