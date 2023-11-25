#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_missing_comments_vue_format_1_cc4b3068() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "<template lang=\"missing-comments\">\n      This should not be replaced.</template>",
    )?;
    assert_eq!(
        formatted,
        "<template lang=\"missing-comments\">\n      This should not be replaced.</template>"
    );
    Ok(())
}
