#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_asterisk_md_prose_wrapalways_format_1_ae78015e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*123*")?;
    assert_eq!(formatted, "_123_");
    Ok(())
}
#[test]
fn test_complex_md_prose_wrapalways_format_1_bb7a0796() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("**Do you want to request a *feature* or report a *bug*?**\n\n*bug*?")?;
    assert_eq!(
        formatted,
        "**Do you want to request a _feature_ or report a _bug_?**\n\n_bug_?"
    );
    Ok(())
}
#[test]
fn test_special_md_prose_wrapalways_format_1_b78ef1d8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("0*1*2\n\n!*1*2\n\n0*1*!\n\n!*1*!\n\n0*1*2\n\n！*1*2\n\n0*1*！\n\n！*1*！\n\n0_1_2\n\n!_1_2\n\n0_1_! <!-- remark-misparsing, should be formatted as `0_1\\_!` -->\n\n!_1_!\n\n0_1_2\n\n！_1_2\n\n0_1_！ <!-- remark-misparsing, should be formatted as `0_1\\_！` -->\n\n！_1_！") ? ;
    assert_eq ! (formatted , "0*1*2\n\n!*1*2\n\n0*1*!\n\n!_1_!\n\n0*1*2\n\n！*1*2\n\n0*1*！\n\n！_1_！\n\n0_1_2\n\n!\\_1_2\n\n0*1*! <!-- remark-misparsing, should be formatted as `0_1\\_!` -->\n\n!_1_!\n\n0_1_2\n\n！\\_1_2\n\n0*1*！ <!-- remark-misparsing, should be formatted as `0_1\\_！` -->\n\n！_1_！");
    Ok(())
}
#[test]
fn test_underscore_md_prose_wrapalways_format_1_16db6593() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_123_")?;
    assert_eq!(formatted, "_123_");
    Ok(())
}
