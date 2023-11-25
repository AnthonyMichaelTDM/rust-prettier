#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_typeof_ts_format_1_37fb8a88() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type A = (typeof node.children)[number];\ntype B = (typeof node.children)[];\ntype C = ((typeof node.children)[number])[];\ntype D = number[(typeof node.children)];") ? ;
    assert_eq ! (formatted , "type A = (typeof node.children)[number];\ntype B = (typeof node.children)[];\ntype C = (typeof node.children)[number][];\ntype D = number[typeof node.children];");
    Ok(())
}
