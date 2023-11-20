#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_e6cfd29e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("declare module A {\n  declare function foo(): string;\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "declare module A {\n  declare function foo(): string;\n}"
    );
}
#[test]
fn test_b_js_format_1_96d79093() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("/**\n * @providesModule B\n * @flow\n */\n\nmodule.exports = require('A');");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/**\n * @providesModule B\n * @flow\n */\n\nmodule.exports = require(\"A\");"
    );
}
#[test]
fn test_c_js_format_1_ec36d55d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("/**\n * @providesModule C\n * @flow\n */\n\nmodule.exports = require('B');");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/**\n * @providesModule C\n * @flow\n */\n\nmodule.exports = require(\"B\");"
    );
}
#[test]
fn test_d_js_format_1_a6be0995() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule D\n * @flow\n */\n\nvar bar1: string = require('A');\nvar bar2: string = require('B');\nvar bar3: string = require('C');") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule D\n * @flow\n */\n\nvar bar1: string = require(\"A\");\nvar bar2: string = require(\"B\");\nvar bar3: string = require(\"C\");");
}
