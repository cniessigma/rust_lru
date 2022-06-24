use std::collections::HashMap;
use std::hash::Hash;
use crate::linked_list::DLL;

pub mod veclru;

pub trait LRU<K, T>
where K: Eq + Hash + Copy {
  type List: DLL<(K, T)>;

  fn new(capacity: usize) -> Self;

  fn get<'a>(&'a mut self, key: &'a K) -> Option<&'_ T> {
    let table = self.hash_table();
    let ptr = match table.get(&key) {
      Some(p) => p.clone(),
      None => { return None }
    };

    let list = self.linked_list();
    list.move_back(&ptr);

    if let Some(tup) = list.get(&ptr) {
      return Some(&tup.1)
    }

    None
  }

  fn put(&mut self, key: K, val: T) {
    if *self.size() == self.capacity() {
      match self.linked_list().pop_front() {
        Some((key, _)) => {
          let size = self.size();
          *size -= 1;
          self.hash_table().remove(&key)
        },
        None => panic!("SIZE MAKES NO SENSE") 
      };
    }

    let existing = self.hash_table().get(&key);

    // I have to keep using the "linked_list" and "hash_table"
    // getters, because self can only have one mutable
    // reference at a time, and both of those data structures are
    // mutable. This seems clunkier than assigning to 
    // a variable, but I don't know if there is any way around it?
    // Re-using the "list" variable above after calling "self.hash_table"
    // does not work.
    // If I had access to the underlying struct then that would work
    // too but... I want to keep this logic in the trait.
    match existing {
      // Entry exists! Replace it, THEN move it back
      Some(ptr) => {
        let new_ptr = ptr.clone();
        self.linked_list().replace_val(&new_ptr, (key, val));
        self.linked_list().move_back(&new_ptr);
      },

      // New entry! Push value to back of the list
      None => {
        let new_ptr = self.linked_list().push_back((key, val));
        let size = self.size();
        *size += 1;
        self.hash_table().insert(key, new_ptr);
      }
    };
  }

  fn size(& mut self) -> &mut usize;
  fn capacity(&self) -> usize;

  // I don't know why I need to do all the disambiguation below...
  // I want to just do Self::List::Pointer, that should only refer to
  // one type, so I don't know why it's freaking out on me.
  fn hash_table(&mut self) -> &mut HashMap<K, <Self::List as DLL<(K, T)>>::Pointer>;
  fn linked_list(&mut self) -> &mut Self::List;
}

#[macro_use]
mod macros {
  macro_rules! lru_tests {
    ($type:ident) => {
      #[cfg(test)]
      mod test {
        use super::*;

        #[test]
        fn test() {
          let mut lru: $type<&str, i32>;
          lru = $type::new(3);
        
          assert_eq!(lru.get(&"Hello"), None);
        
          lru.put(&"Hello", 1);
          lru.put(&"Amy", 2);
          lru.put(&"Santiago", 3);
        
          assert_eq!(lru.get(&"Hello").unwrap(), &1);
          assert_eq!(lru.get(&"Amy").unwrap(), &2);
          assert_eq!(lru.get(&"Santiago").unwrap(), &3);
        
          // Removes correct ones from cache
          lru.put(&"Buster 1", 4);
          assert_eq!(lru.get(&"Hello"), None);
          lru.put(&"Buster 2", 5);
          lru.put(&"Buster 3", 6);
          assert_eq!(lru.get(&"Amy"), None);
          assert_eq!(lru.get(&"Santiago"), None);
        
          // LRU functionality works
          assert_eq!(lru.get(&"Buster 1").unwrap(), &4);
          // Least recently used is now Buster 2, which should have been removed
          lru.put(&"Bla Bla", 10);
          assert_eq!(lru.get(&"Buster 1").unwrap(), &4);
          assert_eq!(lru.get(&"Buster 2"), None);


          let mut other_lru: $type<i32, i32>;
          other_lru = $type::new(3);

          let mut a : i32 = 10;
          other_lru.put(a, 10);
          other_lru.put(a, 11);
          other_lru.put(a, 12);
          other_lru.put(11, 100);
          a += 1;
          assert_eq!(other_lru.get(&a), Some(&100));
          assert_eq!(other_lru.get(&10), Some(&12));
        }
      }
    }
  }
  
  pub(crate) use lru_tests;
}