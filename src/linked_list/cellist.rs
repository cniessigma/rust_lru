use crate::linked_list::DLL;
use std::marker::PhantomData;
use std::cell::RefCell;
use std::fmt::{Debug, Error, Formatter, Display};
use std::fmt;
use std::rc::{Rc, Weak};

#[derive(Clone)]
pub struct BodyNode<T: Clone> {
  elem: T,
  next: Option<StrongNodePointer<T>>,
  prev: Option<WeakNodePointer<T>>,
}

impl<T: Clone + Debug> Debug for BodyNode<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    f.debug_struct("BodyNode")
      .field("elem", &self.elem)
      .finish()
  }
}

// Only owned forwards
type StrongNodePointer<T> = Rc<RefCell<BodyNode<T>>>;
type WeakNodePointer<T> = Weak<RefCell<BodyNode<T>>>;

pub struct CellLinkedList<T: Clone> {
  head: Option<StrongNodePointer<T>>,
  tail: Option<StrongNodePointer<T>>,
  size: usize,
}

impl<T: Clone> CellLinkedList<T> {
  fn new() -> Self {
    CellLinkedList {
      head: None,
      tail: None,
      size: 0,
    }
  }

  fn insert_after(
    &mut self,
    elem: T,
    n: Option<&StrongNodePointer<T>>,
  ) -> StrongNodePointer<T> {
    let new_node = RefCell::new(
      BodyNode {
        elem: elem,
        next: None,
        prev: None,
      }
    );

    let new_node_ptr = Rc::new(new_node);

    self.size += 1;
    match n {
      // Insert at head.
      None => {
        let next_node_ptr = match &self.head {
          Some(ptr) => { 
            ptr.borrow_mut().prev = Some(Rc::downgrade(&new_node_ptr));
            Some(Rc::clone(ptr))
          },
          None => None
        };

        self.head = Some(Rc::clone(&new_node_ptr));
        new_node_ptr.borrow_mut().next = next_node_ptr;

        if self.tail.is_none() {
          self.tail = Some(Rc::clone(&new_node_ptr));
        }
      },

      Some(ptr) => {
        let new_next_node = &ptr.borrow().next.as_ref().map(|ptr| Rc::clone(ptr));
        ptr.borrow_mut().next = Some(new_node_ptr.clone());

        match new_next_node {
          // If there is no next node, then the new node becomes the tail
          None => self.tail = Some(Rc::clone(&new_node_ptr)),

          // If there is a next node, set that node's prev to the new node
          Some(n_ptr) => {
            n_ptr.borrow_mut().prev = Some(Rc::downgrade(&new_node_ptr));
          }
        };


        // Set the new node's next to the cursor node's next 
        new_node_ptr.borrow_mut().next = new_next_node.clone();

        // Set the new node's prev to a weak pointer to the cursor node
        new_node_ptr.borrow_mut().prev = n.map(|t| Rc::downgrade(&t));
      },
    }

    
    new_node_ptr
  }

  fn remove(&mut self, ptr: &StrongNodePointer<T>) -> T {
    self.size -= 1;

    let prior_ptr = ptr.borrow().prev.as_ref().map(|ptr| Weak::clone(ptr));
    let next_ptr = ptr.borrow().next.as_ref().map(|ptr| Rc::clone(ptr));

    if let Some(p_ptr) = &prior_ptr {
      p_ptr.upgrade().unwrap().borrow_mut().next = next_ptr.as_ref().map(|p| Rc::clone(p));
    } else {
      self.head = next_ptr.as_ref().map(|p| Rc::clone(p));
    }

    if let Some(n_ptr) = &next_ptr {
      n_ptr.borrow_mut().prev = prior_ptr;
    } else {
      self.tail = prior_ptr.map(|ptr| ptr.upgrade().unwrap());
    }


    ptr.borrow_mut().next = None;
    ptr.borrow_mut().prev = None;
    ptr.borrow().elem.clone()
  }
}

impl<T: Clone> DLL<T> for CellLinkedList<T> {
  type Pointer = StrongNodePointer<T>;

  fn size(&self) -> usize {
    self.size
  }

  fn get(&self, n: &Self::Pointer) -> Option<&T> {
    unsafe {
      Some(&(*n.as_ptr()).elem)
    }
  }

  fn get_mut(&mut self, n: &Self::Pointer) -> Option<&mut T> {
    unsafe {
      Some(&mut (*n.as_ptr()).elem)
    }
  }

  fn replace_val(&mut self, n: &Self::Pointer, elem: T) -> Option<Self::Pointer> {
    if let Some(f) = self.get_mut(n) {
      *f = elem;
      Some(Rc::clone(n))
    } else {
      None
    }
  }

  fn push_back(&mut self, elem: T) -> Self::Pointer {
    let tail = self.tail.as_ref().map(|ptr| Rc::clone(ptr));
    self.insert_after(elem, tail.as_ref())
  }

  fn push_front(&mut self, elem: T) -> Self::Pointer {
    self.insert_after(elem, None)
  }

  fn pop_front(&mut self) -> Option<T> {
    self.head.clone().map(|head_ptr| self.remove(&head_ptr))
  }

  fn pop_back(&mut self) -> Option<T> {
    self.tail.clone().map(|head_ptr| self.remove(&head_ptr))
  }

  fn peek_front(&self) -> Option<&T> {
    if let None = self.head {
      return None;
    }

    self.get(self.head.as_ref().unwrap())
  }

  fn peek_back(&self) -> Option<&T> {
    if let None = self.tail {
      return None;
    }

    self.get(self.tail.as_ref().unwrap())
  }

  fn move_back(&mut self, n: &Self::Pointer) -> Self::Pointer {
    let elem = self.remove(n);
    self.push_back(elem)
  }

  fn move_front(&mut self, n: &Self::Pointer) -> Self::Pointer {
    let elem = self.remove(n);
    self.push_front(elem)
  }

  fn next_node(&self, ptr: &Self::Pointer) -> Option<Self::Pointer> {
    ptr.borrow().next.as_ref().map(|ptr| Rc::clone(ptr))
  }

  fn prev_node(&self, ptr: &Self::Pointer) -> Option<Self::Pointer> {
    match &ptr.borrow().prev {
      None => None,
      Some(p_ptr) => Some(p_ptr.upgrade()).unwrap(),
    }
  }

  fn head(&self) -> Option<Self::Pointer> {
    self.head.as_ref().map(|ptr| Rc::clone(ptr))
  }

  fn tail(&self) -> Option<Self::Pointer> {
    self.tail.as_ref().map(|ptr| Rc::clone(ptr))
  }
}


impl<T: Clone> IntoIterator for CellLinkedList<T> {
  type Item = T;
  type IntoIter = super::DLLIntoIter<T, Self>;
  fn into_iter(self) -> Self::IntoIter {
    super::DLLIntoIter {
      _wokka: PhantomData,
      list: self,
    }
  }
}

impl<T: Display + Clone + Debug> Display for CellLinkedList<T> {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    let mut vec: Vec<String> = Vec::with_capacity(self.size());
    let mut node = self.head();

    while let Some(n_ptr) = node.clone() {
      node = self.next_node(&n_ptr);


      let curr_val = self.get(&n_ptr).unwrap();
      let last_val = n_ptr.borrow().prev.clone().map(|t| { 
        let strong_last = t.upgrade().unwrap();
        self.get(&strong_last).unwrap()
      });
      vec.push(format!("[{:?} <--- {}]", last_val, curr_val));
    }

    write!(f, "{}", vec.join(" <---> "))
  }
}

crate::linked_list::macros::dll_tests!(CellLinkedList);