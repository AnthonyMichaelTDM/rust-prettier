#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_block_statement_and_regexp_js_format_1_f4ecd9e8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a : { b : 1 }/2/;")?;
    assert_eq!(formatted, "a: {\n  b: 1;\n}\n/2/;");
    Ok(())
}
#[test]
fn test_comment_js_format_1_6697c7ea() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{\n  inf_leave: // goto emulation\n  for (;;) {}\n}\n{\n  inf_leave:\n  // goto emulation\n  for (; ;) { }\n}") ? ;
    assert_eq ! (formatted , "{\n  // goto emulation\n  inf_leave: for (;;) {}\n}\n{\n  // goto emulation\n  inf_leave: for (;;) {}\n}");
    Ok(())
}
#[test]
fn test_empty_label_js_format_1_ac9375a4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a:;\nb")?;
    assert_eq!(formatted, "a:;\nb;");
    Ok(())
}
