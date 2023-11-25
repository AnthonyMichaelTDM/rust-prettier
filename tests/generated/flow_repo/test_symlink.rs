#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_bar_js_format_1_602046af() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("export type Foo = { x: number; }")?;
    assert_eq!(formatted, "export type Foo = { x: number };");
    Ok(())
}
#[test]
fn test_foo_js_format_1_602046af() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("export type Foo = { x: number; }")?;
    assert_eq!(formatted, "export type Foo = { x: number };");
    Ok(())
}
#[test]
fn test_qux_js_format_1_2ee67941() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("import type { Foo } from './bar.js';\n({ x: \"\" }: Foo);")?;
    assert_eq!(
        formatted,
        "import type { Foo } from \"./bar.js\";\n({ x: \"\" }: Foo);"
    );
    Ok(())
}
