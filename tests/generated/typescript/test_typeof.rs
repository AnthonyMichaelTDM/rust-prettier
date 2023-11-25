#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_typeof_ts_format_1_37fb8a88() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type A = (typeof node.children)[number];\ntype B = (typeof node.children)[];\ntype C = ((typeof node.children)[number])[];\ntype D = number[(typeof node.children)];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type A = (typeof node.children)[number];\ntype B = (typeof node.children)[];\ntype C = (typeof node.children)[number][];\ntype D = number[typeof node.children];");
}
