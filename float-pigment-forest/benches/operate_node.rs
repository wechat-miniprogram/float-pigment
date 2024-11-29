use criterion::{black_box, criterion_group, criterion_main, Criterion};
use float_pigment_forest::node::*;

fn append_element(num: usize) {
    let parent: *mut Node = Node::new_ptr();
    for _ in 0..num {
        let node = Node::new_ptr();
        unsafe {
            (*parent).append_child(node);
        }
    }
    black_box(parent);
}

fn insert_element_to_first(num: usize) {
    let parent: *mut Node = Node::new_ptr();
    for _ in 0..num {
        let node = Node::new_ptr();
        unsafe {
            (*parent).insert_child_at(node, 0);
        }
    }
    black_box(parent);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("append 100 elements", |b| b.iter(|| append_element(100)));
    c.bench_function("append 1000 elements", |b| b.iter(|| append_element(1000)));

    c.bench_function("insert 100 elements at the first", |b| {
        b.iter(|| insert_element_to_first(100))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
