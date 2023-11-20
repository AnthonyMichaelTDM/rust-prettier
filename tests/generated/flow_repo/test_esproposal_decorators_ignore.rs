#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_6a9fd477() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\n@decorator4\nclass Foo {\n  @decorator1\n  method1() {}\n\n  @decorator2\n  @decorator3\n  method2() {}\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\n@decorator4\nclass Foo {\n  @decorator1\n  method1() {}\n\n  @decorator2\n  @decorator3\n  method2() {}\n}");
}
