#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_test_js_format_1_1c32dddd() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("/* @flow */\n\nexport * as foo from \"./test\";");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "/* @flow */\n\nexport * as foo from \"./test\";");
}
