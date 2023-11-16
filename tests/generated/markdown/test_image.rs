#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_simple_md_prose_wrapalways_format_1_a19909c1() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("![hello](http://example.com/image.png)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "![hello](http://example.com/image.png)");
}
#[test]
fn test_title_md_prose_wrapalways_format_1_da6ac7e3() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("![hello](http://example.com/image.png \"title\")");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "![hello](http://example.com/image.png \"title\")"
    );
}
