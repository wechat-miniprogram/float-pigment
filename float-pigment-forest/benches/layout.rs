use criterion::{criterion_group, criterion_main, Criterion};
use float_pigment_css::{fixed::traits::ToFixed, typing::*};
use float_pigment_forest::node::*;

unsafe fn as_ref<'a>(node: *mut Node) -> &'a Node {
    &*node
}
fn gen_tree() -> &'static Node {
    unsafe {
        let root = as_ref(Node::new_ptr());
        let flex_container = as_ref(Node::new_ptr());
        flex_container.set_display(Display::Flex);
        flex_container.set_width(float_pigment_layout::DefLength::Points((30000.).to_fixed()));

        root.append_child(convert_node_ref_to_ptr(flex_container));
        for _ in 0..2000 {
            let flex_item = as_ref(Node::new_ptr());
            flex_item.set_width(float_pigment_layout::DefLength::Points(10.0.to_fixed()));
            flex_item.set_height(float_pigment_layout::DefLength::Points(10.0.to_fixed()));
            flex_container.append_child(convert_node_ref_to_ptr(flex_item));
        }
        root
    }
}

fn layout_test(root: &Node) {
    unsafe {
        root.layout(
            OptionSize::new(
                OptionNum::some(30000.0.to_fixed()),
                OptionNum::some(800.0.to_fixed()),
            ),
            Size::new(375.0.to_fixed(), 800.0.to_fixed()),
        );
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let root = gen_tree();
    c.bench_function("layout", |b| b.iter(|| layout_test(root)));
    c.bench_function("layout second", |b| b.iter(|| layout_test(root)));
    // println!(
    //     "{}",
    //     root.dump_to_html(
    //         DumpOptions {
    //             recursive: true,
    //             layout: true,
    //             style: DumpStyleMode::Mutation,
    //         },
    //         0,
    //     )
    // )
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
