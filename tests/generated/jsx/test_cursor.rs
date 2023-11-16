#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_in_jsx_text_js_format_1_0759ce22() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("<>a<|>\n  <div>hi</div>\n</>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<>\n  a<|><div>hi</div>\n</>;");
}
