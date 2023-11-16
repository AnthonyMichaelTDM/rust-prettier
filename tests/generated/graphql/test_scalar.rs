#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_scalar_graphql_format_1_4f6b1a91() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("scalar Color @deprecated");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "scalar Color @deprecated");
}
