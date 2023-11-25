#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_fragments_graphql_format_1_da44d692() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("graphql")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{\n  field\n  ...   XYZ\n  ... on TheType {\n    ...AFragment\n    ...          {\n      noTypeCondition\n    }\n  }\n}\n\nfragment XYZ on ABC {\n  field\n  ...AFragment\n}") ? ;
    assert_eq ! (formatted , "{\n  field\n  ...XYZ\n  ... on TheType {\n    ...AFragment\n    ... {\n      noTypeCondition\n    }\n  }\n}\n\nfragment XYZ on ABC {\n  field\n  ...AFragment\n}");
    Ok(())
}
