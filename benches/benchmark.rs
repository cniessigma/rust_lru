use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lru_cache::linked_list::{DLL, cellist, veclist};

fn add_then_mutate<L: DLL<usize>>(size: usize) {
  let mut list = L::new();
  for i in 0..size {
    list.push_back(i);
  }

  for i in list.iter_mut() {
    *i += 10;
  }

  for i in 0..size {
    assert_eq!(list.pop_front(), Some(i + 10));
  }

  assert_eq!(list.pop_front(), None);
}

fn add_then_move_back<L: DLL<usize>>(size: usize) {
  let mut list = L::new();
  let mut pointers = Vec::with_capacity(size);
  for i in 0..size {
    pointers.push(list.push_back(i));
  }

  for mut ptr in pointers {
    list.move_back(&mut ptr);
  }

  for i in 0..size {
    assert_eq!(list.pop_front(), Some(i));
  }

  assert_eq!(list.pop_front(), None);
}

fn add_then_move_front<L: DLL<usize>>(size: usize) {
  let mut list = L::new();
  let mut pointers = Vec::with_capacity(size);
  for i in 0..size {
    pointers.push(list.push_back(i));
  }

  for mut ptr in pointers {
    list.move_front(&mut ptr);
  }

  for i in 0..size {
    assert_eq!(list.pop_front(), Some(size - 1 - i));
  }

  assert_eq!(list.pop_front(), None);
}



fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function(
    "Vec Add Mutate 1000", |b| b.iter(||
      add_then_mutate::<veclist::VectorLinkedList<usize>>(black_box(1000))
    )
  );
  c.bench_function(
    "Vec Add Mutate 1000", |b| b.iter(||
      add_then_mutate::<cellist::CellLinkedList<usize>>(black_box(1000))
    )
  );

  c.bench_function(
    "Vec Add Move 1000", |b| b.iter(||
      add_then_move_back::<veclist::VectorLinkedList<usize>>(black_box(1000))
    )
  );
  c.bench_function(
    "Cell Add Move 1000", |b| b.iter(||
      add_then_move_back::<cellist::CellLinkedList<usize>>(black_box(1000))
    )
  );

  c.bench_function(
    "Vec Add Move Front 1000", |b| b.iter(||
      add_then_move_front::<veclist::VectorLinkedList<usize>>(black_box(1000))
    )
  );
  c.bench_function(
    "Cell Add Move Front 1000", |b| b.iter(||
      add_then_move_front::<cellist::CellLinkedList<usize>>(black_box(1000))
    )
  );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);