#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_custom_parser_md_format_1_8c835e39() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("---mycustomparser\n- hello:    world\n-         123\n---\n\n__123__\n**456**")?;
    assert_eq!(
        formatted,
        "---mycustomparser\n- hello:    world\n-         123\n---\n\n**123**\n**456**"
    );
    Ok(())
}
#[test]
fn test_empty_md_format_1_7bf837f2() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\n---\n\n__123__\n**456**")?;
    assert_eq!(formatted, "---\n---\n\n**123**\n**456**");
    Ok(())
}
#[test]
fn test_empty_2_md_format_1_bb46c55b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("---\n---\n\n# Title 1\n\nHello, world\n\n---\n\ntext")?;
    assert_eq!(
        formatted,
        "---\n---\n\n# Title 1\n\nHello, world\n\n---\n\ntext"
    );
    Ok(())
}
