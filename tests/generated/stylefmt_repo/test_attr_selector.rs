#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_attr_selector_css_format_1_c60bf547() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "[  data-sizes~=\"m:0\" ]    {\n  color:pink;\n}\n\n[ class*=\"test\" ]{\n  color: silver}",
    )?;
    assert_eq!(
        formatted,
        "[data-sizes~=\"m:0\"] {\n  color: pink;\n}\n\n[class*=\"test\"] {\n  color: silver;\n}"
    );
    Ok(())
}
