extern crate rand;

#[macro_use]
extern crate bencher;

use bencher::Bencher;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use float_pigment_css::StyleSheet;

const RULES: &str = "color: red;";

fn create_random_selector() -> String {
    let random_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    format!(".{}", random_string)
}

fn random_rules_in_selectors(rules_times: i32, selector_times: i32) -> String {
    let mut style = vec![];
    for _ in 0..selector_times {
        let tot_rules = vec![RULES; rules_times as usize];
        let cur = format!("{} {{ {} }}", create_random_selector(), tot_rules.join(""));
        style.push(cur);
    }
    style.join("")
}

fn bench_parse_5000_rules_10_selector(b: &mut Bencher) {
    let style = &random_rules_in_selectors(5000, 10);
    b.iter(|| {
        StyleSheet::from_str(style);
    });
}

fn bench_parse_500_rules_100_selector(b: &mut Bencher) {
    let style = &random_rules_in_selectors(500, 100);
    b.iter(|| {
        StyleSheet::from_str(style);
    });
}

fn bench_parse_50_rules_1000_selector(b: &mut Bencher) {
    let style = &random_rules_in_selectors(50, 1000);
    b.iter(|| {
        StyleSheet::from_str(style);
    });
}

fn bench_parse_50_rules_10_selector(b: &mut Bencher) {
    let style = &random_rules_in_selectors(50, 10);
    b.iter(|| {
        StyleSheet::from_str(style);
    });
}

fn bench_parse_500_rules_10_selector(b: &mut Bencher) {
    let style = &random_rules_in_selectors(500, 10);
    b.iter(|| {
        StyleSheet::from_str(style);
    });
}

benchmark_group!(
    benches,
    bench_parse_5000_rules_10_selector,
    bench_parse_500_rules_100_selector,
    bench_parse_50_rules_1000_selector,
    bench_parse_50_rules_10_selector,
    bench_parse_500_rules_10_selector,
);

benchmark_main!(benches);
