
use dashmap::DashMap;
use crossbeam_queue::SegQueue;
use once_cell;  // ::sync::OnceCell;  // shall we use sync/unsync?

// use lazy_static::lazy_static;

type HashMapT = DashMap<u64, u64>;  // DashMap that stores u64 value from D side
type HashMapsT = Vec<HashMapT>;
type HandleT = usize;
type QueueT = SegQueue<u64>;
type QueuesT = Vec<QueueT>;

// https://github.com/rust-lang-nursery/lazy-static.rs/blob/master/examples/mutex_map.rs
/*
lazy_static! {
  static ref HASHMAPS: HashMapsT = {
    let mut vec = Vec::<HashMapT>::new();
    vec
  };
}
*/

static mut HASHMAPS: once_cell::sync::Lazy<HashMapsT> = once_cell::sync::Lazy::new(HashMapsT::new);
static mut   QUEUES: once_cell::sync::Lazy<QueuesT>   = once_cell::sync::Lazy::new(QueuesT::new);

// NOTE: `dashmap_new` is NOT thread-safe, since it will modify the underlying container Vec
// so in the most called function dashmap_get, dashmap_insert, the HASHMAPS.get() no need to be sync-ed
// othewise, it will be very slow
// return a handle
macro_rules! create_function { ($func_name:ident, $cell:ident, $ctype:ty) => {

#[no_mangle]
pub unsafe extern "C" fn $func_name() -> HandleT {
  let map = <$ctype>::new();
  let handle:HandleT = $cell.len();
  $cell.push(map);

  return handle;
}

}; }

create_function!( dashmap_new, HASHMAPS, HashMapT);
create_function!(segqueue_new,   QUEUES,   QueueT);

#[no_mangle]
pub unsafe extern "C" fn dashmap_get(handle:HandleT, key:u64) -> u64 {
  *(HASHMAPS.get(handle).unwrap().get(&key).unwrap())
}

// return the old val
#[no_mangle]
pub unsafe extern "C" fn dashmap_insert(handle:HandleT, key:u64, val:u64) -> u64 {
  HASHMAPS.get(handle).unwrap().insert(key, val).unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn segqueue_pop(handle:HandleT) -> u64 {
  QUEUES.get(handle).unwrap().pop().unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn segqueue_push(handle:HandleT, val:u64) {
  QUEUES.get(handle).unwrap().push(val)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
