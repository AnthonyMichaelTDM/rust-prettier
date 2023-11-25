#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_interpolation_less_format_1_a26f1ab7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("less")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://github.com/shellscape/postcss-less/pull/159\n@{selector}-title{ @{prop}-size: @{color} }") ? ;
    assert_eq ! (formatted , "// https://github.com/shellscape/postcss-less/pull/159\n@{selector}-title {\n  @{prop}-size: @{color};\n}");
    Ok(())
}
#[test]
fn test_selector_less_format_1_327e8957() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("less")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("@prefix: foo;\n\n@{prefix}-title{\n  color: red;\n}")?;
    assert_eq!(
        formatted,
        "@prefix: foo;\n\n@{prefix}-title {\n  color: red;\n}"
    );
    Ok(())
}
