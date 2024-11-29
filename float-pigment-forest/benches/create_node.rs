use criterion::{black_box, criterion_group, criterion_main, Criterion};
use float_pigment_forest::node::*;

fn create_element(num: usize) {
    for _ in 0..num {
        let node = Node::new_ptr();
        black_box(node);
    }
}

fn gen_elem_tree(deep: usize, cur: usize, mut parent: Option<NodePtr>) {
    if cur > deep {
        return;
    }
    if parent.is_none() {
        parent = Some(Node::new_ptr());
    }
    let current = Node::new_ptr();
    unsafe { (*(parent.unwrap())).append_child(current) }
    for _ in 0..10 {
        gen_elem_tree(deep, cur + 1, Some(current))
    }
    black_box(parent);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("create 1000 elements", |b| b.iter(|| create_element(1000)));
    c.bench_function("create 10000 elements", |b| {
        b.iter(|| create_element(10000))
    });
    c.bench_function("gen 10^2 element tree", |b| {
        b.iter(|| gen_elem_tree(2, 0, None))
    });
    // c.bench_function("gen 10^3 element tree", |b| {
    //     b.iter(|| gen_elem_tree(3, 0, None))
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
