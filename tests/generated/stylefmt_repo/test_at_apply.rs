#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_at_apply_css_format_1_3a22f0de() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(".toolbar{@apply   --toolbar-theme   ;\n}")?;
    assert_eq!(formatted, ".toolbar {\n  @apply --toolbar-theme;\n}");
    Ok(())
}
