#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_intersection_ts_format_1_6cc9db01() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format("type Example = {\n  [A in B]: T;\n} & {\n  [A in B]: T;\n};");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "type Example = {\n  [A in B]: T;\n} & {\n  [A in B]: T;\n};"
    );
}
#[test]
fn test_issue_11098_ts_format_1_5851898c() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("type Type = {\n  // comment\n  readonly [T in number];\n};\n\ntype Type = {\n  // comment1\n  // comment2\n  readonly [T in number];\n};\n\ntype Type = {\n  // comment\n  +readonly [T in number];\n};\n\ntype Type = {\n  // comment\n  -readonly [T in number];\n};\n\ntype Type = {\n  // comment\n  +    readonly [T in number];\n};\n\ntype Type = {\n  // comment\n  +readonly     [T in number];\n};\n\ntype Type = {\n  // comment\n  readonly       [T in number];\n};\n\ntype Type = {\n  // comment\n  [T in number];\n};\n\ntype Type = {\n  readonly\n  // comment\n  [T in number];\n};\n\ntype Type = {\n  readonly // foo\n  /* bar */ [T in number];\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type Type = {\n  // comment\n  readonly [T in number];\n};\n\ntype Type = {\n  // comment1\n  // comment2\n  readonly [T in number];\n};\n\ntype Type = {\n  // comment\n  +readonly [T in number];\n};\n\ntype Type = {\n  // comment\n  -readonly [T in number];\n};\n\ntype Type = {\n  // comment\n  +readonly [T in number];\n};\n\ntype Type = {\n  // comment\n  +readonly [T in number];\n};\n\ntype Type = {\n  // comment\n  readonly [T in number];\n};\n\ntype Type = {\n  // comment\n  [T in number];\n};\n\ntype Type = {\n  // comment\n  readonly [T in number];\n};\n\ntype Type = {\n  // foo\n  /* bar */ readonly [T in number];\n};");
}
#[test]
fn test_mapped_type_ts_format_1_cee8e03f() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("type Keys = 'option1' | 'option2';\ntype A = { [K in Keys] };\ntype B = { [K in Keys]+? };") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type Keys = \"option1\" | \"option2\";\ntype A = { [K in Keys] };\ntype B = { [K in Keys]+? };");
}
