#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_after_key_js_format_1_7d53dd76() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("let a = {\n   a /* comment */: () => 1\n};\n\nlet b = {\n   \"a\" /* comment */: () => 1\n};") ? ;
    assert_eq!(
        formatted,
        "let a = {\n  a /* comment */: () => 1,\n};\n\nlet b = {\n  a /* comment */: () => 1,\n};"
    );
    Ok(())
}
