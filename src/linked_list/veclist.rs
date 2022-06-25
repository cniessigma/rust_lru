use std::mem;
use std::fmt;
use crate::linked_list::DLL;
use std::marker::PhantomData;


struct BodyNode<T> {
  elem: T,
  next: NodePointer,
  prev: NodePointer,
}

struct HeadNode { next: NodePointer }
struct TailNode { prev: NodePointer }

pub struct VectorLinkedList<T> {
  spine: Vec<Option<BodyNode<T>>>,
  size: usize,
  next_insert: usize,
  head: HeadNode,
  tail: TailNode,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NodePointer {
  Head,
  Tail,
  Body(usize),
}

impl<T> VectorLinkedList<T> {
  pub fn new() -> Self {
    Self {
      spine: Vec::new(),
      size: 0,
      next_insert: 0,
      head: HeadNode { next: NodePointer::Tail },
      tail: TailNode { prev: NodePointer::Head },
    }
  }

  fn find_next(&self) -> usize {
    for (i, _) in self.spine.iter().enumerate() {
      match self.spine[i] {
        None => { return i }
        _ => {}
      }
    }

    return self.spine.len();
  }

  fn insert_between(&mut self, elem: T, p: &NodePointer, n: &NodePointer) -> NodePointer {
    let new_node = BodyNode {
      elem: elem, next: n.clone(), prev: p.clone(),
    };

    // If our insert node is within the bounds of the array
    let insert_at = self.next_insert;

    if insert_at < self.spine.len() {
      self.spine[insert_at] = Some(new_node);
    } else {
      self.spine.push(Some(new_node));
    }

    match *n {
      NodePointer::Head => panic!("Head cannot be referred to by next"),
      NodePointer::Tail => self.tail.prev = NodePointer::Body(insert_at),
      NodePointer::Body(next) => {
        match &mut self.spine[next] {
          None => panic!("I hate this"),
          Some(node) => node.prev = NodePointer::Body(insert_at),
        }
      }
    }

    match *p {
      NodePointer::Tail => panic!("Tail cannot be referred to by prev"),
      NodePointer::Head => self.head.next = NodePointer::Body(insert_at),
      NodePointer::Body(prev) => {
        match &mut self.spine[prev] {
          None => panic!("I hate this"),
          Some(node) => node.next = NodePointer::Body(insert_at),
        }
      }
    }

    self.next_insert = self.find_next();
    self.size += 1;
    NodePointer::Body(insert_at)
  }

  fn remove(&mut self, n: &NodePointer) -> Option<T> {
    let vec_index = match *n {
      NodePointer::Body(i) => i,
      _ => return None
    };

    let existing_node = match mem::replace(&mut self.spine[vec_index], None) {
      None => return None,
      Some(node) => node,
    };

    // Free up space in the vector array
    self.next_insert = vec_index;
    self.size -= 1;

    match existing_node.next {
      NodePointer::Head => panic!("Head cannot be referred to by next"),
      NodePointer::Tail => self.tail.prev = existing_node.prev,
      NodePointer::Body(next) => {
        match &mut self.spine[next] {
          None => panic!("I hate this"),
          Some(node) => node.prev = existing_node.prev,
        }
      }
    }

    match existing_node.prev {
      NodePointer::Tail => panic!("Tail cannot be referred to by prev"),
      NodePointer::Head => self.head.next = existing_node.next,
      NodePointer::Body(prev) => {
        match &mut self.spine[prev] {
          None => panic!("I hate this"),
          Some(node) => node.next = existing_node.next,
        }
      }
    }

    return Some(existing_node.elem);
  }
}

impl<T> DLL<T> for VectorLinkedList<T> {
  type Pointer = NodePointer;

  fn size(&self) -> usize {
    self.size
  }

  fn get(&self, n: &NodePointer) -> Option<&T> {
    match *n {
      NodePointer::Body(i) => self.spine[i].as_ref().map(|node| &node.elem),
      _ => None
    }
  }

  fn get_mut(&mut self, n: &NodePointer) -> Option<&mut T> {
    match *n {
      NodePointer::Body(i) => self.spine[i].as_mut().map(|node| &mut node.elem),
      _ => None
    }
  }

  fn replace_val(&mut self, n: &NodePointer, elem: T) {
    match *n {
      NodePointer::Body(i) => {
        match &self.spine[i] {
          Some(curr_node) => {
            self.spine[i] = Some(
              BodyNode {
                elem: elem,
                ..*curr_node
              }
            );
            Some(*n)
          },
          None => None,
        }
      },
      _ => None
    };
  }

  fn push_back(&mut self, elem: T) -> NodePointer {
    return self.insert_between(elem, &self.tail.prev.clone(), &NodePointer::Tail);
  }

  fn push_front(& mut self, elem: T) -> NodePointer {
    return self.insert_between(elem, &NodePointer::Head, &self.head.next.clone());
  }

  fn pop_front(&mut self) -> Option<T> {
    self.remove(&self.head.next.clone())
  }

  fn pop_back(&mut self) -> Option<T> {
    self.remove(&self.tail.prev.clone())
  }

  fn peek_front(&self) -> Option<&T> {
    self.get(&self.head.next)
  }

  fn peek_back(&self) -> Option<&T> {
    self.get(&self.tail.prev)
  }

  fn move_back(&mut self, n: &mut NodePointer) {
    let mut new_ptr = self.remove(n).map(|elem| self.push_back(elem)).unwrap();
    mem::swap(n, &mut new_ptr);
  }

  fn move_front(&mut self, n: &mut NodePointer) {
    let mut new_ptr = self.remove(n).map(|elem| {
      self.push_front(elem)
    }).unwrap();
    mem::swap(n, &mut new_ptr);
  }

  fn next_node(&self, ptr: &NodePointer) -> Option<NodePointer> {
    match *ptr {
      NodePointer::Body(i) => {
        let next = self.spine[i].as_ref()?.next;

        match next {
          NodePointer::Body(_) => Some(next),
          _ => None
        }
      },
      _ => None
    }
  }

  fn prev_node(&self, ptr: &NodePointer) -> Option<NodePointer> {
    match *ptr {
      NodePointer::Body(i) => {
        let prev = self.spine[i].as_ref()?.prev;

        match prev {
          NodePointer::Body(_) => Some(prev),
          _ => None
        }
      },
      _ => None
    }
  }

  fn head(&self) -> Option<NodePointer> {
    let next = self.head.next;
    match next {
      NodePointer::Body(_) => Some(next),
      _ => None
    }
  }

  fn tail(&self) -> Option<NodePointer> {
    let next = self.tail.prev;
    match next {
      NodePointer::Body(_) => Some(next),
      _ => None
    }
  }
}

impl<T: fmt::Display> fmt::Display for VectorLinkedList<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut vec: Vec<String> = Vec::new();
    let mut node = self.head();


    while let Some(ptr) = node {
      node = self.next_node(&ptr);

      if let NodePointer::Body(index) = ptr {
        let elem = self.get(&ptr).unwrap();
        let prev = match self.prev_node(&ptr) {
          Some(NodePointer::Body(p_i)) => format!("<- {p_i}"),
          _ => String::from("HEAD")
        };
        let next = match node {
          Some(NodePointer::Body(n_i)) => format!("{n_i} ->"),
          _ => String::from("TAIL")
        };

        vec.push(format!("[{prev} ({index} - {elem}) {next}]"));
      }
    }
  
    write!(f, "{}", vec.join(""))
  }
}

impl<T> IntoIterator for VectorLinkedList<T> {
  type Item = T;
  type IntoIter = super::DLLIntoIter<T, Self>;
  fn into_iter(self) -> Self::IntoIter {
    super::DLLIntoIter {
      _wokka: PhantomData,
      list: self,
    }
  }
}

crate::linked_list::macros::dll_tests!(VectorLinkedList);
