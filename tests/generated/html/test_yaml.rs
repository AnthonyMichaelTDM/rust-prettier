#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_invalid_html_format_1_a113b104() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("---\n    invalid:\ninvalid:\n---\n\n\n\n<html><head></head><body></body></html>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "---\n    invalid:\ninvalid:\n---\n\n<html>\n  <head></head>\n  <body></body>\n</html>"
    );
}
#[test]
fn test_yaml_html_format_1_9f228366() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("---\nhello:     world\n---\n\n\n\n\n\n\n<html><head></head><body></body></html>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "---\nhello: world\n---\n\n<html>\n  <head></head>\n  <body></body>\n</html>"
    );
}
