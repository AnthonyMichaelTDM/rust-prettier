#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_await_using_with_type_declaration_ts_format_1_c6d58c26() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("{\n    await     using    foo: Foo   = new Foo();\n}")?;
    assert_eq!(formatted, "{\n  await using foo: Foo = new Foo();\n}");
    Ok(())
}
#[test]
fn test_using_with_type_declaration_ts_format_1_c56a45c1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{\n    using     foo: Foo =    new Foo();\n}")?;
    assert_eq!(formatted, "{\n  using foo: Foo = new Foo();\n}");
    Ok(())
}
