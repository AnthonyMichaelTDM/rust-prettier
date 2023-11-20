#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_colon_css_format_1_1421ac58() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["css"])
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("div {\n  filter: progid:DXImageTransform.Microsoft.BasicImage(rotation=3);\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "div {\n  filter: progid:DXImageTransform.Microsoft.BasicImage(rotation=3);\n}"
    );
}
