#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_invalid_html_format_1_a113b104() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "---\n    invalid:\ninvalid:\n---\n\n\n\n<html><head></head><body></body></html>",
    )?;
    assert_eq!(
        formatted,
        "---\n    invalid:\ninvalid:\n---\n\n<html>\n  <head></head>\n  <body></body>\n</html>"
    );
    Ok(())
}
#[test]
fn test_yaml_html_format_1_9f228366() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "---\nhello:     world\n---\n\n\n\n\n\n\n<html><head></head><body></body></html>",
    )?;
    assert_eq!(
        formatted,
        "---\nhello: world\n---\n\n<html>\n  <head></head>\n  <body></body>\n</html>"
    );
    Ok(())
}
