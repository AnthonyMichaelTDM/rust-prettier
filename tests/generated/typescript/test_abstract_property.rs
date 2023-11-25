#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_semicolon_ts_semifalse_format_1_88f84b98() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("abstract class C {\n  abstract get;\n  x() {}\n}")?;
    assert_eq!(
        formatted,
        "abstract class C {\n  abstract get;\n  x() {}\n}"
    );
    Ok(())
}
#[test]
fn test_semicolon_ts_format_1_88f84b98() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("abstract class C {\n  abstract get;\n  x() {}\n}")?;
    assert_eq!(
        formatted,
        "abstract class C {\n  abstract get;\n  x() {}\n}"
    );
    Ok(())
}
