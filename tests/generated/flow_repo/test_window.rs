#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_window_1_js_format_1_33b2abb1() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format("/*\n@providesModule Window1\n*/\nmodule.exports = window.history;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/*\n@providesModule Window1\n*/\nmodule.exports = window.history;"
    );
}
#[test]
fn test_window_2_js_format_1_720e4193() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format("/* @providesModule Window2 */\n\nmodule.exports = window.parent;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/* @providesModule Window2 */\n\nmodule.exports = window.parent;"
    );
}
