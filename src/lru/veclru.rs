use std::collections::HashMap;
use std::hash::Hash;
use crate::lru::LRU;
use crate::linked_list::DLL;
use crate::linked_list::veclist;


pub struct VecLRU<K: Eq + Hash + Copy, T: Copy> {
  list: veclist::VectorLinkedList<(K, T)>,
  hash_map: HashMap<K, veclist::NodePointer>
}

impl<K: Eq + Hash + Copy, T: Copy> LRU<K, T> for VecLRU<K, T> {
  type List = veclist::VectorLinkedList<(K, T)>;

  fn new(capacity: usize) -> Self {
    VecLRU {
      list: veclist::VectorLinkedList::new(capacity),
      hash_map: HashMap::new(),
    }
  }

  fn size(&self) -> usize {
    self.list.size()
  }

  fn capacity(&self) -> usize {
    self.list.capacity()
  }

  fn hash_table(&mut self) -> &mut HashMap<K, veclist::NodePointer> {
    &mut self.hash_map
  }

  fn linked_list(&mut self) -> &mut veclist::VectorLinkedList<(K, T)> {
    &mut self.list
  }
}


crate::lru::macros::lru_tests!(VecLRU);
