use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_4941b22d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* @flow */\nmodule.exports = 0;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "/* @flow */\nmodule.exports = 0;");
}
#[test]
fn test_b_js_format_1_614ddba3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* @flow */\nmodule.exports = { foo: \"\" }");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "/* @flow */\nmodule.exports = { foo: \"\" };");
}
#[test]
fn test_c_js_format_1_69a03374() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\nvar o = {\n    A: require('./A'),\n    ...require('./B'),\n};\nmodule.exports = o;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\nvar o = {\n  A: require(\"./A\"),\n  ...require(\"./B\"),\n};\nmodule.exports = o;");
}
#[test]
fn test_d_js_format_1_2ab470b8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\nvar C = require('./C');\nvar x: number = C.foo;\nvar y: string = C.A;\nC.A = false;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\nvar C = require(\"./C\");\nvar x: number = C.foo;\nvar y: string = C.A;\nC.A = false;");
}
