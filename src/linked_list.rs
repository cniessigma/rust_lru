pub mod veclist;

pub trait DLL<T: Clone + Copy> {
  type Pointer: Clone + Copy;

  fn size(&self) -> usize;
  fn get(&self, ptr: Self::Pointer) -> Option<T>;
  fn replace_val(&mut self, ptr: Self::Pointer, elem: T) -> Option<Self::Pointer>;
  fn peek_front(&self) -> Option<T>;
  fn pop_front(&mut self) -> Option<T>;

  fn push_back(&mut self, elem: T) -> Self::Pointer;
  fn move_back(&mut self, ptr: Self::Pointer) -> Self::Pointer;

  fn next_node(&self, prt: Self::Pointer) -> Option<Self::Pointer>;
  fn prev_node(&self, prt: Self::Pointer) -> Option<Self::Pointer>;
}

// pub struct DLLIterator<T: Copy, L: DLL<T>> {
//   list: L,
// }

#[macro_use]
mod macros {
  macro_rules! dll_tests {
    ($type:ident) => {
      #[cfg(test)]
      mod test {
        use super::*;

        #[test]
        fn test() {
          let mut l = $type::new();
          assert_eq!(l.size(), 0);
          let first = l.push_back(100);
          assert_eq!(l.size(), 1);
          let second = l.push_back(-1);
          assert_eq!(l.size(), 2);
          let third = l.push_back(20);
          assert_eq!(l.size(), 3);

          // Can be got, with a pointer
          assert_eq!(l.get(first), Some(100));
          assert_eq!(l.get(second), Some(-1));
          assert_eq!(l.get(third), Some(20));

          //Can remove
          assert_eq!(l.peek_front(), Some(100));
          assert_eq!(l.pop_front(), Some(100));

          assert_eq!(l.size(), 2);

          assert_eq!(l.peek_front(), Some(-1));
          assert_eq!(l.pop_front(), Some(-1));

          assert_eq!(l.size(), 1);

          assert_eq!(l.peek_front(), Some(20));
          assert_eq!(l.pop_front(), Some(20));

          assert_eq!(l.size(), 0);

          assert_eq!(l.peek_front(), None);
          assert_eq!(l.pop_front(), None);

          assert_eq!(l.size(), 0);

          l.push_back(10);
          assert_eq!(l.peek_front(), Some(10));
          assert_eq!(l.pop_front(), Some(10));
          assert_eq!(l.pop_front(), None);

          // Can re-arrange
          l.push_back(1);
          let ptr = l.push_back(3);
          l.push_back(2);
          l.move_back(ptr);

          assert_eq!(l.pop_front(), Some(1));
          assert_eq!(l.pop_front(), Some(2));
          assert_eq!(l.pop_front(), Some(3));

          // Can replace value at a pointer.
          let ptr = l.push_back(10);
          assert_eq!(l.peek_front(), Some(10));
          l.replace_val(ptr, 40);
          assert_eq!(l.peek_front(), Some(40));
          l.replace_val(ptr, 100);
          assert_eq!(l.pop_front(), Some(100));

          // Can grab the next Pointer and get the next entry
          let ptr1 = l.push_back(100);
          l.push_back(200);
          l.push_back(300);
          let ptr2 = l.next_node(ptr1).unwrap();
          let ptr3 = l.next_node(ptr2).unwrap();
          let ptr2_again = l.prev_node(ptr3).unwrap();
          let ptr1_again = l.prev_node(ptr2_again).unwrap();

          assert_eq!(l.get(ptr2), Some(200));
          assert_eq!(l.get(ptr3), Some(300));
          assert_eq!(l.next_node(ptr3), None);
          l.push_back(400);
          assert_eq!(l.get(l.next_node(ptr3).unwrap()), Some(400));
          assert_eq!(l.get(ptr2_again), Some(200));
          assert_eq!(l.get(ptr1_again), Some(100));
          assert_eq!(l.prev_node(ptr1_again), None);
        }
      }
    }
  }
  
  pub(crate) use dll_tests;
}