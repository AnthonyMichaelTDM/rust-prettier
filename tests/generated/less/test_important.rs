#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_important_less_format_1_6bddc39f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("less")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "div {\n  max-height: none !important;\n}\n\n.important {\n  .foo() !important;\n}",
    )?;
    assert_eq!(
        formatted,
        "div {\n  max-height: none !important;\n}\n\n.important {\n  .foo() !important;\n}"
    );
    Ok(())
}
