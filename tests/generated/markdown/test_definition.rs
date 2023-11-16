#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_simple_md_prose_wrapalways_format_1_c5b97e92() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("[alpha]: http://example.com");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[alpha]: http://example.com");
}
#[test]
fn test_space_md_prose_wrapalways_format_1_a32fe84d() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("[alpha]: <http://example.com 123> \"title\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[alpha]: <http://example.com 123> \"title\"");
}
#[test]
fn test_title_md_prose_wrapalways_format_1_0509eec6() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("[alpha]: http://example.com \"title\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[alpha]: http://example.com \"title\"");
}
