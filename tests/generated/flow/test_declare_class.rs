#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_indexer_js_format_1_321eff5c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare class A {\n  [number]: boolean;\n  static [string]: boolean;\n}\n\n// Read-only\ndeclare class B {\n  +[number]: boolean;\n  static +[string]: boolean;\n}") ? ;
    assert_eq ! (formatted , "declare class A {\n  [number]: boolean;\n  static [string]: boolean;\n}\n\n// Read-only\ndeclare class B {\n  +[number]: boolean;\n  static +[string]: boolean;\n}");
    Ok(())
}
