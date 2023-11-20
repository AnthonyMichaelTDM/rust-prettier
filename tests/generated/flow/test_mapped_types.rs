#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_comments_js_babel_flow_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_comments_js_babel_ts_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_comments_js_typescript_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_comments_js_format_1_04b1de52() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Type = {\n  // comment\n  +[T in number]: number;\n};\n\ntype Type = {\n  // comment1\n  // comment2\n  +[T in number]: number;\n};\n\ntype Type = {\n  // comment\n  -[T in number]: number;\n};\n\ntype Type = {\n  // comment\n  +    [T in number]: number;\n};\n\ntype Type = {\n  // comment\n  +     [T in number]: number;\n};\n\ntype Type = {\n  // comment\n  +       [T in number]: number;\n};\n\ntype Type = {\n  // comment\n  [T in number]: number;\n};\n\ntype Type = {\n  + // foo\n  /* bar */ [T in number]: number;\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type Type = {\n  // comment\n  +[T in number]: number,\n};\n\ntype Type = {\n  // comment1\n  // comment2\n  +[T in number]: number,\n};\n\ntype Type = {\n  // comment\n  -[T in number]: number,\n};\n\ntype Type = {\n  // comment\n  +[T in number]: number,\n};\n\ntype Type = {\n  // comment\n  +[T in number]: number,\n};\n\ntype Type = {\n  // comment\n  +[T in number]: number,\n};\n\ntype Type = {\n  // comment\n  [T in number]: number,\n};\n\ntype Type = {\n  +[/* bar */ T in number]: number, // foo\n};");
}
#[test]
fn test_mapped_types_js_babel_flow_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_mapped_types_js_babel_ts_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_mapped_types_js_typescript_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_mapped_types_js_format_1_4a7b3096() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("//@flow\n\ntype Mapped = {[key in keyof O]: O[key]};\ntype MappedLong = {[key in keyof AReallyLongNameThatShouldReallyMostDefinitelyCauseALineWrap]: number};\ntype MappedWithVariance = {+[key in keyof O]: number};\ntype MappedWithVarianceLong = {-[key in keyof AReallyLongNameThatShouldReallyMostDefinitelyCauseALineWrap]: number};\ntype MappedWithPlusOptional = {[key in keyof O]+?: number};\ntype MappedWithMinusOptional = {[key in keyof O]-?: number};\ntype MappedWithOptional = {[key in keyof O]?: number};\ntype MappedWithPlusOptionalLong = {[key in keyof AReallyLongNameThatShouldReallyMostDefinitelyCauseALineWrap]+?: number};\ntype MappedWithMinusOptionalLong = {[key in keyof AReallyLongNameThatShouldReallyMostDefinitelyCauseALineWrap]-?: number};\ntype MappedWithOptionalLong = {[key in keyof AReallyLongNameThatShouldReallyMostDefinitelyCauseALineWrap]?: number};\ntype MappedWithOptionalAndVariance = {+[key in keyof O]+?: number};\ntype MappedWithOptionalAndVarianceLong = {+[key in keyof AReallyLongNameThatShouldReallyMostDefinitelyCauseALineWrap]+?: number};\ntype MappedWithComments = {/*comment about variance */+[key /* comment about key name */ in /* comment before source type */ keyof O /* comment after source type */]/* comment about optionality */? /*comment before colon */:number /*comment about the prop type */};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "//@flow\n\ntype Mapped = { [key in keyof O]: O[key] };\ntype MappedLong = {\n  [key in keyof AReallyLongNameThatShouldReallyMostDefinitelyCauseALineWrap]: number,\n};\ntype MappedWithVariance = { +[key in keyof O]: number };\ntype MappedWithVarianceLong = {\n  -[key in keyof AReallyLongNameThatShouldReallyMostDefinitelyCauseALineWrap]: number,\n};\ntype MappedWithPlusOptional = { [key in keyof O]+?: number };\ntype MappedWithMinusOptional = { [key in keyof O]-?: number };\ntype MappedWithOptional = { [key in keyof O]?: number };\ntype MappedWithPlusOptionalLong = {\n  [key in keyof AReallyLongNameThatShouldReallyMostDefinitelyCauseALineWrap]+?: number,\n};\ntype MappedWithMinusOptionalLong = {\n  [key in keyof AReallyLongNameThatShouldReallyMostDefinitelyCauseALineWrap]-?: number,\n};\ntype MappedWithOptionalLong = {\n  [key in keyof AReallyLongNameThatShouldReallyMostDefinitelyCauseALineWrap]?: number,\n};\ntype MappedWithOptionalAndVariance = { +[key in keyof O]+?: number };\ntype MappedWithOptionalAndVarianceLong = {\n  +[key in keyof AReallyLongNameThatShouldReallyMostDefinitelyCauseALineWrap]+?: number,\n};\ntype MappedWithComments = {\n  /*comment about variance */ +[key /* comment about key name */ in /* comment before source type */ keyof O /* comment after source type */ /* comment about optionality */ /*comment before colon */]?: number /*comment about the prop type */,\n};");
}
#[test]
fn test_ts_compatibility_js_babel_flow_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_ts_compatibility_js_format_1_cd666787() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// Tests that on single-line mapped types Flow and TS make the same formatting decisions\n\ntype Test = {[key in T]: number};\ntype Test = {[key in keyof T]: number};\ntype Test = {[otherKeyName     in keyof T]: number};\ntype Test = {[key in T]:number};\ntype Test = {[key in T]+?:number};\ntype Test = {[key in T]-?:                   number};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// Tests that on single-line mapped types Flow and TS make the same formatting decisions\n\ntype Test = { [key in T]: number };\ntype Test = { [key in keyof T]: number };\ntype Test = { [otherKeyName in keyof T]: number };\ntype Test = { [key in T]: number };\ntype Test = { [key in T]+?: number };\ntype Test = { [key in T]-?: number };");
}
