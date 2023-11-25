#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_intersection_ts_format_1_6cc9db01() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("type Example = {\n  [A in B]: T;\n} & {\n  [A in B]: T;\n};")?;
    assert_eq!(
        formatted,
        "type Example = {\n  [A in B]: T;\n} & {\n  [A in B]: T;\n};"
    );
    Ok(())
}
#[test]
fn test_issue_11098_ts_format_1_5851898c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Type = {\n  // comment\n  readonly [T in number];\n};\n\ntype Type = {\n  // comment1\n  // comment2\n  readonly [T in number];\n};\n\ntype Type = {\n  // comment\n  +readonly [T in number];\n};\n\ntype Type = {\n  // comment\n  -readonly [T in number];\n};\n\ntype Type = {\n  // comment\n  +    readonly [T in number];\n};\n\ntype Type = {\n  // comment\n  +readonly     [T in number];\n};\n\ntype Type = {\n  // comment\n  readonly       [T in number];\n};\n\ntype Type = {\n  // comment\n  [T in number];\n};\n\ntype Type = {\n  readonly\n  // comment\n  [T in number];\n};\n\ntype Type = {\n  readonly // foo\n  /* bar */ [T in number];\n};") ? ;
    assert_eq ! (formatted , "type Type = {\n  // comment\n  readonly [T in number];\n};\n\ntype Type = {\n  // comment1\n  // comment2\n  readonly [T in number];\n};\n\ntype Type = {\n  // comment\n  +readonly [T in number];\n};\n\ntype Type = {\n  // comment\n  -readonly [T in number];\n};\n\ntype Type = {\n  // comment\n  +readonly [T in number];\n};\n\ntype Type = {\n  // comment\n  +readonly [T in number];\n};\n\ntype Type = {\n  // comment\n  readonly [T in number];\n};\n\ntype Type = {\n  // comment\n  [T in number];\n};\n\ntype Type = {\n  // comment\n  readonly [T in number];\n};\n\ntype Type = {\n  // foo\n  /* bar */ readonly [T in number];\n};");
    Ok(())
}
#[test]
fn test_mapped_type_ts_format_1_cee8e03f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Keys = 'option1' | 'option2';\ntype A = { [K in Keys] };\ntype B = { [K in Keys]+? };") ? ;
    assert_eq ! (formatted , "type Keys = \"option1\" | \"option2\";\ntype A = { [K in Keys] };\ntype B = { [K in Keys]+? };");
    Ok(())
}
