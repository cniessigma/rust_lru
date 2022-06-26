use std::collections::HashMap;
use std::hash::Hash;
use crate::linked_list::DLL;
use std::marker::PhantomData;

pub mod veclru;
pub mod cellru;


pub struct KeyHolder<K: Eq + Hash + Copy, T, L: DLL<(K, T)>> {
  list: L,
  hash: HashMap<K, L::Pointer>,
  size: usize,
  capacity: usize,
  _marker: PhantomData<T>,
}

pub trait LRU<K, T>
where K: Eq + Hash + Copy {
  type List: DLL<(K, T)>;

  fn new(capacity: usize) -> Self;

  fn key_holder(&mut self) -> &mut KeyHolder<K, T, Self::List>;

  fn get<'a>(&'a mut self, key: &'a K) -> Option<&'_ T> {
    let holder = self.key_holder();
    let mut ptr = match holder.hash.get_mut(&key) {
      Some(p) => p,
      None => { return None }
    };

    holder.list.move_back(&mut ptr);

    if let Some(tup) = holder.list.get(ptr) {
      return Some(&tup.1)
    }

    None
  }

  fn put(&mut self, key: K, val: T) {
    let holder = self.key_holder();
    if holder.size == holder.capacity {
      match holder.list.pop_front() {
        Some((key, _)) => {
          holder.size -= 1;
          holder.hash.remove(&key)
        },
        None => panic!("SIZE MAKES NO SENSE") 
      };
    }

    let existing = holder.hash.get_mut(&key);

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
      Some(mut ptr) => {
        holder.list.replace_val(&ptr, (key, val));
        holder.list.move_back(&mut ptr);
      },

      // New entry! Push value to back of the list
      None => {
        let new_ptr = holder.list.push_back((key, val));
        holder.size += 1;
        holder.hash.insert(key, new_ptr);
      }
    };
  }
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