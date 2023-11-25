#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_enum_unknown_members_js_trailing_commaall_format_1_c48af451() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @flow\n\nenum E {\n  A,\n  B,\n  ...\n}")?;
    assert_eq!(formatted, "// @flow\n\nenum E {\n  A,\n  B,\n  ...\n}");
    Ok(())
}
#[test]
fn test_enum_unknown_members_empty_js_trailing_commaall_format_1_d271186d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @flow\n\nenum E {\n  ...\n}")?;
    assert_eq!(formatted, "// @flow\n\nenum E {\n  ...\n}");
    Ok(())
}
