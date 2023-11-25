#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_delete_variable_js_format_1_e90281c6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("function foo() {\n  var bar = 1;\n  delete bar;}")?;
    assert_eq!(
        formatted,
        "function foo() {\n  var bar = 1;\n  delete bar;\n}"
    );
    Ok(())
}
#[test]
fn test_eval_arguments_js_format_1_0d73a9e3() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("function myfunc() {\n  eval = 1\n  arguments = arguments;\n}")?;
    assert_eq!(
        formatted,
        "function myfunc() {\n  eval = 1;\n  arguments = arguments;\n}"
    );
    Ok(())
}
#[test]
fn test_eval_arguments_binding_js_format_1_e957d00a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("function myfunc() {\n  var eval\n  var arguments;\n}")?;
    assert_eq!(
        formatted,
        "function myfunc() {\n  var eval;\n  var arguments;\n}"
    );
    Ok(())
}
#[test]
fn test_function_declaration_in_if_js_format_1_ee875618() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("if (false) function foo(){}")?;
    assert_eq!(formatted, "if (false) function foo() {}");
    Ok(())
}
#[test]
fn test_function_declaration_in_while_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_function_declaration_in_while_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_function_declaration_in_while_js_flow_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_function_declaration_in_while_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_function_declaration_in_while_js_format_1_3f60482f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("while (false) function foo(){}")?;
    assert_eq!(formatted, "while (false) function foo() {}");
    Ok(())
}
#[test]
fn test_labeled_function_declaration_js_format_1_c36ca52f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo: function bar() {}")?;
    assert_eq!(formatted, "foo: function bar() {}");
    Ok(())
}
