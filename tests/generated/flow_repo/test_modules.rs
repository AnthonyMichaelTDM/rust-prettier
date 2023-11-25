use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_cli_js_format_1_fa96be43() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("/* @flow */\n\nvar f = require('./lib');\n\nvar y:number = f(0);");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/* @flow */\n\nvar f = require(\"./lib\");\n\nvar y: number = f(0);"
    );
}
#[test]
fn test_cli_2_js_format_1_bb49e254() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("/* @flow */\n\nvar f = require('./lib');\n\nf(\"...\");");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/* @flow */\n\nvar f = require(\"./lib\");\n\nf(\"...\");"
    );
}
#[test]
fn test_lib_js_format_1_026bdf92() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nfunction g(x:string) { }\n\n//function f(x) { g(x); return x; }\n//function f(x:number) { g(x); return x; }\nfunction f(x:number):number { g(x); return x; }\n\nmodule.exports = f;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nfunction g(x: string) {}\n\n//function f(x) { g(x); return x; }\n//function f(x:number) { g(x); return x; }\nfunction f(x: number): number {\n  g(x);\n  return x;\n}\n\nmodule.exports = f;");
}
