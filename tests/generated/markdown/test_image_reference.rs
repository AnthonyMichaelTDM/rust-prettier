#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_collapsed_md_prose_wrapalways_format_1_ab09a7d1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("![hello][]")?;
    assert_eq!(formatted, "![hello][]");
    Ok(())
}
#[test]
fn test_full_md_prose_wrapalways_format_1_3307c579() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("![hello][world]")?;
    assert_eq!(formatted, "![hello][world]");
    Ok(())
}
#[test]
fn test_full_no_alt_md_prose_wrapalways_format_1_70297797() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("![][world]")?;
    assert_eq!(formatted, "![][world]");
    Ok(())
}
#[test]
fn test_shortcut_md_prose_wrapalways_format_1_5d19aeb1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("![hello]")?;
    assert_eq!(formatted, "![hello]");
    Ok(())
}
