#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_arguments_graphql_format_1_8dd433fc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("graphql")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("type Video {\n  playable_url(quality: String, preferred: String): Url\n}")?;
    assert_eq!(
        formatted,
        "type Video {\n  playable_url(quality: String, preferred: String): Url\n}"
    );
    Ok(())
}
#[test]
fn test_directives_graphql_format_1_015463b3() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("graphql")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("type PokemonType {\n  pokemon_type: String @mock(value: \"Electric\")\n}")?;
    assert_eq!(
        formatted,
        "type PokemonType {\n  pokemon_type: String @mock(value: \"Electric\")\n}"
    );
    Ok(())
}
#[test]
fn test_extend_graphql_format_1_70893eda() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("graphql")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("extend type Feedback {\n  custom_int: Int\n}")?;
    assert_eq!(formatted, "extend type Feedback {\n  custom_int: Int\n}");
    Ok(())
}
#[test]
fn test_implements_graphql_format_1_0e393185() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("graphql")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("type VRMConversation implements Node& Entity @foo {\n  a: Int\n}")?;
    assert_eq!(
        formatted,
        "type VRMConversation implements Node & Entity @foo {\n  a: Int\n}"
    );
    Ok(())
}
#[test]
fn test_input_graphql_format_1_271f33b7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("graphql")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("input Params {\n  app_id: ID!\n  key_hash: String!\n}")?;
    assert_eq!(
        formatted,
        "input Params {\n  app_id: ID!\n  key_hash: String!\n}"
    );
    Ok(())
}
#[test]
fn test_object_type_def_graphql_format_1_089b8268() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("graphql")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("type FeedHomeStories {\n  debug_info: String\n  query_title: String\n}")?;
    assert_eq!(
        formatted,
        "type FeedHomeStories {\n  debug_info: String\n  query_title: String\n}"
    );
    Ok(())
}
