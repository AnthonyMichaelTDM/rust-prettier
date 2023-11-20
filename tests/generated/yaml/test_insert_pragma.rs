#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_with_pragma_yml_insert_pragmatrue_format_1_f4bd2e0f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .insert_pragma(true)
        .parsers(vec!["yaml"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("# @prettier\n\n    123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "# @prettier\n\n123");
}
#[test]
fn test_without_pragma_yml_insert_pragmatrue_format_1_a0991ecd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .insert_pragma(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\n\n    123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "# @format\n\n123");
}
