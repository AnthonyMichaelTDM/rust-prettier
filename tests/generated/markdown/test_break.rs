#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_simple_md_prose_wrapalways_format_1_95d7e4b0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("123  \n456\n\n123\\\\\n456\n\n- 123  \n  123")?;
    assert_eq!(formatted, "123  \n456\n\n123\\\\\n456\n\n- 123  \n  123");
    Ok(())
}
