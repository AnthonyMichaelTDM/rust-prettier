#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_dupe_1_js_format_1_52636103() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * Dupe provider 1/2\n * @providesModule Dupe\n * @flow\n */\nmodule.exports = \"dupe1\";") ? ;
    assert_eq ! (formatted , "/**\n * Dupe provider 1/2\n * @providesModule Dupe\n * @flow\n */\nmodule.exports = \"dupe1\";");
    Ok(())
}
#[test]
fn test_dupe_2_js_format_1_6eef903e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * Dupe provider 2/2\n * @providesModule Dupe\n * @flow\n */\nmodule.exports = \"dupe2\";") ? ;
    assert_eq ! (formatted , "/**\n * Dupe provider 2/2\n * @providesModule Dupe\n * @flow\n */\nmodule.exports = \"dupe2\";");
    Ok(())
}
#[test]
fn test_requires_dupe_js_format_1_58af2be4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "/**\n * depends on doubly-provided module\n * @flow\n */\nvar dupe = require('Dupe');",
    )?;
    assert_eq!(
        formatted,
        "/**\n * depends on doubly-provided module\n * @flow\n */\nvar dupe = require(\"Dupe\");"
    );
    Ok(())
}
