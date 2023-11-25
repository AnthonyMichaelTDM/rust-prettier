#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_10f90113() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* @providesModule IncrModuleA */")?;
    assert_eq!(formatted, "/* @providesModule IncrModuleA */");
    Ok(())
}
#[test]
fn test_b_js_format_1_d111e161() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "/* @providesModule IncrModuleB\n   @flow\n*/\n\nvar A = require('IncrModuleA');",
    )?;
    assert_eq!(
        formatted,
        "/* @providesModule IncrModuleB\n   @flow\n*/\n\nvar A = require(\"IncrModuleA\");"
    );
    Ok(())
}
#[test]
fn test_dup_a_js_format_1_a31e1841() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("/* @providesModule IncrModuleA */\n\nvar x:string = 0;")?;
    assert_eq!(
        formatted,
        "/* @providesModule IncrModuleA */\n\nvar x: string = 0;"
    );
    Ok(())
}
