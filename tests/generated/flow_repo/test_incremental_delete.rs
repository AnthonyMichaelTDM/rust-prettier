#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_ccdd8e05() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @flow\nvar a: string = 0;\nmodule.exports = a;")?;
    assert_eq!(
        formatted,
        "// @flow\nvar a: string = 0;\nmodule.exports = a;"
    );
    Ok(())
}
#[test]
fn test_b_js_format_1_1837df07() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("// @flow\nvar a = require('./a');\nvar b: number = a;\nmodule.exports = b;")?;
    assert_eq!(
        formatted,
        "// @flow\nvar a = require(\"./a\");\nvar b: number = a;\nmodule.exports = b;"
    );
    Ok(())
}
#[test]
fn test_c_js_format_1_fbec86c4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("// @flow\nvar b = require('./b');\nvar c: string = b;\nmodule.exports = c;")?;
    assert_eq!(
        formatted,
        "// @flow\nvar b = require(\"./b\");\nvar c: string = b;\nmodule.exports = c;"
    );
    Ok(())
}
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
#[test]
fn test_requires_unchecked_js_format_1_33a8097c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * depends on an unchecked module, which will be deleted\n * @flow\n */\nvar unchecked = require('Unchecked');") ? ;
    assert_eq ! (formatted , "/**\n * depends on an unchecked module, which will be deleted\n * @flow\n */\nvar unchecked = require(\"Unchecked\");");
    Ok(())
}
#[test]
fn test_unchecked_js_format_1_b8360940() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * Not a flow module.\n * @providesModule Unchecked\n */\nmodule.exports = \"unchecked\";") ? ;
    assert_eq ! (formatted , "/**\n * Not a flow module.\n * @providesModule Unchecked\n */\nmodule.exports = \"unchecked\";");
    Ok(())
}
