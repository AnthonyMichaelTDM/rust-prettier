#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_ec1d2728() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @flow")?;
    assert_eq!(formatted, "// @flow");
    Ok(())
}
#[test]
fn test_b_js_format_1_23da6dac() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("/**\n * @flow\n * @providesModule b\n */\nrequire('./a');")?;
    assert_eq!(
        formatted,
        "/**\n * @flow\n * @providesModule b\n */\nrequire(\"./a\");"
    );
    Ok(())
}
#[test]
fn test_c_js_format_1_a97759d3() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @flow\nrequire('./a.js');\nrequire('b');")?;
    assert_eq!(formatted, "// @flow\nrequire(\"./a.js\");\nrequire(\"b\");");
    Ok(())
}
