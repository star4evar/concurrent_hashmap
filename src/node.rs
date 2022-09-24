use crossbeam::epoch::{Atomic, Guard, Shared};
use std::sync::atomic::Ordering;
use std::cell::UnsafeCell;

/// Entry in a bin
///
/// will generally be 'Node', Any entry that is not first in the bin will be a 'node'
pub(crate) enum BinEntry<K,V>{
    Node(Node<K,V>),
}

impl<K,V> BinEntry<K,V>
where
    K: Eq,
{
    pub(crate) fn find<'g>(
        &'g self,
        hash: u64,
        key: &K,
        guard: &'g Guard
    ) -> Shared<'g, Node<K,V>> {
        match *self {
            BinEntry::Node(ref start) => {
                if n.hash == hash && &n.key == key{
                    return Some(n);
                }
                let next = n.next().load(Ordering::SeqCst, guard);
                if next::is_null(){
                    return Shared::null();
                }
                return next;
            }
        }
    }
}

/// Key-value entry
pub(crate) struct Node<K,V>{
    pub(crate) hash: u64,
    pub(crate) key: K,
    pub(crate) value: Atomic<V>,
    pub(crate) next: Atomic<BinEntry<K,V>>,
}


