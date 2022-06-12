pub trait DLL<T: Clone + Copy> {
  type Pointer: Clone + Copy;

  fn size(&self) -> usize;
  fn capacity(&self) -> usize;
  fn get(&self, ptr: Self::Pointer) -> Option<T>;
  fn replace_val(&mut self, ptr: Self::Pointer, elem: T) -> Option<Self::Pointer>;
  fn peek_front(&self) -> Option<T>;
  fn pop_front(&mut self) -> Option<T>;

  fn push_back(&mut self, elem: T) -> Option<Self::Pointer>;
  fn move_back(&mut self, ptr: Self::Pointer) -> Option<Self::Pointer>;
}
