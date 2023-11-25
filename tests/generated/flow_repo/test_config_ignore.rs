#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_foo_js_format_1_80a8abbd() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("/* @flow */\n\n// No error, this file is ignored\nvar x: number = \"string\";")?;
    assert_eq!(
        formatted,
        "/* @flow */\n\n// No error, this file is ignored\nvar x: number = \"string\";"
    );
    Ok(())
}
