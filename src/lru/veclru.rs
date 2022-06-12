use std::collections::HashMap;
use std::hash::Hash;
use crate::lru::LRU;
use crate::linked_list::DLL;
use crate::linked_list::veclist;


pub struct VecLRU<K: Eq + Hash + Copy, T: Copy> {
  list: veclist::VectorLinkedList<(K, T)>,
  hash_map: HashMap<K, veclist::NodePointer>
}

impl<K: Eq + Hash + Copy, T: Copy> VecLRU<K, T> {
  fn new(capacity: usize) -> Self {
    VecLRU {
      list: veclist::VectorLinkedList::new(capacity),
      hash_map: HashMap::new(),
    }
  }
}

impl<K: Eq + Hash + Copy, T: Copy> LRU<K, T> for VecLRU<K, T> {
  type List = veclist::VectorLinkedList<(K, T)>;

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


#[cfg(test)]
mod tests {
  use super::VecLRU;
  use super::LRU;

  #[test]
  fn it_works() {
    let mut lru: VecLRU<&str, i32> = VecLRU::new(3);

    assert_eq!(lru.get(&"Hello"), None);

    lru.put("Hello", 1);
    lru.put("Amy", 2);
    lru.put("Santiago", 3);

    assert_eq!(lru.get("Hello").unwrap(), 1);
    assert_eq!(lru.get("Amy").unwrap(), 2);
    assert_eq!(lru.get("Santiago").unwrap(), 3);

    // Removes correct ones from cache
    lru.put("Buster 1", 4);
    assert_eq!(lru.get("Hello"), None);
    lru.put("Buster 2", 5);
    lru.put("Buster 3", 6);
    assert_eq!(lru.get("Amy"), None);
    assert_eq!(lru.get("Santiago"), None);

    // LRU functionality works
    assert_eq!(lru.get("Buster 1").unwrap(), 4);
    // Least recently used is now Buster 2, which should have been removed
    lru.put("Bla Bla", 10);
    assert_eq!(lru.get("Buster 1").unwrap(), 4);
    assert_eq!(lru.get("Buster 2"), None);
  }
}
