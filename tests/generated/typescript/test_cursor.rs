#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_array_pattern_ts_format_1_3fbf99b0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["typescript", "flow"])
        .cursor_offset(6)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("let []<|>: T = [];");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "let []<|>: T = [];");
}
#[test]
fn test_arrow_function_type_ts_format_1_8e7f7550() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["typescript", "flow"])
        .cursor_offset(14)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("type foo = () <|>=> boolean;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "type foo = () <|>=> boolean;");
}
#[test]
fn test_class_property_ts_format_1_3c4e3ff5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript", "flow"])
        .print_width(80)
        .cursor_offset(15)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("class A {\n  foo<|>: A\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "class A {\n  foo<|>: A;\n}");
}
#[test]
fn test_function_return_type_ts_format_1_dc16bc17() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript", "flow"])
        .print_width(80)
        .cursor_offset(12)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("function a()<|>: boolean {}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "function a()<|>: boolean {}");
}
#[test]
fn test_identifier_1_ts_format_1_015a8908() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .cursor_offset(5)
        .parsers(vec!["typescript", "flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("let i<|>: T;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "let i<|>: T;");
}
#[test]
fn test_identifier_2_ts_format_1_14212fe0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript", "flow"])
        .print_width(80)
        .cursor_offset(6)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("let i:<|> T;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "let i:<|> T;");
}
#[test]
fn test_identifier_3_ts_format_1_66fdcf16() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .cursor_offset(5)
        .parsers(vec!["typescript", "flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("let i<|> : T;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "let i<|>: T;");
}
#[test]
fn test_method_signature_ts_format_1_256f80a7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript", "flow"])
        .print_width(80)
        .cursor_offset(21)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("interface A {\n  foo()<|>: T;\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "interface A {\n  foo()<|>: T;\n}");
}
#[test]
fn test_property_signature_ts_format_1_fc2f9444() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(19)
        .parsers(vec!["typescript", "flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("interface A {\n  foo<|>: T;\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "interface A {\n  foo<|>: T;\n}");
}
#[test]
fn test_rest_ts_format_1_372a9c4f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .cursor_offset(20)
        .parsers(vec!["typescript", "flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("function foo(...args<|>: any) { }");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "function foo(...args<|>: any) {}");
}
