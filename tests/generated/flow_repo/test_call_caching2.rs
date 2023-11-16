#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_test_js_format_1_937d46dd() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format(
        "// @flow\n\nfunction Foo(items: ?Iterable<number>) {\n  Iterable(items || []).size;\n}",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// @flow\n\nfunction Foo(items: ?Iterable<number>) {\n  Iterable(items || []).size;\n}"
    );
}
