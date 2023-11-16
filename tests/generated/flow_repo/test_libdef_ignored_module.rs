#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_test_js_format_1_e79731d2() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format(
        "/* @flow */\n\nimport foo from \"foo\";\n\n(foo.bar : string); // error number ~> string",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/* @flow */\n\nimport foo from \"foo\";\n\n(foo.bar: string); // error number ~> string"
    );
}
