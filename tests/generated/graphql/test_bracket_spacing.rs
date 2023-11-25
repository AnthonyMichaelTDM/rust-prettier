#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_bracket_spacing_graphql_bracket_spacingfalse_format_1_52e19abc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("graphql")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("fragment Visit on HighlightedVisit\n  @argumentDefinitions(\n    count: {type: \"Int\", defaultValue: 20}\n  ) {\n  name\n}") ? ;
    assert_eq ! (formatted , "fragment Visit on HighlightedVisit\n@argumentDefinitions(count: {type: \"Int\", defaultValue: 20}) {\n  name\n}");
    Ok(())
}
#[test]
fn test_bracket_spacing_graphql_format_1_52e19abc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("graphql")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("fragment Visit on HighlightedVisit\n  @argumentDefinitions(\n    count: {type: \"Int\", defaultValue: 20}\n  ) {\n  name\n}") ? ;
    assert_eq ! (formatted , "fragment Visit on HighlightedVisit\n@argumentDefinitions(count: { type: \"Int\", defaultValue: 20 }) {\n  name\n}");
    Ok(())
}
