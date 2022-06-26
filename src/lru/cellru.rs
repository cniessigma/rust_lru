use std::collections::HashMap;
use std::hash::Hash;
use crate::lru::{LRU, KeyHolder};
use crate::linked_list::cellist;
use std::marker::PhantomData;


pub struct CellLRU<K: Eq + Hash + Copy, T> {
  key_holder: KeyHolder<K, T, cellist::CellLinkedList<(K, T)>>,
}

impl<K: Eq + Hash + Copy, T> LRU<K, T> for CellLRU<K, T> {
  type List = cellist::CellLinkedList<(K, T)>;

  fn new(capacity: usize) -> Self {
    CellLRU {
      key_holder: KeyHolder {
        hash: HashMap::new(),
        list: cellist::CellLinkedList::new(),
        _marker: PhantomData,
        size: 0,
        capacity: capacity,
      },
    }
  }

  fn key_holder(&mut self) -> &mut KeyHolder<K, T, cellist::CellLinkedList<(K, T)>> {
    &mut self.key_holder
  }
}


crate::lru::macros::lru_tests!(CellLRU);
