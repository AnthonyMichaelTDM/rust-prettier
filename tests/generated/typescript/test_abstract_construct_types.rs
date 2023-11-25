#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_abstract_construct_types_ts_format_1_c8e86828() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type T = abstract new () => void;\ntype T = abstract    new () => void;\ntype T = abstract\n  new () => void;") ? ;
    assert_eq ! (formatted , "type T = abstract new () => void;\ntype T = abstract new () => void;\ntype T = abstract new () => void;");
    Ok(())
}
