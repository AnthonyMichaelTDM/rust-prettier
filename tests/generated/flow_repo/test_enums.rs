use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_declared_js_babel_flow_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_declared_js_format_1_6ce3e3e4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\ndeclare enum E {\n  A,\n  B,\n}\n\n(E.A: E); // OK\n(E.A: empty); // ERROR\n\ndeclare export enum F {\n  N,\n  M,\n}\n\n(F.N: F); // OK\n(F.N: empty); // ERROR") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\ndeclare enum E {\n  A,\n  B,\n}\n\n(E.A: E); // OK\n(E.A: empty); // ERROR\n\ndeclare export enum F {\n  N,\n  M,\n}\n\n(F.N: F); // OK\n(F.N: empty); // ERROR");
}
#[test]
fn test_lib_js_babel_flow_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_lib_js_format_1_0b7e8224() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\ndeclare module 'declared-module' {\n  declare enum G {\n    J,\n    K,\n  }\n  declare export enum H {\n    X,\n    Y,\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\ndeclare module \"declared-module\" {\n  declare enum G {\n    J,\n    K,\n  }\n  declare export enum H {\n    X,\n    Y,\n  }\n}");
}
