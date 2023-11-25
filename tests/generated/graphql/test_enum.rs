#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_enum_graphql_format_1_b87558f1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("graphql")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("enum State {\n  PENDING\n  VISIBLE\n  ARCHIVED\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "enum State {\n  PENDING\n  VISIBLE\n  ARCHIVED\n}"
    );
}
