#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_test_js_format_1_193ee44f() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("var a = 0;\n\nfunction foo(x) { }\n\nfoo(\"\");");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "var a = 0;\n\nfunction foo(x) {}\n\nfoo(\"\");");
}
