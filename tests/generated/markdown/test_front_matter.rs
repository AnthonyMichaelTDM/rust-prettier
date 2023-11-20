#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_custom_parser_md_format_1_8c835e39() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("---mycustomparser\n- hello:    world\n-         123\n---\n\n__123__\n**456**");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "---mycustomparser\n- hello:    world\n-         123\n---\n\n**123**\n**456**"
    );
}
#[test]
fn test_empty_md_format_1_7bf837f2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\n---\n\n__123__\n**456**");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "---\n---\n\n**123**\n**456**");
}
#[test]
fn test_empty_2_md_format_1_bb46c55b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\n---\n\n# Title 1\n\nHello, world\n\n---\n\ntext");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "---\n---\n\n# Title 1\n\nHello, world\n\n---\n\ntext"
    );
}
