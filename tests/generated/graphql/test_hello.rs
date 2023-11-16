#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_hello_graphql_format_1_6127a37e() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("{\n  project(name: \"GraphQL\") {\n    tagline\n  }\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "{\n  project(name: \"GraphQL\") {\n    tagline\n  }\n}"
    );
}
