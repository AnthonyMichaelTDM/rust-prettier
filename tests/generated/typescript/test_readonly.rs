#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_array_ts_format_1_acbe0f31() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "declare const array: readonly number[];\ndeclare const tuple: readonly [number, number];",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "declare const array: readonly number[];\ndeclare const tuple: readonly [number, number];"
    );
}
#[test]
fn test_readonly_ts_format_1_444bf607() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("class Foo {\n  public readonly foo = 'string';\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "class Foo {\n  public readonly foo = \"string\";\n}"
    );
}
