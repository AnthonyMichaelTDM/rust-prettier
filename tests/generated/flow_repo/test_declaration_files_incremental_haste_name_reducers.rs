#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_ebb7c7f8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nclass AImplementation {}\nexport function foo(): AImplementation { return new AImplementation(); }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nclass AImplementation {}\nexport function foo(): AImplementation {\n  return new AImplementation();\n}");
}
#[test]
fn test_index_js_format_1_bd9a57c5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nvar A = require('A');\n(A.foo(): boolean); // Error: Either AImplementation ~> boolean or ADefinition ~> boolean\n\nvar test = require('test');\n(test.foo(): boolean); // Error: Either TestImplementation ~> boolean or TestDefinition ~> boolean") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nvar A = require(\"A\");\n(A.foo(): boolean); // Error: Either AImplementation ~> boolean or ADefinition ~> boolean\n\nvar test = require(\"test\");\n(test.foo(): boolean); // Error: Either TestImplementation ~> boolean or TestDefinition ~> boolean");
}
