#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_declare_module_exports_js_format_1_1b40e244() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("declare module.exports<|>: A;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "declare module.exports<|>: A;");
}
#[test]
fn test_function_predicate_js_format_1_b2251ee2() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("function a()<|>: %checks(a) {}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "function a()<|>: %checks(a) {}");
}
#[test]
fn test_function_predicate_2_js_format_1_ead1be6d() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("function a()<|>: %checks {}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "function a()<|>: %checks {}");
}
#[test]
fn test_function_return_type_js_format_1_5a37a184() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("function a(): boolean {}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "function a(): boolean {}");
}
#[test]
fn test_function_return_type_and_predicate_js_format_1_8b7bcdc4() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("function a()<|>: boolean %checks(a) {}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "function a()<|>: boolean %checks(a) {}");
}
#[test]
fn test_function_return_type_and_predicate_2_js_format_1_3b33c345() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("function a()<|>: boolean %checks {}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "function a()<|>: boolean %checks {}");
}
#[test]
fn test_function_type_parameter_bound_js_format_1_e05e1c2d() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("function a<T<|>: X>() {}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "function a<T<|>: X>() {}");
}
#[test]
fn test_type_cast_expression_js_format_1_7876a101() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("(a<|>: A);");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "(a<|>: A);");
}
