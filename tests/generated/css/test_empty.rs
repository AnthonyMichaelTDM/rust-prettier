#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_empty_css_format_1_8a7ac226() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer
        .format("\ndetails[open] {\n}\n\ndiv {\n  box-sizing: border-box;;\n  color: red;\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "details[open] {\n}\n\ndiv {\n  box-sizing: border-box;\n  color: red;\n}"
    );
}
