#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_important_less_format_1_6bddc39f() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format(
        "div {\n  max-height: none !important;\n}\n\n.important {\n  .foo() !important;\n}",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "div {\n  max-height: none !important;\n}\n\n.important {\n  .foo() !important;\n}"
    );
}
