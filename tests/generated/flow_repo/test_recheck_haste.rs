#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_1_js_format_1_1ea03ff3() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/**\n * @providesModule A\n * @flow\n */")?;
    assert_eq!(formatted, "/**\n * @providesModule A\n * @flow\n */");
    Ok(())
}
#[test]
fn test_a_3_js_format_1_93d06262() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @flow\n\nrequire('A');")?;
    assert_eq!(formatted, "// @flow\n\nrequire(\"A\");");
    Ok(())
}
#[test]
fn test_b_1_js_format_1_4db10796() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("/**\n * @flow\n */\n\nrequire('B2');\nrequire('B3');")?;
    assert_eq!(
        formatted,
        "/**\n * @flow\n */\n\nrequire(\"B2\");\nrequire(\"B3\");"
    );
    Ok(())
}
#[test]
fn test_b_3_js_format_1_057908e8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("/**\n * @providesModule B3\n * @flow\n */\n\nrequire('B2');")?;
    assert_eq!(
        formatted,
        "/**\n * @providesModule B3\n * @flow\n */\n\nrequire(\"B2\");"
    );
    Ok(())
}
