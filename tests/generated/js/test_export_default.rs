#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_binary_and_template_js_format_1_1d00c9f8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("export default (function() {} + foo)\\`\\`;")?;
    assert_eq!(formatted, "export default (function () {} + foo)\\`\\`;");
    Ok(())
}
#[test]
fn test_body_js_format_1_830b26b9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("export default (class {}[1] = 1);")?;
    assert_eq!(formatted, "export default (class {}[1] = 1);");
    Ok(())
}
#[test]
fn test_class_instance_js_format_1_ac3a2d01() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("export default (class {}.getInstance());")?;
    assert_eq!(formatted, "export default (class {}.getInstance());");
    Ok(())
}
#[test]
fn test_function_in_template_js_format_1_57c6e4e8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("export default (function templ() {})\\`foo\\`;")?;
    assert_eq!(formatted, "export default (function templ() {})\\`foo\\`;");
    Ok(())
}
#[test]
fn test_function_tostring_js_format_1_e727cc0c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("export default (function() {}).toString();")?;
    assert_eq!(formatted, "export default (function () {}.toString());");
    Ok(())
}
#[test]
fn test_iife_js_format_1_a3766151() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("export default (function foo() {})();")?;
    assert_eq!(formatted, "export default (function foo() {})();");
    Ok(())
}
