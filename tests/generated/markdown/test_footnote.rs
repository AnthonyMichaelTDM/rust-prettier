#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_footnote_md_prose_wrapalways_format_1_cfbc39d3() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("^[inline]")?;
    assert_eq!(formatted, "^[inline]");
    Ok(())
}
#[test]
fn test_simple_md_prose_wrapalways_format_1_2600a577() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[^alpha bravo]")?;
    assert_eq!(formatted, "[^alpha bravo]");
    Ok(())
}
