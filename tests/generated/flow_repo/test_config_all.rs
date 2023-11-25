#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_no_at_flow_js_format_1_93d2adc9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("var x: number = \"not a number\"; // Error: string ~> number")?;
    assert_eq!(
        formatted,
        "var x: number = \"not a number\"; // Error: string ~> number"
    );
    Ok(())
}
