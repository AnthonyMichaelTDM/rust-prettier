#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_issue_2267_css_format_1_b0812228() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .range_end(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> 1 | .x{} \n    | ^^^^\n  2 ")?;
    assert_eq!(formatted, ".x{} ");
    Ok(())
}
