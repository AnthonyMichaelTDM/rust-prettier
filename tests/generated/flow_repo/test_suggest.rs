use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_lib_js_format_1_ad6104a1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "/* @flow */\n\nfunction bar(w: number): number { return w; }\n\nmodule.exports = bar;",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/* @flow */\n\nfunction bar(w: number): number {\n  return w;\n}\n\nmodule.exports = bar;"
    );
}
#[test]
fn test_suggest_js_format_1_5d92be07() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nvar bar = require('./lib');\n\nfunction foo(z: number) { return bar(z); }\n\nvar array = [\"foo\", \"bar\"];\narray;\n\nmodule.exports = {foo:foo};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nvar bar = require(\"./lib\");\n\nfunction foo(z: number) {\n  return bar(z);\n}\n\nvar array = [\"foo\", \"bar\"];\narray;\n\nmodule.exports = { foo: foo };");
}
