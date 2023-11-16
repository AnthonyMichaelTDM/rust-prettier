#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_optional_catch_binding_js_format_1_95b5ba03() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("try {\n\n}\ncatch {\n\n}\nfinally {\n\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "try {\n} catch {\n} finally {\n}");
}
