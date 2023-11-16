#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_attr_selector_css_format_1_c60bf547() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format(
        "[  data-sizes~=\"m:0\" ]    {\n  color:pink;\n}\n\n[ class*=\"test\" ]{\n  color: silver}",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "[data-sizes~=\"m:0\"] {\n  color: pink;\n}\n\n[class*=\"test\"] {\n  color: silver;\n}"
    );
}
