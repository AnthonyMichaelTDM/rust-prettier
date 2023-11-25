#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_decorator_auto_accessors_new_line_ts_format_1_da0b4aaf() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class Foo {\n  accessor\n  [\"bar\"];\n}\n\nclass Foo {\n  static accessor\n  bar;\n}\n\nclass Foo {\n  accessor\n  bar;\n}") ? ;
    assert_eq ! (formatted , "class Foo {\n  accessor;\n  [\"bar\"];\n}\n\nclass Foo {\n  static accessor;\n  bar;\n}\n\nclass Foo {\n  accessor;\n  bar;\n}");
    Ok(())
}
#[test]
fn test_parenthesized_decorators_call_expression_ts_format_1_b2f4d19f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("@(test().x(\"global\").y())\nclass X {}")?;
    assert_eq!(formatted, "@(test().x(\"global\").y())\nclass X {}");
    Ok(())
}
#[test]
fn test_parenthesized_decorators_tagged_template_ts_format_1_e0f70504() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("class Test {\n  @(foo\\`bar\\`)\n  text: string = \"text\"\n}")?;
    assert_eq!(
        formatted,
        "class Test {\n  @(foo\\`bar\\`)\n  text: string = \"text\";\n}"
    );
    Ok(())
}
