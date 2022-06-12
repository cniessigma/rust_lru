use std::collections::HashMap;
use std::hash::Hash;
use crate::dll::DLL;
use crate::veclist;

pub trait LRU<K: Eq + Hash + Copy, T: Copy> {
  type List: crate::dll::DLL<(K, T)>;

  fn get(&mut self, key: K) -> Option<T> {
    let table = self.hash_table();
    let ptr = match table.get(&key) {
      Some(p) => *p,
      None => { return None }
    };

    let list = self.linked_list();

    match list.get(ptr) {
      None => { return None }
      Some((_, elem)) => {
        list.move_back(ptr);
        Some(elem)
      }
    }
  }

  fn put(&mut self, key: K, val: T) {
    let list = self.linked_list();
    
    if list.size() == list.capacity() {
      match list.pop_front() {
        Some((key, _)) => self.hash_table().remove(&key),
        None => panic!("SIZE MAKES NO SENSE") 
      };
    }

    match self.hash_table().get(&key) {
      // Entry exists! Replace it, THEN move it back
      Some(&ptr) => {
        self.linked_list().replace_val(ptr, (key, val));
        self.linked_list().move_back(ptr);
      },

      // New entry! Push value to back of the list
      None => {
        let new_ptr = self.linked_list().push_back((key, val));
        self.hash_table().insert(key, new_ptr.unwrap());
      }
    };
  }

  fn size(&self) -> usize;
  fn capacity(&self) -> usize;
  fn hash_table(&mut self) -> &mut HashMap<K, <Self::List as DLL<(K, T)>>::Pointer>;
  fn linked_list(&mut self) -> &mut Self::List;
}

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
    lru.put("Salud", 2);
    lru.put("Sentiego", 3);

    assert_eq!(lru.get("Hello").unwrap(), 1);
    assert_eq!(lru.get("Salud").unwrap(), 2);
    assert_eq!(lru.get("Sentiego").unwrap(), 3);

    // Removes correct ones from cache
    lru.put("Buster 1", 4);
    assert_eq!(lru.get("Hello"), None);
    lru.put("Buster 2", 5);
    lru.put("Buster 3", 6);
    assert_eq!(lru.get("Salud"), None);
    assert_eq!(lru.get("Sentiego"), None);

    // LRU functionality works
    assert_eq!(lru.get("Buster 1").unwrap(), 4);
    // Least recently used is now Buster 2, which should have been removed
    lru.put("Bla Bla", 10);
    assert_eq!(lru.get("Buster 1").unwrap(), 4);
    assert_eq!(lru.get("Buster 2"), None);
  }
}


