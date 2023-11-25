use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_yaml_less_format_1_cf05a353() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("less")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("---\ntitle: Title\ndescription: Description\n---\n\na {\n  color: red;\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "---\ntitle: Title\ndescription: Description\n---\n\na {\n  color: red;\n}"
    );
}
