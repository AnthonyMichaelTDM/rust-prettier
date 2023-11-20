#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_charset_css_format_1_acc3ec5e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["css"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("@charset \"utf-8\";");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "@charset \"utf-8\";");
}
