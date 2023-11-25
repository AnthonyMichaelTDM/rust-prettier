#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_array_pattern_ts_format_1_3fbf99b0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(6)
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("let []<|>: T = [];")?;
    assert_eq!(formatted, "let []<|>: T = [];");
    Ok(())
}
#[test]
fn test_arrow_function_type_ts_format_1_8e7f7550() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(14)
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("type foo = () <|>=> boolean;")?;
    assert_eq!(formatted, "type foo = () <|>=> boolean;");
    Ok(())
}
#[test]
fn test_class_property_ts_format_1_3c4e3ff5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(15)
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("class A {\n  foo<|>: A\n}")?;
    assert_eq!(formatted, "class A {\n  foo<|>: A;\n}");
    Ok(())
}
#[test]
fn test_function_return_type_ts_format_1_dc16bc17() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(12)
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("function a()<|>: boolean {}")?;
    assert_eq!(formatted, "function a()<|>: boolean {}");
    Ok(())
}
#[test]
fn test_identifier_1_ts_format_1_015a8908() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(5)
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("let i<|>: T;")?;
    assert_eq!(formatted, "let i<|>: T;");
    Ok(())
}
#[test]
fn test_identifier_2_ts_format_1_14212fe0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(6)
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("let i:<|> T;")?;
    assert_eq!(formatted, "let i:<|> T;");
    Ok(())
}
#[test]
fn test_identifier_3_ts_format_1_66fdcf16() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(5)
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("let i<|> : T;")?;
    assert_eq!(formatted, "let i<|>: T;");
    Ok(())
}
#[test]
fn test_method_signature_ts_format_1_256f80a7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(21)
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("interface A {\n  foo()<|>: T;\n}")?;
    assert_eq!(formatted, "interface A {\n  foo()<|>: T;\n}");
    Ok(())
}
#[test]
fn test_property_signature_ts_format_1_fc2f9444() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(19)
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("interface A {\n  foo<|>: T;\n}")?;
    assert_eq!(formatted, "interface A {\n  foo<|>: T;\n}");
    Ok(())
}
#[test]
fn test_rest_ts_format_1_372a9c4f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(20)
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("function foo(...args<|>: any) { }")?;
    assert_eq!(formatted, "function foo(...args<|>: any) {}");
    Ok(())
}
