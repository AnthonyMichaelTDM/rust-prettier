#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_simple_md_prose_wrapalways_format_1_c5b97e92() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .prose_wrap("always")
        .parsers(vec!["markdown"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[alpha]: http://example.com");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[alpha]: http://example.com");
}
#[test]
fn test_space_md_prose_wrapalways_format_1_a32fe84d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .prose_wrap("always")
        .print_width(80)
        .parsers(vec!["markdown"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[alpha]: <http://example.com 123> \"title\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[alpha]: <http://example.com 123> \"title\"");
}
#[test]
fn test_title_md_prose_wrapalways_format_1_0509eec6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .prose_wrap("always")
        .parsers(vec!["markdown"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[alpha]: http://example.com \"title\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[alpha]: http://example.com \"title\"");
}
