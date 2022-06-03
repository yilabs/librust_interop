use once_cell::sync::Lazy;
use std::sync::RwLock;

use dashmap::DashMap;
use crossbeam_queue::SegQueue;
/*
use lockfree::queue::Queue;
type QueueT = Queue<u64>;
*/

type HandleT = usize;

type DashMapT = DashMap<u64, u64>;  // DashMap that stores u64 value from D side
type DashMapsT = Vec<DashMapT>;

type SegQueueT = SegQueue<u64>;
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
macro_rules! create_function { ($func_name:ident, $cell:ident, $ctype:ty) => {

#[no_mangle]
pub unsafe extern "C" fn $func_name() -> HandleT {
  let mut w = $cell.write().unwrap();  // hold the write lock!
  let handle:HandleT = w.len();
  w.push(<$ctype>::new());

  return handle;
}

}; }

create_function!( dashmap_new,  DASHMAPS,  DashMapT);
create_function!(segqueue_new, SEGQUEUES, SegQueueT);

macro_rules! get_handle_obj { ($container:ident, $handle:ident, $obj:ident, $code:block) => {
  let read = $container.read().unwrap();  // NOTE: we only use the read lock on the container Vec!
  let $obj = read.get($handle).unwrap();
  { $code }
}; }

#[no_mangle]
pub unsafe extern "C" fn dashmap_get(handle:HandleT, key:u64) -> u64 {
  get_handle_obj!(DASHMAPS, handle, obj,
    { return *(obj.get(&key).unwrap()); }
  );
}


// return the old val if there is any; NOTE: otherwise, will return 0 (TODO?)
#[no_mangle]
pub unsafe extern "C" fn dashmap_insert(handle:HandleT, key:u64, val:u64) -> u64 {
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


#[no_mangle]
pub unsafe extern "C" fn segqueue_pop(handle:HandleT) -> u64 {
  get_handle_obj!(SEGQUEUES, handle, obj,
    { return obj.pop().unwrap(); }
  );
}

// return bool: ok or err
#[no_mangle]
pub unsafe extern "C" fn segqueue_push(handle:HandleT, val:u64) -> bool {
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
