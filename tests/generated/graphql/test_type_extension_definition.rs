use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_type_extendsion_syntax_graphql_format_1_5e4baba0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("graphql")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "type User {\n  name: String\n  birthday: Int\n}\n\nextend type User {\n  age: Int\n}",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "type User {\n  name: String\n  birthday: Int\n}\n\nextend type User {\n  age: Int\n}"
    );
}
