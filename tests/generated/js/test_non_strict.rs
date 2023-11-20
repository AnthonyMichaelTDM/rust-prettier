#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_argument_name_clash_js_format_1_b6f4c0da() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("function f(a,a){return a}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "function f(a, a) {\n  return a;\n}");
}
#[test]
fn test_keywords_js_format_1_ad0425aa() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var package = require('../package');\n\n/**\n * My amazing comment\n */\nfunction myFunction() {\n\treturn 'StringGainz';\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var package = require(\"../package\");\n\n/**\n * My amazing comment\n */\nfunction myFunction() {\n  return \"StringGainz\";\n}");
}
#[test]
fn test_octal_number_js_typescript_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_octal_number_js_format_1_030ed232() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("0777");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "0777;");
}
