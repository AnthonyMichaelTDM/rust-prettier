#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_number_js_typescript_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_number_js_format_1_7d75c180() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel", "typescript"])
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("// https://github.com/babel/babel/pull/11854\n\na = 09e1_1;\na = 09.1_1;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// https://github.com/babel/babel/pull/11854\n\na = 09e1_1;\na = 09.1_1;"
    );
}
