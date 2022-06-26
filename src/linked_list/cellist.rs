use crate::linked_list::DLL;
use std::marker::PhantomData;
use std::cell::RefCell;
use std::fmt::{Debug, Error, Formatter, Display};
use std::mem;
use std::fmt;
use std::rc::{Rc, Weak};

pub struct BodyNode<T> {
  elem: T,
  next: StrongNodePointer<T>,
  prev: WeakNodePointer<T>,
}

impl<T: Debug> Debug for BodyNode<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    f.debug_struct("BodyNode")
      .field("elem", &self.elem)
      .finish()
  }
}

type StrongNodePointer<T> = Option<Rc<RefCell<BodyNode<T>>>>;
type WeakNodePointer<T> = Option<Weak<RefCell<BodyNode<T>>>>;

pub struct CellLinkedList<T> {
  head: StrongNodePointer<T>,
  tail: StrongNodePointer<T>,
  size: usize,
}

fn convert_weak<T>(weak_ptr: &WeakNodePointer<T>) -> StrongNodePointer<T> {
  let result = weak_ptr.clone().map(|t| t.upgrade());
  if let Some(None) = result {
    return None;
  }

  result.map(|t| t.unwrap())
}

impl<T> CellLinkedList<T> {
  pub fn new() -> Self {
    CellLinkedList {
      head: None,
      tail: None,
      size: 0,
    }
  }

  fn insert_after(
    &mut self,
    elem: T,
    n: &StrongNodePointer<T>,
  ) -> WeakNodePointer<T> {
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
        new_node_ptr.borrow_mut().prev = n.as_ref().map(|t| Rc::downgrade(t));
      },
    }

    
    Some(Rc::downgrade(&new_node_ptr))
  }

  fn remove(
    p: &mut StrongNodePointer<T>,
    h: &mut StrongNodePointer<T>,
    t: &mut StrongNodePointer<T>,
    size: &mut usize,
  ) -> T {
    *size -= 1;

    let ptr = match p {
      None => panic!("DO NOT DO THIS"),
      Some(i) => i,
    };

    println!("BEFORE REMOVAL {}", Rc::strong_count(ptr));

    let prior_ptr = ptr.borrow().prev.as_ref().map(|ptr| Weak::clone(ptr));
    let next_ptr = ptr.borrow().next.as_ref().map(|ptr| Rc::clone(ptr));

    if let Some(p_ptr) = &prior_ptr {
      p_ptr.upgrade().unwrap().borrow_mut().next = next_ptr.as_ref().map(|p| Rc::clone(p));
    } else {
      *h = next_ptr.as_ref().map(|p| Rc::clone(p));
    }

    if let Some(n_ptr) = &next_ptr {
      n_ptr.borrow_mut().prev = prior_ptr;
    } else {
      *t = prior_ptr.map(|ptr| ptr.upgrade().unwrap());
    }

    println!("AFTER REASSOCIATION {}", Rc::strong_count(ptr));

    ptr.borrow_mut().next = None;
    ptr.borrow_mut().prev = None;
    let curr_ptr = mem::replace(p, None).unwrap();

    println!("AFTER NULLIFYING POINTERS {}", Rc::strong_count(&curr_ptr));
    match Rc::try_unwrap(curr_ptr) {
      Ok(ref_cell) => ref_cell.into_inner().elem,
      _ => panic!("PANIC"),
    }
  }
}

impl<T> DLL<T> for CellLinkedList<T> {
  type Pointer = WeakNodePointer<T>;

  fn size(&self) -> usize {
    self.size
  }

  fn get(&self, weak_ptr: &Self::Pointer) -> Option<&T> {
    let ptr = convert_weak(weak_ptr);

    match ptr {
      None => None,
      Some(n) => {
        unsafe {
          Some(&(*n.as_ptr()).elem)
        }
      }
    }
  }

  fn get_mut(&mut self, weak_ptr: &Self::Pointer) -> Option<&mut T> {
    let ptr = convert_weak(weak_ptr);

    match ptr {
      None => None,
      Some(n) => {
        unsafe {
          Some(&mut (*n.as_ptr()).elem)
        }
      }
    }
  }

  fn replace_val(&mut self, ptr: &Self::Pointer, elem: T) {
    let n = match ptr {
      None => panic!("DO NOT DO THIS"),
      Some(i) => i,
    };

    if let Some(f) = self.get_mut(ptr) {
      *f = elem;
    }
  }

  fn push_back(&mut self, elem: T) -> Self::Pointer {
    let tail = self.tail.as_ref().map(|ptr| Rc::clone(ptr));
    self.insert_after(elem, &tail)
  }

  fn push_front(&mut self, elem: T) -> Self::Pointer {
    self.insert_after(elem, &None)
  }

  fn pop_front(&mut self) -> Option<T> {
    let head = &mut self.head;
    if let None = head {
      return None;
    }
    if let Some(h) = head {
      println!("BEFORE POP {}", Rc::strong_count(h));
    }
    Some(Self::remove(&mut head.clone(), head, &mut self.tail, &mut self.size))
  }

  fn pop_back(&mut self) -> Option<T> {
    let tail = &mut self.tail;
    if let None = tail {
      return None;
    }
    Some(Self::remove(&mut tail.clone(), &mut self.head, tail, &mut self.size))
  }

  fn peek_front(&self) -> Option<&T> {
    if let None = self.head {
      return None;
    }

    self.get(&self.head.as_ref().map(|t| Rc::downgrade(t)))
  }

  fn peek_back(&self) -> Option<&T> {
    if let None = self.tail {
      return None;
    }

    self.get(&self.tail.as_ref().map(|t| Rc::downgrade(t)))
  }

  fn move_back(&mut self, n: &mut Self::Pointer) {
    let elem = Self::remove(
      &mut convert_weak(n),
      &mut self.head,
      &mut self.tail,
      &mut self.size,
    );
    let new_ptr = self.push_back(elem);
    *n = new_ptr;
  }

  fn move_front(&mut self, n: &mut Self::Pointer) {
    let elem = Self::remove(
      &mut convert_weak(n),
      &mut self.head,
      &mut self.tail,
      &mut self.size
    );
    let new_ptr = self.push_front(elem);
    *n = new_ptr;
  }

  fn next_node(&self, weak_ptr: &Self::Pointer) -> Option<Self::Pointer> {
    let ptr = convert_weak(weak_ptr);
    if let Some(p) = ptr {
      let next = &p.borrow().next;
      if let None = next {
        return None;
      }

      Some(next.as_ref().map(|ptr| Rc::downgrade(ptr)))
    } else {
      panic!("Should not happen")
    }
  }

  fn prev_node(&self, weak_ptr: &Self::Pointer) -> Option<Self::Pointer> {
    let ptr = convert_weak(weak_ptr);
    if let Some(p) = ptr {
      match &p.borrow().prev {
        None => None,
        Some(p_ptr) => Some(Some(Weak::clone(p_ptr))),
      }
    } else {
      panic!("NOOOOO!!")
    }
  }

  fn head(&self) -> Option<Self::Pointer> {
    self.head.as_ref().map(|ptr| Some(Rc::downgrade(ptr)))
  }

  fn tail(&self) -> Option<Self::Pointer> {
    self.tail.as_ref().map(|ptr| Some(Rc::downgrade(ptr)))
  }
}


impl<T> IntoIterator for CellLinkedList<T> {
  type Item = T;
  type IntoIter = super::DLLIntoIter<T, Self>;
  fn into_iter(self) -> Self::IntoIter {
    super::DLLIntoIter {
      _wokka: PhantomData,
      list: self,
    }
  }
}

impl<T: Display + Debug> Display for CellLinkedList<T> {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    let mut vec: Vec<String> = Vec::with_capacity(self.size());
    let mut node = self.head();

    while let Some(n_ptr) = node.clone() {
      node = self.next_node(&n_ptr);
      let curr_val = self.get(&n_ptr).unwrap();
      let last_val = convert_weak(&n_ptr).unwrap().borrow().prev.clone().map(|t| { 
        self.get(&Some(t)).unwrap()
      });
      vec.push(format!("[{:?} <--- {}]", last_val, curr_val));
    }

    write!(f, "{}", vec.join(" <---> "))
  }
}

crate::linked_list::macros::dll_tests!(CellLinkedList);