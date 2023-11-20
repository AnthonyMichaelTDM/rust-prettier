#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_issue_2267_css_format_1_b0812228() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["css"])
        .print_width(80)
        .range_end(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> 1 | .x{} \n    | ^^^^\n  2 ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, ".x{} ");
}
