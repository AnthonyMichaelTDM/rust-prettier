#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_empty_less_format_1_f55e7940() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format (".ui-syntax-color();\n\ndetails[open] {\n}\n\ndiv {\n  box-sizing: border-box;;\n  color: red;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ".ui-syntax-color();\n\ndetails[open] {\n}\n\ndiv {\n  box-sizing: border-box;\n  color: red;\n}");
}
