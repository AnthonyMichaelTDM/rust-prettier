#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_0980fc7b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/*\n * @flow\n */\n\nfunction foo(x) {\n  var a: number = 'asdf';\n  return x * 10;\n}\n\n// This file should be ignored, so this should not result in an error\nfoo('Hello, world!');") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/*\n * @flow\n */\n\nfunction foo(x) {\n  var a: number = \"asdf\";\n  return x * 10;\n}\n\n// This file should be ignored, so this should not result in an error\nfoo(\"Hello, world!\");");
}
