use std::mem;
use crate::dll::DLL;

struct BodyNode<T> {
  elem: T,
  next: NodePointer,
  prev: NodePointer,
}

struct HeadNode { next: NodePointer }
struct TailNode { prev: NodePointer }

pub struct VectorLinkedList<T> {
  spine: Vec<Option<BodyNode<T>>>,
  capacity: usize,
  size: usize,
  next_insert: Option<usize>,
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
  pub fn new(capacity: usize) -> Self {
    let mut vec = Vec::new();
    for _ in 0..capacity {
      vec.push(None)
    }

    Self {
      spine: vec,
      size: 0,
      capacity: capacity,
      next_insert: Some(0),
      head: HeadNode { next: NodePointer::Tail },
      tail: TailNode { prev: NodePointer::Head },
    }
  }

  fn find_next(&self) -> Option<usize> {
    match self.next_insert {
      Some(i) if i < self.spine.len() - 1 => {
        match self.spine[i + 1] {
          None => return Some(i + 1),
          _ => {}
        }
      }
      _ => {}
    }

    for (i, _) in self.spine.iter().enumerate() {
      println!("Looping to find next spot");
      match self.spine[i] {
        None => { return Some(i) }
        _ => {}
      }
    }

    return None;
  }

  fn insert_between(&mut self, elem: T, p: NodePointer, n: NodePointer) -> Option<NodePointer> {
    let new_node = BodyNode {
      elem: elem, next: n, prev: p,
    };

    let insert_at = match self.next_insert {
      None => return None,
      Some(i) => {
        self.spine[i] = Some(new_node);
        i
      },
    };

    match n {
      NodePointer::Head => panic!("Head cannot be referred to by next"),
      NodePointer::Tail => self.tail.prev = NodePointer::Body(insert_at),
      NodePointer::Body(next) => {
        match &mut self.spine[next] {
          None => panic!("I hate this"),
          Some(node) => node.prev = NodePointer::Body(insert_at),
        }
      }
    }

    match p {
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
    Some(NodePointer::Body(insert_at))
  }

  fn remove(&mut self, n: NodePointer) -> Option<T> {
    let vec_index = match n {
      NodePointer::Body(i) => i,
      _ => return None
    };

    let existing_node = match mem::replace(& mut self.spine[vec_index], None) {
      None => return None,
      Some(node) => node,
    };

    // Free up space in the vector array
    self.next_insert = Some(vec_index);
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

impl<T: Clone + Copy> DLL<T> for VectorLinkedList<T> {
  type Pointer = NodePointer;

  fn size(&self) -> usize {
    self.size
  }

  fn capacity(&self) -> usize {
    self.capacity
  }

  fn get(&self, n: NodePointer) -> Option<T> {
    match n {
      NodePointer::Body(i) => self.spine[i].as_ref().map(|node| node.elem),
      _ => None
    }
  }

  fn replace_val(&mut self, n: NodePointer, elem: T) -> Option<NodePointer> {
    match n {
      NodePointer::Body(i) => {
        match &self.spine[i] {
          Some(curr_node) => {
            self.spine[i] = Some(
              BodyNode {
                elem: elem,
                ..*curr_node
              }
            );
            Some(n)
          },
          None => None,
        }
      },
      _ => None
    }
  }

  fn push_back(& mut self, elem: T) -> Option<NodePointer> {
    if self.size == self.capacity {
      return None;
    }

    return self.insert_between(elem, self.tail.prev, NodePointer::Tail);
  }

  fn pop_front(&mut self) -> Option<T> {
    self.remove(self.head.next)
  }

  fn peek_front(&self) -> Option<T> {
    self.get(self.head.next)
  }

  fn move_back(&mut self, n: NodePointer) -> Option<NodePointer> {
    self.remove(n).map(|elem| self.push_back(elem)).unwrap()
  }
}

#[cfg(test)]
mod tests {
  use super::DLL;
  #[test]
  fn it_works() {
    let mut l = super::VectorLinkedList::new(3);
    assert_eq!(l.size(), 0);
    let first = l.push_back(100);
    assert_eq!(l.size(), 1);
    let second = l.push_back(-1);
    assert_eq!(l.size(), 2);
    let third = l.push_back(20);
    assert_eq!(l.size(), 3);
    let last = l.push_back(1337);
    assert_eq!(l.size(), 3);

    // Can be got, with a pointer
    assert_eq!(l.get(first.unwrap()), Some(100));
    assert_eq!(l.get(second.unwrap()), Some(-1));
    assert_eq!(l.get(third.unwrap()), Some(20));
    assert_eq!(last, None);

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
    let ptr = l.push_back(3).unwrap();
    l.push_back(2);
    l.move_back(ptr);

    assert_eq!(l.pop_front(), Some(1));
    assert_eq!(l.pop_front(), Some(2));
    assert_eq!(l.pop_front(), Some(3));

    // Can replace value at a pointer.
    let ptr = l.push_back(10);
    assert_eq!(l.peek_front(), Some(10));
    l.replace_val(ptr.unwrap(), 40);
    assert_eq!(l.peek_front(), Some(40));
    l.replace_val(ptr.unwrap(), 100);
    assert_eq!(l.pop_front(), Some(100));
  }
}
