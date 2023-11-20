#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_vendor_prefix_css_format_1_41c56621() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["css"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format(".class\n{-webkit-box-sizing:border-box;box-sizing:border-box\n\n\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        ".class {\n  -webkit-box-sizing: border-box;\n  box-sizing: border-box;\n}"
    );
}
