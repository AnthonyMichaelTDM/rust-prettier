#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_list_md_prose_wrapalways_format_1_f0a35be8() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("- * * *\n+ - - -");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- ***\n\n* ---");
}
#[test]
fn test_simple_md_prose_wrapalways_format_1_03845f8c() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("***");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "---");
}
