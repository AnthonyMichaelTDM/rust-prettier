#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_7b6ac6e8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["markdown"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_ http://www.example.com:80/_a ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_ http://www.example.com:80/_a _");
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_8d9b49e4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["markdown"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_ http://www.example.com:80/_a_ ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_ http://www.example.com:80/_a_ \\\\_");
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_bd846ff9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_ http://www.example.com:80/_a");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_ http://www.example.com:80/_a_");
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_0d0803e2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["markdown"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_ http://www.example.com:80/_a_/ ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_ http://www.example.com:80/_a_/ \\\\_");
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_157040bf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["markdown"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_ http://www.example.com:80/_a_/_ ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_ http://www.example.com:80/_a_/\\\\_ \\\\_");
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_b48088c9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_ http://www.example.com:80/_a_/");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_ http://www.example.com:80/_a_/\\\\_");
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_8bd96f50() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["markdown"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_ http://www.example.com:80/_a_/_");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_ http://www.example.com:80/_a_/\\\\_\\\\_");
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_8923996e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_ http://www.example.com:80/_a__ ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_ http://www.example.com:80/_a__ _");
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_2e4043dd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_ http://www.example.com:80/_a_");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\\\_ http://www.example.com:80/_a__");
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_90ba1d99() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_ http://www.example.com:80/_a__");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\\\_ http://www.example.com:80/_a___");
}
#[test]
fn test_snippet_httpwww_example_com_80_a_b_format_1_dc86dfcf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["markdown"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_ http://www.example.com:80/_a_b ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_ http://www.example.com:80/_a_b _");
}
#[test]
fn test_snippet_httpwww_example_com_80_a_b_format_1_85c0ee84() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_ http://www.example.com:80/_a_b");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_ http://www.example.com:80/_a_b_");
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_55ebe5ee() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_http://www.example.com:80/_a ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_http://www.example.com:80/_a _");
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_d7f96584() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_http://www.example.com:80/_a_ ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_http://www.example.com:80/_a_ \\\\_");
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_dad5a8fc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_http://www.example.com:80/_a");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_http://www.example.com:80/_a_");
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_d2bca30f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["markdown"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_http://www.example.com:80/_a_/ ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_http://www.example.com:80/_a_/ \\\\_");
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_ea89d285() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["markdown"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_http://www.example.com:80/_a_/_ ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_http://www.example.com:80/_a_/\\\\_ \\\\_");
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_a4670c40() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["markdown"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_http://www.example.com:80/_a_/");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_http://www.example.com:80/_a_/\\\\_");
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_3a2341ff() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_http://www.example.com:80/_a_/_");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_http://www.example.com:80/_a_/\\\\_\\\\_");
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_9d1c75ea() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["markdown"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_http://www.example.com:80/_a__ ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_http://www.example.com:80/_a__ _");
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_54678ec1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["markdown"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_http://www.example.com:80/_a_");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\\\_http://www.example.com:80/_a__");
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_b7422d5c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_http://www.example.com:80/_a__");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\\\_http://www.example.com:80/_a___");
}
#[test]
fn test_snippet_httpwww_example_com_80_a_b_format_1_c9c09612() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_http://www.example.com:80/_a_b ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_http://www.example.com:80/_a_b _");
}
#[test]
fn test_snippet_httpwww_example_com_80_a_b_format_1_4ff6fbbe() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_http://www.example.com:80/_a_b");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_http://www.example.com:80/_a_b_");
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_266da82c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["markdown"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("http://www.example.com:80/_");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "http://www.example.com:80/_a");
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_d59dc3a3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("http://www.example.com:80/_a");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "http://www.example.com:80/_a_");
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_dc5b174e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["markdown"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("http://www.example.com:80/_a_");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "http://www.example.com:80/_a_/");
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_39280ae3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("http://www.example.com:80/_a_/");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "http://www.example.com:80/_a_/_");
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_54137733() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["markdown"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("http://www.example.com:80/_a_");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "http://www.example.com:80/_a__");
}
#[test]
fn test_snippet_httpwww_example_com_80_a_b_format_1_9013d69a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("http://www.example.com:80/_a_");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "http://www.example.com:80/_a_b");
}
