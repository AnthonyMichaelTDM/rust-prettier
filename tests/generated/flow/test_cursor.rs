#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_declare_module_exports_js_format_1_1b40e244() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(22)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("declare module.exports<|>: A;")?;
    assert_eq!(formatted, "declare module.exports<|>: A;");
    Ok(())
}
#[test]
fn test_function_predicate_js_format_1_b2251ee2() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(12)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("function a()<|>: %checks(a) {}")?;
    assert_eq!(formatted, "function a()<|>: %checks(a) {}");
    Ok(())
}
#[test]
fn test_function_predicate_2_js_format_1_ead1be6d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(12)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("function a()<|>: %checks {}")?;
    assert_eq!(formatted, "function a()<|>: %checks {}");
    Ok(())
}
#[test]
fn test_function_return_type_js_format_1_5a37a184() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("function a(): boolean {}")?;
    assert_eq!(formatted, "function a(): boolean {}");
    Ok(())
}
#[test]
fn test_function_return_type_and_predicate_js_format_1_8b7bcdc4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(12)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("function a()<|>: boolean %checks(a) {}")?;
    assert_eq!(formatted, "function a()<|>: boolean %checks(a) {}");
    Ok(())
}
#[test]
fn test_function_return_type_and_predicate_2_js_format_1_3b33c345() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(12)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("function a()<|>: boolean %checks {}")?;
    assert_eq!(formatted, "function a()<|>: boolean %checks {}");
    Ok(())
}
#[test]
fn test_function_type_parameter_bound_js_format_1_e05e1c2d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(12)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("function a<T<|>: X>() {}")?;
    assert_eq!(formatted, "function a<T<|>: X>() {}");
    Ok(())
}
#[test]
fn test_type_cast_expression_js_format_1_7876a101() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(2)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("(a<|>: A);")?;
    assert_eq!(formatted, "(a<|>: A);");
    Ok(())
}
