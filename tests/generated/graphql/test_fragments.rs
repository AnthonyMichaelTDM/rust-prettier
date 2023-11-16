#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_fragments_graphql_format_1_da44d692() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("{\n  field\n  ...   XYZ\n  ... on TheType {\n    ...AFragment\n    ...          {\n      noTypeCondition\n    }\n  }\n}\n\nfragment XYZ on ABC {\n  field\n  ...AFragment\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{\n  field\n  ...XYZ\n  ... on TheType {\n    ...AFragment\n    ... {\n      noTypeCondition\n    }\n  }\n}\n\nfragment XYZ on ABC {\n  field\n  ...AFragment\n}");
}
