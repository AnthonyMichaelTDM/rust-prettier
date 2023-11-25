#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_hello_graphql_format_1_6127a37e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("graphql")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("{\n  project(name: \"GraphQL\") {\n    tagline\n  }\n}")?;
    assert_eq!(
        formatted,
        "{\n  project(name: \"GraphQL\") {\n    tagline\n  }\n}"
    );
    Ok(())
}
