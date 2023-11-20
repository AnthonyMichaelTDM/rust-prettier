#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_empty_css_format_1_8a7ac226() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["css"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("\ndetails[open] {\n}\n\ndiv {\n  box-sizing: border-box;;\n  color: red;\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "details[open] {\n}\n\ndiv {\n  box-sizing: border-box;\n  color: red;\n}"
    );
}
