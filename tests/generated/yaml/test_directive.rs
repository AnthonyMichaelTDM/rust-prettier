#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_with_parameters_yml_format_1_61cf25df() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("%YAML 1.2\n---");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "%YAML 1.2\n---");
}
#[test]
fn test_without_parameters_yml_format_1_85c72d90() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("%SOMETHING\n---");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "%SOMETHING\n---");
}
