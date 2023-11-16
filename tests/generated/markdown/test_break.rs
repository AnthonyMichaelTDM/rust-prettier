#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_simple_md_prose_wrapalways_format_1_95d7e4b0() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("123  \n456\n\n123\\\\\n456\n\n- 123  \n  123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "123  \n456\n\n123\\\\\n456\n\n- 123  \n  123");
}
