#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_lib_a_js_format_1_a8c10ff4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("declare function foo(x: number): void;")?;
    assert_eq!(formatted, "declare function foo(x: number): void;");
    Ok(())
}
#[test]
fn test_lib_b_js_format_1_da0eef14() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("declare function bar(x: string): void;")?;
    assert_eq!(formatted, "declare function bar(x: string): void;");
    Ok(())
}
#[test]
fn test_libtest_js_format_1_3e4aeb59() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo(123);\nbar(123);")?;
    assert_eq!(formatted, "foo(123);\nbar(123);");
    Ok(())
}
