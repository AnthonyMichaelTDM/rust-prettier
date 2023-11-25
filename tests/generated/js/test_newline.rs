#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_backslash_2028_js_format_1_f328a571() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1;/*a*///b\u{2028}/*c*/2")?;
    assert_eq!(formatted, "1; /*a*/ //b\n/*c*/ 2;");
    Ok(())
}
#[test]
fn test_backslash_2029_js_format_1_a3a3bbba() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1;/*a*///b\u{2029}/*c*/2")?;
    assert_eq!(formatted, "1; /*a*/ //b\n/*c*/ 2;");
    Ok(())
}
