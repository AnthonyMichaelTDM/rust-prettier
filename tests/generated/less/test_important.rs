#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_important_less_format_1_6bddc39f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["less"])
        .print_width(80)
        .build()
        .unwrap();
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
