#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_bom_css_format_1_52a7c750() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("\u{feff}\n\n/* Block comment */\n\nhtml {\n  content: \"#{1}\";\n\n}")?;
    assert_eq!(
        formatted,
        "\u{feff}/* Block comment */\n\nhtml {\n  content: \"#{1}\";\n}"
    );
    Ok(())
}
