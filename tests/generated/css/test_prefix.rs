#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_prefix_css_format_1_c102d616() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer
        .format("div {\n  margin-left: -@leftMargin;\n  transform: translateY(-@offset);\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "div {\n  margin-left: -@leftMargin;\n  transform: translateY(-@offset);\n}"
    );
}
