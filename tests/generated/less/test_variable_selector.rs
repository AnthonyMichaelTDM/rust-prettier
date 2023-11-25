#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_variable_selector_less_format_1_df5c2d6b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("less")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("@{selector}{}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "@{selector} {\n}");
}
