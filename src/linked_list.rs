pub mod veclist;
use std::marker::PhantomData;

pub trait DLL<T: Clone + Copy> {
  type Pointer: Clone + Copy;

  fn size(&self) -> usize;
  fn get(&self, ptr: Self::Pointer) -> Option<&T>;
  fn replace_val(&mut self, ptr: Self::Pointer, elem: T) -> Option<Self::Pointer>;
  fn peek_front(&self) -> Option<&T>;
  fn pop_front(&mut self) -> Option<T>;

  fn push_back(&mut self, elem: T) -> Self::Pointer;
  fn move_back(&mut self, ptr: Self::Pointer) -> Self::Pointer;

  fn next_node(&self, prt: Self::Pointer) -> Option<Self::Pointer>;
  fn prev_node(&self, prt: Self::Pointer) -> Option<Self::Pointer>;

  fn head(&self) -> Option<Self::Pointer>;

  // fn iter(&self) -> DLLIterator<'_, T, Self>;
}

pub struct DLLIterator<'a, T, L>
where T: 'a + Copy, L: DLL<T>
{
  list: &'a L,
  curr: Option<L::Pointer>,
  wokka: PhantomData<T>,
}

impl<'a, T, L> Iterator for DLLIterator<'a, T, L>
where T: 'a + Copy, L: DLL<T>
{
  type Item = &'a T;
  fn next(&mut self) -> Option<Self::Item> {
    let curr_ptr = self.curr;

    if let None = curr_ptr {
      return None;
    }

    let next_node = self.list.next_node(self.curr.unwrap());
    self.curr = next_node;
    

    self.list.get(curr_ptr.unwrap())
  }
}

#[macro_use]
mod macros {
  macro_rules! dll_tests {
    ($type:ident) => {
      #[cfg(test)]
      mod test {
        use super::*;
        use std::marker::PhantomData;

        #[test]
        fn test() {
          let mut l: $type<i32> = $type::new();
          assert_eq!(l.size(), 0);
          let first = l.push_back(100);
          assert_eq!(l.size(), 1);
          let second = l.push_back(-1);
          assert_eq!(l.size(), 2);
          let third = l.push_back(20);
          assert_eq!(l.size(), 3);

          // Can be got, with a pointer
          assert_eq!(l.get(first), Some(&100));
          assert_eq!(l.get(second), Some(&-1));
          assert_eq!(l.get(third), Some(&20));

          //Can remove
          assert_eq!(l.peek_front(), Some(&100));
          assert_eq!(l.pop_front(), Some(100));

          assert_eq!(l.size(), 2);

          assert_eq!(l.peek_front(), Some(&-1));
          assert_eq!(l.pop_front(), Some(-1));

          assert_eq!(l.size(), 1);

          assert_eq!(l.peek_front(), Some(&20));
          assert_eq!(l.pop_front(), Some(20));

          assert_eq!(l.size(), 0);

          assert_eq!(l.peek_front(), None);
          assert_eq!(l.pop_front(), None);

          assert_eq!(l.size(), 0);

          l.push_back(10);
          assert_eq!(l.peek_front(), Some(&10));
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
          assert_eq!(l.peek_front(), Some(&10));
          l.replace_val(ptr, 40);
          assert_eq!(l.peek_front(), Some(&40));
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

          assert_eq!(l.get(ptr2), Some(&200));
          assert_eq!(l.get(ptr3), Some(&300));
          assert_eq!(l.next_node(ptr3), None);
          l.push_back(400);
          assert_eq!(l.get(l.next_node(ptr3).unwrap()), Some(&400));
          assert_eq!(l.get(ptr2_again), Some(&200));
          assert_eq!(l.get(ptr1_again), Some(&100));
          assert_eq!(l.prev_node(ptr1_again), None);

          let iter = crate::linked_list::DLLIterator {
            list: &l,
            curr: l.head(),
            wokka: PhantomData
          };

          for (i, n) in Box::new(iter).enumerate() {
            println!("{i} {n}");
            assert_eq!(*n, (i as i32 + 1) * 100);
          }
        }
      }
    }
  }
  
  pub(crate) use dll_tests;
}