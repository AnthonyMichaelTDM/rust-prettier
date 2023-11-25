#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_maybe_return_js_format_1_bdc0c197() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("function getScaledData({x}): ?{foo: number} {}")?;
    assert_eq!(
        formatted,
        "function getScaledData({ x }): ?{ foo: number } {}"
    );
    Ok(())
}
#[test]
fn test_prettier_ignore_js_format_1_5455fae1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("type A = {\n  // prettier-ignore\n  +input: ?(?B),\n}")?;
    assert_eq!(
        formatted,
        "type A = {\n  // prettier-ignore\n  +input: ?(?B),\n};"
    );
    Ok(())
}
