
/* ---------------- Constants -------------- */

/// The largest possible table capacity.  This value must be
/// exactly 1<<30 to stay within Java array allocation and indexing
/// bounds for power of two table sizes, and is further required
/// because the top two bits of 32bit hash fields are used for
/// control purposes.
const MAXIMUM_CAPACITY:usize = 1 << 30;

/// The default initial table capacity.  Must be a power of 2
/// (i.e., at least 1) and at most MAXIMUM_CAPACITY.
const DEFAULT_CAPACITY:usize = 16;


/// The load factor for this table. Overrides of this value in
/// constructors affect only the initial table capacity.  The
/// actual floating point value isn't normally used -- it is
/// simpler to use expressions such as {@code n - (n >>> 2)} for
/// the associated resizing threshold.
const LOAD_FACTOR:f64 = 0.75;


/// The largest possible (non-power of two) array size.
// /// Needed by toArray and related methods.
// const MAX_ARRAY_SIZE:usize = Integer.MAX_VALUE - 8;
//
// /// The default concurrency level for this table. Unused but
// /// defined for compatibility with previous versions of this class.
// private static final int DEFAULT_CONCURRENCY_LEVEL = 16;



/// Minimum number of rebinnings per transfer step. Ranges are
/// subdivided to allow multiple resizer threads.  This value
/// serves as a lower bound to avoid resizers encountering
/// excessive memory contention.  The value should be at least
/// DEFAULT_CAPACITY.
const MIN_TRANSFER_STRIDE: usize = 16;

/// The number of bits used for generation stamp in sizeCtl.
/// Must be at least 6 for 32bit arrays.
const RESIZE_STAMP_BITS: usize = 16;

/// The maximum number of threads that can help resize.
/// Must fit in 32 - RESIZE_STAMP_BITS bits.
const MAX_RESIZERS: usize = (1 << (32 - RESIZE_STAMP_BITS)) - 1;

/// The bit shift for recording size stamp in sizeCtl.
const RESIZE_STAMP_SHIFT: usize = 32 - RESIZE_STAMP_BITS;

/// Number of CPUS, to place bounds on some sizings
// static final int NCPU = Runtime.getRuntime().availableProcessors();

mod node;

use crossbeam::epoch::{Atomic, Guard, Shared};
use std::collections::hash_map::{BuildHasher, RandomState};

pub struct ConcurrentHashMap<K,V, S = RandomState>{
    table: Atomic<Table<K,V>>,
    build_hasher: S,
}

impl<K,V,S> ConcurrentHashMap<K,V,S>
where
    K: Hash,
    S: BuildHasher,
{
    pub fn get<'g>(&'g self, key: &K, guard: &'g Gurad ) -> Option<Shared<'g V>>{
        let mut h = self.build_hasher.build_hasher();
        key.hash(&mut h);
        let h = h.finish();
        let table = self.table.load(Ordering::SeqCst, guard);
        if table.is_null() {
            return None;
        }
        if table.bins.len() == 0 {
            return None;
        }

        let mask = table.bins.len() - 1;
        let bin_index = (h & mask) as usize;
        let bin = table.bin_at(bin_index, guard);
        if bin.is_null(){
            return None;
        }
        let node = bin.find(h, key);
        if node.is_null(){
            return None;
        }

        let v = node.value.load(Ordering::SeqCst, guard);
        assert!(!v.is_null());
        Some(v)
    }


    pub fn put(&self, key: K, value:V) -> Option<V>{
        //TODO
    }
}

struct Table<K,V> {
    bins: [Atomic<node::BinEntry<K,V>>],
}

impl Table<K,V> {
    fn bin_at<'g>(&'g self, i: usize, guard: &'g Guard) -> Shared<'g, node::BinEntry<K,V>> {
        self.bins[i].load(Ordering::Acquire, guard);
    }
}