#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_with_parameters_yml_format_1_61cf25df() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("%YAML 1.2\n---")?;
    assert_eq!(formatted, "%YAML 1.2\n---");
    Ok(())
}
#[test]
fn test_without_parameters_yml_format_1_85c72d90() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("%SOMETHING\n---")?;
    assert_eq!(formatted, "%SOMETHING\n---");
    Ok(())
}
