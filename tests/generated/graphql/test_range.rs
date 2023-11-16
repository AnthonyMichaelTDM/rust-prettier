#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_issue_2296_graphql_format_1_c4fc8f9a() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("> 1 | {NPC{life}}\n    | ^^^\n  2 ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "{\n  NPC {\n    life\n  }\n}");
}
