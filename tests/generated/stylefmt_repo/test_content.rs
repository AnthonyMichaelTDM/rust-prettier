#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_content_css_format_1_1d0e32e0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("  div {content: \"  test 1/2  \";   content:'  test   1+2';\n}")?;
    assert_eq!(
        formatted,
        "div {\n  content: \"  test 1/2  \";\n  content: \"  test   1+2\";\n}"
    );
    Ok(())
}
