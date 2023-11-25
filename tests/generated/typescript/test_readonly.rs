#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_array_ts_format_1_acbe0f31() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "declare const array: readonly number[];\ndeclare const tuple: readonly [number, number];",
    )?;
    assert_eq!(
        formatted,
        "declare const array: readonly number[];\ndeclare const tuple: readonly [number, number];"
    );
    Ok(())
}
#[test]
fn test_readonly_ts_format_1_444bf607() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("class Foo {\n  public readonly foo = 'string';\n}")?;
    assert_eq!(
        formatted,
        "class Foo {\n  public readonly foo = \"string\";\n}"
    );
    Ok(())
}
