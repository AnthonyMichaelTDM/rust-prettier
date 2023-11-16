#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_license_with_flow_js_format_1_caad8d9d() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format("/* Copyright example */\n/* @flow */\n\n(\"\": void); // error");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/* Copyright example */\n/* @flow */\n\n(\"\": void); // error"
    );
}
#[test]
fn test_max_header_tokens_js_format_1_4078833d() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n/* second token */\n/* third token */\n/**\n * After max_header_tokens (in .flowconfig), we no longer care:\n *\n * @flow\n */") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n/* second token */\n/* third token */\n/**\n * After max_header_tokens (in .flowconfig), we no longer care:\n *\n * @flow\n */");
}
#[test]
fn test_multiple_flows_1_js_format_1_e9ee9e7e() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("/* @flow */\n/* @flow */");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "/* @flow */\n/* @flow */");
}
#[test]
fn test_multiple_flows_2_js_format_1_fddae2f5() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("/**\n * @flow\n * @noflow\n */");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "/**\n * @flow\n * @noflow\n */");
}
#[test]
fn test_multiple_provides_module_1_js_format_1_f6210eb3() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format("/**\n * @providesModule Foo\n * @providesModule Bar\n * @flow\n */");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/**\n * @providesModule Foo\n * @providesModule Bar\n * @flow\n */"
    );
}
#[test]
fn test_multiple_provides_module_2_js_format_1_5649bb3a() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer
        .format("/**\n * @providesModule Foo\n * @flow\n */\n/**\n * @providesModule Bar\n */");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/**\n * @providesModule Foo\n * @flow\n */\n/**\n * @providesModule Bar\n */"
    );
}
#[test]
fn test_use_strict_with_flow_js_format_1_18e50e09() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("\"use strict\";\n/* @flow */\n\n(\"\": void); // error");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "\"use strict\";\n/* @flow */\n\n(\"\": void); // error"
    );
}
#[test]
fn test_with_flow_js_format_1_3c2a4ad6() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("/* @flow */\n\n(\"\": void); // error");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "/* @flow */\n\n(\"\": void); // error");
}
#[test]
fn test_without_flow_js_format_1_e2384d32() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("/* some other comment */\n\n(\"\": void); // no error");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/* some other comment */\n\n(\"\": void); // no error"
    );
}
