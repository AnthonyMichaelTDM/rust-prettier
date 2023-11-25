#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_trailing_graphql_trailing_commaall_format_1_3bd343f6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("graphql")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("query Query(\n  $pageID: ID!\n  $scale: Float\n  $PROJECT_UNIT_PROFILE_PICTURE_SIZE: Int\n  $PROJECT_UNIT_CARD_SIZE: Int\n) @argumentDefinitions(\n    count: {type: \"Int\", defaultValue: 20, someSuperSuperSuperSuperLongType: 301}\n    test: [{type: \"Int\", defaultValue: 20, someSuperSuperSuperSuperLongType: 301}]\n  ) {\n  cover_photo {\n    image(\n      width: $PROJECT_UNIT_CARD_SIZE,\n      height: $PROJECT_UNIT_CARD_SIZE,\n      sizing: \"cover-fill\",\n      scale: $scale,\n    ) {\n      uri\n    }\n  }\n}") ? ;
    assert_eq ! (formatted , "query Query(\n  $pageID: ID!\n  $scale: Float\n  $PROJECT_UNIT_PROFILE_PICTURE_SIZE: Int\n  $PROJECT_UNIT_CARD_SIZE: Int\n)\n@argumentDefinitions(\n  count: {\n    type: \"Int\"\n    defaultValue: 20\n    someSuperSuperSuperSuperLongType: 301\n  }\n  test: [\n    { type: \"Int\", defaultValue: 20, someSuperSuperSuperSuperLongType: 301 }\n  ]\n) {\n  cover_photo {\n    image(\n      width: $PROJECT_UNIT_CARD_SIZE\n      height: $PROJECT_UNIT_CARD_SIZE\n      sizing: \"cover-fill\"\n      scale: $scale\n    ) {\n      uri\n    }\n  }\n}");
    Ok(())
}
#[test]
fn test_trailing_graphql_trailing_commaes_5_format_1_3bd343f6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("graphql")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("query Query(\n  $pageID: ID!\n  $scale: Float\n  $PROJECT_UNIT_PROFILE_PICTURE_SIZE: Int\n  $PROJECT_UNIT_CARD_SIZE: Int\n) @argumentDefinitions(\n    count: {type: \"Int\", defaultValue: 20, someSuperSuperSuperSuperLongType: 301}\n    test: [{type: \"Int\", defaultValue: 20, someSuperSuperSuperSuperLongType: 301}]\n  ) {\n  cover_photo {\n    image(\n      width: $PROJECT_UNIT_CARD_SIZE,\n      height: $PROJECT_UNIT_CARD_SIZE,\n      sizing: \"cover-fill\",\n      scale: $scale,\n    ) {\n      uri\n    }\n  }\n}") ? ;
    assert_eq ! (formatted , "query Query(\n  $pageID: ID!\n  $scale: Float\n  $PROJECT_UNIT_PROFILE_PICTURE_SIZE: Int\n  $PROJECT_UNIT_CARD_SIZE: Int\n)\n@argumentDefinitions(\n  count: {\n    type: \"Int\"\n    defaultValue: 20\n    someSuperSuperSuperSuperLongType: 301\n  }\n  test: [\n    { type: \"Int\", defaultValue: 20, someSuperSuperSuperSuperLongType: 301 }\n  ]\n) {\n  cover_photo {\n    image(\n      width: $PROJECT_UNIT_CARD_SIZE\n      height: $PROJECT_UNIT_CARD_SIZE\n      sizing: \"cover-fill\"\n      scale: $scale\n    ) {\n      uri\n    }\n  }\n}");
    Ok(())
}
#[test]
fn test_trailing_graphql_trailing_commanone_format_1_3bd343f6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("graphql")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("query Query(\n  $pageID: ID!\n  $scale: Float\n  $PROJECT_UNIT_PROFILE_PICTURE_SIZE: Int\n  $PROJECT_UNIT_CARD_SIZE: Int\n) @argumentDefinitions(\n    count: {type: \"Int\", defaultValue: 20, someSuperSuperSuperSuperLongType: 301}\n    test: [{type: \"Int\", defaultValue: 20, someSuperSuperSuperSuperLongType: 301}]\n  ) {\n  cover_photo {\n    image(\n      width: $PROJECT_UNIT_CARD_SIZE,\n      height: $PROJECT_UNIT_CARD_SIZE,\n      sizing: \"cover-fill\",\n      scale: $scale,\n    ) {\n      uri\n    }\n  }\n}") ? ;
    assert_eq ! (formatted , "query Query(\n  $pageID: ID!\n  $scale: Float\n  $PROJECT_UNIT_PROFILE_PICTURE_SIZE: Int\n  $PROJECT_UNIT_CARD_SIZE: Int\n)\n@argumentDefinitions(\n  count: {\n    type: \"Int\"\n    defaultValue: 20\n    someSuperSuperSuperSuperLongType: 301\n  }\n  test: [\n    { type: \"Int\", defaultValue: 20, someSuperSuperSuperSuperLongType: 301 }\n  ]\n) {\n  cover_photo {\n    image(\n      width: $PROJECT_UNIT_CARD_SIZE\n      height: $PROJECT_UNIT_CARD_SIZE\n      sizing: \"cover-fill\"\n      scale: $scale\n    ) {\n      uri\n    }\n  }\n}");
    Ok(())
}
