#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_empty_md_prose_wrapalways_format_1_3a055119() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("+++\n+++");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "+++\n+++");
}
#[test]
fn test_simple_md_prose_wrapalways_format_1_c4953209() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("+++\ndate: '2017-10-10T22:49:47.369Z'\ntitle: 'My Post Title'\ncategories: ['foo', 'bar']\n+++") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "+++\ndate: '2017-10-10T22:49:47.369Z'\ntitle: 'My Post Title'\ncategories: ['foo', 'bar']\n+++");
}
