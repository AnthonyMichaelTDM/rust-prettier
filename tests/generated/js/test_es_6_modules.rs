#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_export_default_arrow_expression_js_format_1_44a552bc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("export default () => {};")?;
    assert_eq!(formatted, "export default () => {};");
    Ok(())
}
#[test]
fn test_export_default_call_expression_js_format_1_abf578c9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("export default foo()")?;
    assert_eq!(formatted, "export default foo();");
    Ok(())
}
#[test]
fn test_export_default_class_declaration_js_format_1_c782546d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("export default class Foo {}")?;
    assert_eq!(formatted, "export default class Foo {}");
    Ok(())
}
#[test]
fn test_export_default_class_expression_js_format_1_9bb43d23() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("export default (class foobar {})")?;
    assert_eq!(formatted, "export default (class foobar {});");
    Ok(())
}
#[test]
fn test_export_default_function_declaration_js_format_1_84165f8c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("export default function() {}")?;
    assert_eq!(formatted, "export default function () {}");
    Ok(())
}
#[test]
fn test_export_default_function_declaration_async_js_format_1_8099ecfc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("export default async function foo() {}")?;
    assert_eq!(formatted, "export default async function foo() {}");
    Ok(())
}
#[test]
fn test_export_default_function_declaration_named_js_format_1_42ad7490() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("export default function f(){}")?;
    assert_eq!(formatted, "export default function f() {}");
    Ok(())
}
#[test]
fn test_export_default_function_expression_js_format_1_777f8cb6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("export default (function() {});")?;
    assert_eq!(formatted, "export default (function () {});");
    Ok(())
}
#[test]
fn test_export_default_function_expression_named_js_format_1_e0439402() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("export default (function f(){})")?;
    assert_eq!(formatted, "export default (function f() {});");
    Ok(())
}
#[test]
fn test_export_default_new_expression_js_format_1_9765f6fb() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("export default new Foo();")?;
    assert_eq!(formatted, "export default new Foo();");
    Ok(())
}
