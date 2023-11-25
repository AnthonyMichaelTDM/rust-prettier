#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_colon_css_format_1_1421ac58() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("div {\n  filter: progid:DXImageTransform.Microsoft.BasicImage(rotation=3);\n}")?;
    assert_eq!(
        formatted,
        "div {\n  filter: progid:DXImageTransform.Microsoft.BasicImage(rotation=3);\n}"
    );
    Ok(())
}
