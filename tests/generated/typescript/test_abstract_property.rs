#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_semicolon_ts_semifalse_format_1_88f84b98() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("abstract class C {\n  abstract get;\n  x() {}\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "abstract class C {\n  abstract get;\n  x() {}\n}"
    );
}
#[test]
fn test_semicolon_ts_format_1_88f84b98() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("abstract class C {\n  abstract get;\n  x() {}\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "abstract class C {\n  abstract get;\n  x() {}\n}"
    );
}
