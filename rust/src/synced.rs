use std::slice;
use once_cell::sync::Lazy;
use std::sync::RwLock;

use dashmap::DashMap;
use crossbeam_queue::SegQueue;
/*
use lockfree::queue::Queue;
type QueueT = Queue<ValT>;
*/

type HandleT = usize;  // the handle which "holds" the DashMap or SegQueue object

type KeyT = u64;  // TODO: fixed for now
type ValT = u64;  // of the container

type DashMapT = DashMap<KeyT, ValT>;  // on the Rust side, DashMap whose <key, value> are both u64 from D side
type DashMapsT = Vec<DashMapT>;

type SegQueueT = SegQueue<ValT>;
type SegQueuesT = Vec<SegQueueT>;


// How do I create a global, mutable singleton?
// https://stackoverflow.com/a/27826181
static  DASHMAPS: Lazy<RwLock< DashMapsT>> = Lazy::new(|| RwLock::new(vec![]));
static SEGQUEUES: Lazy<RwLock<SegQueuesT>> = Lazy::new(|| RwLock::new(vec![]));

/*
fn main() {
    segqueue_new();
    segqueue_new();
    segqueue_new();

    println!("called {}", ARRAY.read().unwrap().len());
}
*/


// NOTE: use RwLock to protect `dashmap_new` to be thread-safe, since it will modify the underlying container Vec
// so in the most called function dashmap_get, dashmap_insert, the DASHMAPS.get() no need to be sync-ed
// othewise, it will be very slow
// return a handle
macro_rules! create_function_new { ($func_name:ident, $cell:ident, $ctype:ty) => {

#[no_mangle]
pub unsafe extern "C" fn $func_name() -> HandleT {
  let mut w = $cell.write().unwrap();  // hold the write lock!
  let handle:HandleT = w.len();
  w.push(<$ctype>::new());

  return handle;
}

}; }

create_function_new!( dashmap_new,  DASHMAPS,  DashMapT);
create_function_new!(segqueue_new, SEGQUEUES, SegQueueT);

macro_rules! get_handle_obj { ($container:ident, $handle:ident, $obj:ident, $code:block) => {
  let read = $container.read().unwrap();  // NOTE: we only use the read lock on the container Vec!
  let $obj = read.get($handle).unwrap();
  { $code }
}; }

#[no_mangle]
pub unsafe extern "C" fn dashmap_contains_key(handle:HandleT, key:KeyT) -> bool {
  get_handle_obj!(DASHMAPS, handle, obj,
    { return obj.contains_key(&key); }
  );
}


// NOTE: right now, since we will directly .unwrap(), the caller need to make sure that dashmap_contains_key()
#[no_mangle]
pub unsafe extern "C" fn dashmap_get(handle:HandleT, key:KeyT) -> ValT {
  get_handle_obj!(DASHMAPS, handle, obj,
    { return *(obj.get(&key).unwrap()); }
  );
}


// return the old val if there is any; NOTE: otherwise, will return 0 (TODO?)
#[no_mangle]
pub unsafe extern "C" fn dashmap_insert(handle:HandleT, key:KeyT, val:ValT) -> ValT {
  get_handle_obj!(DASHMAPS, handle, obj,
    { match obj.insert(key, val) {
        Some(old) => return old,
        None      => return 0,
      }
    }
  );
}

#[no_mangle]
pub unsafe extern "C" fn dashmap_length(handle:HandleT) -> usize {
  get_handle_obj!(DASHMAPS, handle, obj,
    { return obj.len(); }
  );
}

// NOTE: the caller need to make sure that the c_array length is big enough to hold all the keys/values in the map
// for C arrays, need to pass array size
macro_rules! create_function { ($func_name:ident, $func:ident) => {

#[no_mangle]
pub extern "C" fn $func_name(handle:HandleT, c_array: *mut ValT, length: usize) {
  // build a Rust array from array & length
  let rust_array: &mut [ValT] = unsafe { slice::from_raw_parts_mut(c_array, length as usize) };
  get_handle_obj!(DASHMAPS, handle, obj, {
    let mut i = 0;
    for it in obj.iter() {
      rust_array[i] = *it.$func();
      i += 1;
    }
  });
}

}; }


create_function!(dashmap_keys, key);
create_function!(dashmap_values, value);


#[no_mangle]
pub unsafe extern "C" fn segqueue_pop(handle:HandleT) -> ValT {
  get_handle_obj!(SEGQUEUES, handle, obj,
    { return obj.pop().unwrap(); }
  );
}

// return bool: ok or err
#[no_mangle]
pub unsafe extern "C" fn segqueue_push(handle:HandleT, val:ValT) -> bool {
  get_handle_obj!(SEGQUEUES, handle, obj,
    { obj.push(val); return true; }
  );
}

// NOTE: _destroy and _length are D's convention
#[no_mangle]
pub unsafe extern "C" fn segqueue_length(handle:HandleT) -> usize {
  get_handle_obj!(SEGQUEUES, handle, obj,
    { return obj.len(); }
  );
}

#[no_mangle]
pub unsafe extern "C" fn segqueue_destroy(handle:HandleT) {
  get_handle_obj!(SEGQUEUES, handle, obj,
    { drop(obj); }  // TODO: also remove from SEGQUEUES vec?
  );
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
