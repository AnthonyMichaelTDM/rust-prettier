#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_cccfe517() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @flow\n\n(require('./b'): number);")?;
    assert_eq!(formatted, "// @flow\n\n(require(\"./b\"): number);");
    Ok(())
}
#[test]
fn test_b_js_format_1_2cf06a0a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @flow\n\nmodule.exports = \"\";")?;
    assert_eq!(formatted, "// @flow\n\nmodule.exports = \"\";");
    Ok(())
}
#[test]
fn test_test_js_format_1_a855ea71() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @flow\n\n(\"\": number);")?;
    assert_eq!(formatted, "// @flow\n\n(\"\": number);");
    Ok(())
}
