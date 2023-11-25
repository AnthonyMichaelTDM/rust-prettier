#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_foo_js_format_1_d7a10726() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/*\n * @providesModule Foo\n */")?;
    assert_eq!(formatted, "/*\n * @providesModule Foo\n */");
    Ok(())
}
#[test]
fn test_test_js_format_1_bd853d4d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/**\n * @flow\n */\n\nrequire('Foo');")?;
    assert_eq!(formatted, "/**\n * @flow\n */\n\nrequire(\"Foo\");");
    Ok(())
}
