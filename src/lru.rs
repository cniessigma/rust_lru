use std::collections::HashMap;
use std::hash::Hash;
use crate::linked_list::DLL;

pub mod veclru;

pub trait LRU<K: Eq + Hash + Copy, T: Copy> {
  type List: DLL<(K, T)>;

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

    // I have to keep using the "linked_list" and "hash_table"
    // getters, because self can only have one mutable
    // reference at a time, and both of those data structures are
    // mutable. This seems clunkier than assigning to 
    // a variable, but I don't know if there is any way around it?
    // Re-using the "list" variable above after calling "self.hash_table"
    // does not work.
    // If I had access to the underlying struct then that would work
    // too but... I want to keep this logic in the trait.
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

  // I don't know why I need to do all the disambiguation below...
  // I want to just do Self::List::Pointer, that should only refer to
  // one type, so I don't know why it's freaking out on me.
  fn hash_table(&mut self) -> &mut HashMap<K, <Self::List as DLL<(K, T)>>::Pointer>;
  fn linked_list(&mut self) -> &mut Self::List;
}

