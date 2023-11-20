#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_249b374f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("var o = Object.freeze({ foo: 0 });\n(o.foo: string);\nmodule.exports = o;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "var o = Object.freeze({ foo: 0 });\n(o.foo: string);\nmodule.exports = o;"
    );
}
#[test]
fn test_test_2_js_format_1_41bfbc0a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("var o = require('./test');\n(o.foo: string);");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "var o = require(\"./test\");\n(o.foo: string);");
}
