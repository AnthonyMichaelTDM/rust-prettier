#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_maybe_return_js_format_1_bdc0c197() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("function getScaledData({x}): ?{foo: number} {}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "function getScaledData({ x }): ?{ foo: number } {}"
    );
}
#[test]
fn test_prettier_ignore_js_format_1_5455fae1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("type A = {\n  // prettier-ignore\n  +input: ?(?B),\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "type A = {\n  // prettier-ignore\n  +input: ?(?B),\n};"
    );
}
