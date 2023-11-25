#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_ca1b9e56() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nvar m1 = require('1DoesntExist');\nimport {numVal as numVal1} from '1DoesntExist';\nvar a_1: number = m1.numVal;\nvar a_2: number = numVal1;\n\n// Error: 'Exists2' is not a valid module name\n//\n// This tests that, for haste, the first name_mapper regexp that happens to\n// match the given module name string is picked.\nvar m2 = require('2DoesntExist'); // Error\nimport {numVal as numVal2} from '3DoesntExist'; // Error") ? ;
    assert_eq ! (formatted , "/* @flow */\n\nvar m1 = require(\"1DoesntExist\");\nimport { numVal as numVal1 } from \"1DoesntExist\";\nvar a_1: number = m1.numVal;\nvar a_2: number = numVal1;\n\n// Error: 'Exists2' is not a valid module name\n//\n// This tests that, for haste, the first name_mapper regexp that happens to\n// match the given module name string is picked.\nvar m2 = require(\"2DoesntExist\"); // Error\nimport { numVal as numVal2 } from \"3DoesntExist\"; // Error");
    Ok(())
}
#[test]
fn test_exists_js_format_1_e1dc3f39() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "/**\n * @providesModule Exists\n * @flow\n */\n\nmodule.exports = {\n  numVal: 42\n};",
    )?;
    assert_eq!(
        formatted,
        "/**\n * @providesModule Exists\n * @flow\n */\n\nmodule.exports = {\n  numVal: 42,\n};"
    );
    Ok(())
}
