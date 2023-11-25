#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_in_jsx_text_js_format_1_0759ce22() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(3)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<>a<|>\n  <div>hi</div>\n</>")?;
    assert_eq!(formatted, "<>\n  a<|><div>hi</div>\n</>;");
    Ok(())
}
