use std::collections::HashMap;
use std::hash::Hash;
use crate::lru::{LRU, KeyHolder};
use crate::linked_list::{DLL, veclist};
use std::marker::PhantomData;


pub struct VecLRU<K: Eq + Hash + Copy, T> {
  key_holder: KeyHolder<K, T, veclist::VectorLinkedList<(K, T)>>,
}

impl<K: Eq + Hash + Copy, T> LRU<K, T> for VecLRU<K, T> {
  type List = veclist::VectorLinkedList<(K, T)>;

  fn new(capacity: usize) -> Self {
    VecLRU {
      key_holder: KeyHolder {
        hash: HashMap::new(),
        list: veclist::VectorLinkedList::new(),
        _marker: PhantomData,
        size: 0,
        capacity: capacity,
      },
    }
  }

  fn key_holder(&mut self) -> &mut KeyHolder<K, T, veclist::VectorLinkedList<(K, T)>> {
    &mut self.key_holder
  }
}


crate::lru::macros::lru_tests!(VecLRU);
