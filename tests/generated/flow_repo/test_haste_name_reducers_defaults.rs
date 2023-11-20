#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_module_1_js_format_1_13f6e16a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "/* @flow */\n\nvar test = require('test');\n\n(test: boolean);\n\nmodule.exports = test;",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nvar test = require(\"test\");\n\n(test: boolean);\n\nmodule.exports = test;");
}
#[test]
fn test_index_js_format_1_53876e0f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\n(require('Module1'): boolean);\n(require('Module2').foo(): boolean);\nrequire('Module3');") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\n(require(\"Module1\"): boolean);\n(require(\"Module2\").foo(): boolean);\nrequire(\"Module3\");");
}
