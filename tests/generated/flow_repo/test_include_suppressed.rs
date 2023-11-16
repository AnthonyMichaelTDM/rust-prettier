#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_test_js_format_1_f4ebdf25() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n(123: string); // Normal error\n\n// $FlowFixMe\n(123: string); // Suppressed error\n\n// $FlowFixMe - unused suppression comment") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n(123: string); // Normal error\n\n// $FlowFixMe\n(123: string); // Suppressed error\n\n// $FlowFixMe - unused suppression comment");
}
