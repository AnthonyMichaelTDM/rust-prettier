#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_upper_case_html_tag_html_format_1_9a2d9078() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("<!doctype html><HTML></HTML>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<!doctype html><HTML></HTML>");
}
#[test]
fn test_upper_case_html_tag_2_html_format_1_a2244292() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("<!DOCTYPE html><HTML>\n  <body>\n  </body>\n</HTML>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<!doctype html>\n<HTML>\n  <body></body>\n</HTML>"
    );
}
