#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_computed_members_ts_babel_ts_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_computed_members_ts_format_1_7612997c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("enum A {\n  [i++],\n}\n\nconst bar = \"bar\"\nenum B {\n  [bar] = 2,\n}\n\nconst foo = () => \"foo\";\nenum C {\n  [foo()] = 2,\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "enum A {\n  [i++],\n}\n\nconst bar = \"bar\";\nenum B {\n  [bar] = 2,\n}\n\nconst foo = () => \"foo\";\nenum C {\n  [foo()] = 2,\n}");
}
#[test]
fn test_enum_ts_format_1_329aea72() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("enum Direction {\n    Up = 1,\n    Down,\n    Left,\n    Right\n}\n\nenum FileAccess {\n    // constant members\n    None,\n    Read    = 1 << 1,\n    Write   = 1 << 2,\n    ReadWrite  = Read | Write,\n    // computed member\n    G = \"123\".length\n}\n\nenum Empty {\n}\n\nconst enum Enum {\n    A = 1,\n    B = A * 2\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "enum Direction {\n  Up = 1,\n  Down,\n  Left,\n  Right,\n}\n\nenum FileAccess {\n  // constant members\n  None,\n  Read = 1 << 1,\n  Write = 1 << 2,\n  ReadWrite = Read | Write,\n  // computed member\n  G = \"123\".length,\n}\n\nenum Empty {}\n\nconst enum Enum {\n  A = 1,\n  B = A * 2,\n}");
}
#[test]
fn test_multiline_ts_format_1_8c6383dc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("declare enum\nE\n{}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "declare enum E {}");
}
