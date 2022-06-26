use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lru_cache::linked_list::{DLL, cellist, veclist};

fn add_then_mutate<L: DLL<u64>>(size: u64) {
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



fn criterion_benchmark(c: &mut Criterion) {
  let vec_func = add_then_mutate::<veclist::VectorLinkedList<u64>>;
  let cell_func = add_then_mutate::<cellist::CellLinkedList<u64>>;

  c.bench_function("Vec 1000", |b| b.iter(|| vec_func(black_box(1000))));
  c.bench_function("Cell 1000", |b| b.iter(|| cell_func(black_box(1000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);