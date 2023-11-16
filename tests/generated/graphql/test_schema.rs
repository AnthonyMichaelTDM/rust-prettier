#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_schema_graphql_format_1_c2a171f7() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("\"\"\"Schema definition description\"\"\"\nschema {\n  query: Root\n  mutation: Mutation\n  subscription: Subscription\n}\n\nextend schema { subscription: Subscription }\n\n# \\`SchemaExtension\\`s don't require any operation types and doesn't print\n# curlies in their abscence.\nextend schema\n  @directive") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "\"\"\"\nSchema definition description\n\"\"\"\nschema {\n  query: Root\n  mutation: Mutation\n  subscription: Subscription\n}\n\nextend schema {\n  subscription: Subscription\n}\n\n# \\`SchemaExtension\\`s don't require any operation types and doesn't print\n# curlies in their abscence.\nextend schema @directive");
}
