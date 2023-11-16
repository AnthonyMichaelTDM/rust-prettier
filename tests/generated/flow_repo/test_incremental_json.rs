#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_test_js_format_1_7bf016cd() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer
        .format("/**\n * @flow\n */\nvar data = require('./data');\nvar x: number = data.x;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/**\n * @flow\n */\nvar data = require(\"./data\");\nvar x: number = data.x;"
    );
}
