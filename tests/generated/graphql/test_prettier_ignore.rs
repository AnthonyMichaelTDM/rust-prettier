#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_prettier_ignore_comment_graphql_format_1_511acde9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format("{\n  # prettier-ignore\n  hero {\n       name\n    height\n  }\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "{\n  # prettier-ignore\n  hero {\n       name\n    height\n  }\n}"
    );
}
