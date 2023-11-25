#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_enum_graphql_format_1_b87558f1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("graphql")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("enum State {\n  PENDING\n  VISIBLE\n  ARCHIVED\n}")?;
    assert_eq!(
        formatted,
        "enum State {\n  PENDING\n  VISIBLE\n  ARCHIVED\n}"
    );
    Ok(())
}
