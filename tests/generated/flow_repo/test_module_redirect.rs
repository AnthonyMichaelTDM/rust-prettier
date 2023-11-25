#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_e6cfd29e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("declare module A {\n  declare function foo(): string;\n}")?;
    assert_eq!(
        formatted,
        "declare module A {\n  declare function foo(): string;\n}"
    );
    Ok(())
}
#[test]
fn test_b_js_format_1_96d79093() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("/**\n * @providesModule B\n * @flow\n */\n\nmodule.exports = require('A');")?;
    assert_eq!(
        formatted,
        "/**\n * @providesModule B\n * @flow\n */\n\nmodule.exports = require(\"A\");"
    );
    Ok(())
}
#[test]
fn test_c_js_format_1_ec36d55d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("/**\n * @providesModule C\n * @flow\n */\n\nmodule.exports = require('B');")?;
    assert_eq!(
        formatted,
        "/**\n * @providesModule C\n * @flow\n */\n\nmodule.exports = require(\"B\");"
    );
    Ok(())
}
#[test]
fn test_d_js_format_1_a6be0995() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule D\n * @flow\n */\n\nvar bar1: string = require('A');\nvar bar2: string = require('B');\nvar bar3: string = require('C');") ? ;
    assert_eq ! (formatted , "/**\n * @providesModule D\n * @flow\n */\n\nvar bar1: string = require(\"A\");\nvar bar2: string = require(\"B\");\nvar bar3: string = require(\"C\");");
    Ok(())
}
