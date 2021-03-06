pub mod veclist;
pub mod cellist;
use std::marker::PhantomData;

pub trait DLL<T> {
  type Pointer;

  // How many are in the list?
  fn new() -> Self;
  fn size(&self) -> usize;

  fn peek_front(&self) -> Option<&T>;
  fn peek_back(&self) -> Option<&T>;
  fn pop_front(&mut self) -> Option<T>;
  fn pop_back(&mut self) -> Option<T>;

  fn head(&self) -> Option<Self::Pointer>;
  fn tail(&self) -> Option<Self::Pointer>;

  fn get(&self, ptr: &Self::Pointer) -> Option<&T>;
  fn get_mut(&mut self, ptr: &Self::Pointer) -> Option<&mut T>;
  fn replace_val(&mut self, ptr: &Self::Pointer, elem: T);

  fn push_back(&mut self, elem: T) -> Self::Pointer;
  fn push_front(&mut self, elem: T) -> Self::Pointer;
  fn move_back(&mut self, ptr: &mut Self::Pointer);
  fn move_front(&mut self, ptr: &mut Self::Pointer);

  // Traversers, so that iter can use it.
  fn next_node(&self, ptr: &Self::Pointer) -> Option<Self::Pointer>;
  fn prev_node(&self, ptr: &Self::Pointer) -> Option<Self::Pointer>;

  fn iter(&self) -> DLLIterator<T, Self> {
    DLLIterator {
      list: &self,
      curr: self.head(),
      wokka: PhantomData,
    }
  }

  fn iter_mut(&mut self) -> DLLMutIterator<T, Self> {
    DLLMutIterator {
      curr: self.head(),
      list: self,
      _wokka: PhantomData,
    }
  }
}

pub struct DLLIntoIter<T, L: DLL<T>> {
  list: L,
  _wokka: PhantomData<T>,
}

impl<T, L: DLL<T>> Iterator for DLLIntoIter<T, L> {
  type Item = T;
  fn next(&mut self) -> Option<Self::Item> {
    self.list.pop_front()
  }
}

pub struct DLLIterator<'a, T, L>
where T: 'a, L: DLL<T> + ?Sized
{
  list: &'a L,
  curr: Option<L::Pointer>,
  wokka: PhantomData<T>,
}

impl<'a, T, L> Iterator for DLLIterator<'a, T, L>
where L: DLL<T>
{
  type Item = &'a T;
  fn next(&mut self) -> Option<Self::Item> {
    let curr_ptr = self.curr.as_ref();

    if let None = curr_ptr {
      return None;
    }

    let next_node = self.list.next_node(curr_ptr.unwrap());
    let item = self.list.get(curr_ptr.as_ref().clone().unwrap());
    self.curr = next_node;
    item
  }
}

pub struct DLLMutIterator<'a, T, L>
where T: 'a, L: DLL<T> + ?Sized
{
  list: &'a mut L,
  curr: Option<L::Pointer>,
  _wokka: PhantomData<&'a T>,
}

impl<'a, T, L> Iterator for DLLMutIterator<'a, T, L>
where L: DLL<T>
{
  type Item = &'a mut T;
  fn next(&mut self) -> Option<Self::Item> {
    let curr_ptr = self.curr.as_ref();

    if let None = &curr_ptr {
      return None;
    }

    let next_node = self.list.next_node(curr_ptr.unwrap());

    // The problem is the mutable reference is moved to this
    // function once we grab it, and we can't return it here
    // because we want to be able to call next again. Rust
    // is deadly afraid of you returning the same &mut twice.
    let output = self.list.get_mut(&curr_ptr.unwrap());
    self.curr = next_node;
    
    unsafe {
      // But since I know they are different every time, let's ignore it
      // and de-reference.
      output.map(|n| &mut *(n as *mut T))
    }
  }
}

#[macro_use]
mod macros {
  macro_rules! dll_tests {
    ($type:ident) => {
      #[cfg(test)]
      mod test {
        use super::*;

        #[test]
        fn mut_test() {
          let mut l = $type::new();
          println!("{l}");

          l.push_back(10);
          l.push_back(20);
          l.push_back(30);

          println!("{l}");

          let iter = l.iter_mut();

          for i in iter {
            *i = 100 + *i;
          }

          for (i, n) in l.into_iter().enumerate() {
            assert_eq!(n, 100 + (i + 1) * 10)
          }
        }

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
          assert_eq!(l.get(&first), Some(&100));
          assert_eq!(l.get(&second), Some(&-1));
          assert_eq!(l.get(&third), Some(&20));
        
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
          let mut ptr = l.push_back(3);
          l.push_back(2);
          l.move_back(&mut ptr);

          assert_eq!(l.get(&l.head().unwrap()), l.peek_front());
          assert_eq!(l.get(&l.tail().unwrap()), l.peek_back());
        
          assert_eq!(l.pop_front(), Some(1));
          assert_eq!(l.pop_back(), Some(3));
          assert_eq!(l.pop_front(), Some(2));
        
          // Can replace value at a pointer.
          let ptr = l.push_back(10);
          assert_eq!(l.peek_front(), Some(&10));
          assert_eq!(l.peek_back(), Some(&10));
          l.replace_val(&ptr, 40);
          assert_eq!(l.peek_front(), Some(&40));
          assert_eq!(l.peek_back(), Some(&40));
          l.replace_val(&ptr, 100);
          assert_eq!(l.pop_front(), Some(100));
          assert_eq!(l.pop_back(), None);
        
          // Can grab the next Pointer and get the next entry
          let mut ptr1 = l.push_back(100);
          l.push_back(200);
          l.push_back(300);
          let ptr2 = l.next_node(&ptr1).unwrap();
          let ptr3 = l.next_node(&ptr2).unwrap();
          let ptr2_again = l.prev_node(&ptr3).unwrap();
          let ptr1_again = l.prev_node(&ptr2_again).unwrap();
        
          assert_eq!(l.get(&ptr2), Some(&200));
          assert_eq!(l.get(&ptr3), Some(&300));
          assert_eq!(l.next_node(&ptr3).is_none(), true);
          l.push_back(400);
          let mut ptr4 = l.next_node(&ptr3).unwrap();
          assert_eq!(l.get(&ptr4), Some(&400));
          assert_eq!(l.get(&ptr2_again), Some(&200));
          assert_eq!(l.get(&ptr1_again), Some(&100));
          assert_eq!(l.prev_node(&ptr1_again).is_none(), true);
        
          l.move_back(&mut ptr1);
          l.move_front(&mut ptr4);
        
          assert_eq!(l.get(&l.head().unwrap()), Some(&400));
          assert_eq!(l.get(&l.tail().unwrap()), Some(&100));
          l.move_front(&mut ptr1);
          l.move_back(&mut ptr4);

          // Iterating works for &'s
          for (i, n) in l.iter().enumerate() {
            assert_eq!(*n, (i as i32 + 1) * 100);
          }
        }
      }
    }
  }
  
  pub(crate) use dll_tests;
}