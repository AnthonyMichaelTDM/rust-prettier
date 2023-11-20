#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_unary_js_format_1_0766bcd2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nfunction x0(y: string): number {\n  return +y; // ok, + exists solely for coercion\n}\n\nfunction x1(y: string): number {\n  return -y; // error, we don't allow coercion here\n}\n\nfunction x3(y: string) {\n  return ~y;  // error, we don't allow coercion here\n}\n\nfunction x4(y: string): boolean {\n  return !y; // ok, coercion is allowed\n}\n\n(-1: void); // error, number ~> void") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nfunction x0(y: string): number {\n  return +y; // ok, + exists solely for coercion\n}\n\nfunction x1(y: string): number {\n  return -y; // error, we don't allow coercion here\n}\n\nfunction x3(y: string) {\n  return ~y; // error, we don't allow coercion here\n}\n\nfunction x4(y: string): boolean {\n  return !y; // ok, coercion is allowed\n}\n\n(-1: void); // error, number ~> void");
}
#[test]
fn test_update_js_format_1_5b70f86d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nlet tests = [\n  function(y: number) {\n    y++;\n    y--;\n    ++y;\n    --y;\n  },\n\n  function(y: string) {\n    y++; // error, we don't allow coercion here\n    (y: number); // ok, y is a number now\n    y++; // error, but you still can't write a number to a string\n  },\n\n  function(y: string) {\n    y--; // error, we don't allow coercion here\n  },\n\n  function(y: string) {\n    ++y; // error, we don't allow coercion here\n  },\n\n  function(y: string) {\n    --y; // error, we don't allow coercion here\n  },\n\n  function() {\n    const y = 123;\n    y++; // error, can't update const\n    y--; // error, can't update const\n  },\n\n  function(y: any) {\n    y++; // ok\n  },\n];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nlet tests = [\n  function (y: number) {\n    y++;\n    y--;\n    ++y;\n    --y;\n  },\n\n  function (y: string) {\n    y++; // error, we don't allow coercion here\n    (y: number); // ok, y is a number now\n    y++; // error, but you still can't write a number to a string\n  },\n\n  function (y: string) {\n    y--; // error, we don't allow coercion here\n  },\n\n  function (y: string) {\n    ++y; // error, we don't allow coercion here\n  },\n\n  function (y: string) {\n    --y; // error, we don't allow coercion here\n  },\n\n  function () {\n    const y = 123;\n    y++; // error, can't update const\n    y--; // error, can't update const\n  },\n\n  function (y: any) {\n    y++; // ok\n  },\n];");
}
