#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_trailing_comma_notjson_trailing_commaall_format_1_02e9caad() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format("{\n  \"k1\": \"v1\",\n  \"k2\": \"v2\",\n  \"k3\": \"v3\"\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "{\n  \"k1\": \"v1\",\n  \"k2\": \"v2\",\n  \"k3\": \"v3\"\n}"
    );
}
#[test]
fn test_trailing_comma_notjson_trailing_commaall_format_2_02e9caad() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format("{\n  \"k1\": \"v1\",\n  \"k2\": \"v2\",\n  \"k3\": \"v3\"\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "{\n  k1: \"v1\",\n  k2: \"v2\",\n  k3: \"v3\",\n}"
    );
}
#[test]
fn test_trailing_comma_notjson_trailing_commaes_5_format_1_02e9caad() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format("{\n  \"k1\": \"v1\",\n  \"k2\": \"v2\",\n  \"k3\": \"v3\"\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "{\n  \"k1\": \"v1\",\n  \"k2\": \"v2\",\n  \"k3\": \"v3\"\n}"
    );
}
#[test]
fn test_trailing_comma_notjson_trailing_commaes_5_format_2_02e9caad() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format("{\n  \"k1\": \"v1\",\n  \"k2\": \"v2\",\n  \"k3\": \"v3\"\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "{\n  k1: \"v1\",\n  k2: \"v2\",\n  k3: \"v3\",\n}"
    );
}
#[test]
fn test_trailing_comma_notjson_format_1_02e9caad() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format("{\n  \"k1\": \"v1\",\n  \"k2\": \"v2\",\n  \"k3\": \"v3\"\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "{\n  \"k1\": \"v1\",\n  \"k2\": \"v2\",\n  \"k3\": \"v3\"\n}"
    );
}
