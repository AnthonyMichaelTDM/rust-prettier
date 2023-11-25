#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_issue_2296_graphql_format_1_c4fc8f9a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("graphql")
        .print_width(80)
        .range_end(3)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> 1 | {NPC{life}}\n    | ^^^\n  2 ")?;
    assert_eq!(formatted, "{\n  NPC {\n    life\n  }\n}");
    Ok(())
}
