#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_enum_unknown_members_js_trailing_commaall_format_1_c48af451() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("// @flow\n\nenum E {\n  A,\n  B,\n  ...\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "// @flow\n\nenum E {\n  A,\n  B,\n  ...\n}");
}
#[test]
fn test_enum_unknown_members_empty_js_trailing_commaall_format_1_d271186d() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("// @flow\n\nenum E {\n  ...\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "// @flow\n\nenum E {\n  ...\n}");
}
