#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_fill_css_format_1_2d1bbbf1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("div {\n  border-left: 1px solid mix($warningBackgroundColors, $warningBorderColors, 50%);\n  $fontFamily: \"Lato\", -apple-system, \"Helvetica Neue\", Helvetica, Arial, sans-serif;\n}") ? ;
    assert_eq ! (formatted , "div {\n  border-left: 1px solid\n    mix($warningBackgroundColors, $warningBorderColors, 50%);\n  $fontFamily:\n    \"Lato\",\n    -apple-system,\n    \"Helvetica Neue\",\n    Helvetica,\n    Arial,\n    sans-serif;\n}");
    Ok(())
}
