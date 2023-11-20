#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_should_fail_without_weak_js_format_1_24ff8360() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n// This should fail without weak mode, because of the glaring type error.\n\nfunction returns_a_string() {\n  return \"Hello\";\n}\n\nfunction expects_an_int() {\n  return returns_a_string() * 10;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n// This should fail without weak mode, because of the glaring type error.\n\nfunction returns_a_string() {\n  return \"Hello\";\n}\n\nfunction expects_an_int() {\n  return returns_a_string() * 10;\n}");
}
#[test]
fn test_should_pass_with_weak_js_format_1_357c95c1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow weak */\n// This should fail without weak mode, because of the glaring type error.\n\nfunction returns_a_string() {\n  return \"Hello\";\n}\n\nfunction expects_an_int() {\n  return returns_a_string() * 10;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow weak */\n// This should fail without weak mode, because of the glaring type error.\n\nfunction returns_a_string() {\n  return \"Hello\";\n}\n\nfunction expects_an_int() {\n  return returns_a_string() * 10;\n}");
}
