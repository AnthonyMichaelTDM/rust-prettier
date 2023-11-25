#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_custom_parser_css_format_1_b14a6c04() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("---mycustomparser\ntitle: Title\ndescription: Description\n---\n/* comment */\n.something \n{}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "---mycustomparser\ntitle: Title\ndescription: Description\n---\n\n/* comment */\n.something {\n}");
}
#[test]
fn test_empty_css_format_1_750c7f52() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\n---\n/* comment */\n.something\n{}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "---\n---\n\n/* comment */\n.something {\n}");
}
