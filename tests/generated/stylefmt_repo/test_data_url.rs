#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_data_url_css_format_1_e5235d7b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("div {\n  background-image: url(data:image/png;base64,iVBORw0KGg);\n}")?;
    assert_eq!(
        formatted,
        "div {\n  background-image: url(data:image/png;base64,iVBORw0KGg);\n}"
    );
    Ok(())
}
