#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_api_js_format_1_9d2c9434() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer
        .format("// @flow\n\nvar OpenGraphObject = require('./models/OpenGraphObject.js');");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// @flow\n\nvar OpenGraphObject = require(\"./models/OpenGraphObject.js\");"
    );
}
