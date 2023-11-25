#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_09afbf79() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nvar m1 = require('./1DoesntExist');\nvar a_1: number = m1.numVal;\nvar a_2: string = m1.numVal; // Error: number ~> string\nimport {numVal} from './1DoesntExist';\nvar a_3: number = numVal;\nvar a_4: string = numVal; // Error: number ~> string\n\n// This tests that, for node, the first name mapping that both matches *and*\n// results in a valid module filename is picked.\nvar m2 = require('./2DoesntExist');\nvar b_1: number = m2.numVal;\nvar b_2: string = m2.numVal; // Error: number ~> string\nimport {numVal as numVal2} from './3DoesntExist';\nvar b_3: number = numVal2;\nvar b_4: string = numVal2; // Error: number ~> string\n\n// node_modules/Exists/index.js\nvar m3 = require('4DoesntExist');\nvar c_1: number = m3.numVal;\nvar c_2: string = m3.numVal; // Error: number ~> string\nimport {numVal as numVal3} from '5DoesntExist';\nvar c_3: number = numVal3;\nvar c_4: string = numVal3; // Error: number ~> string") ? ;
    assert_eq ! (formatted , "/* @flow */\n\nvar m1 = require(\"./1DoesntExist\");\nvar a_1: number = m1.numVal;\nvar a_2: string = m1.numVal; // Error: number ~> string\nimport { numVal } from \"./1DoesntExist\";\nvar a_3: number = numVal;\nvar a_4: string = numVal; // Error: number ~> string\n\n// This tests that, for node, the first name mapping that both matches *and*\n// results in a valid module filename is picked.\nvar m2 = require(\"./2DoesntExist\");\nvar b_1: number = m2.numVal;\nvar b_2: string = m2.numVal; // Error: number ~> string\nimport { numVal as numVal2 } from \"./3DoesntExist\";\nvar b_3: number = numVal2;\nvar b_4: string = numVal2; // Error: number ~> string\n\n// node_modules/Exists/index.js\nvar m3 = require(\"4DoesntExist\");\nvar c_1: number = m3.numVal;\nvar c_2: string = m3.numVal; // Error: number ~> string\nimport { numVal as numVal3 } from \"5DoesntExist\";\nvar c_3: number = numVal3;\nvar c_4: string = numVal3; // Error: number ~> string");
    Ok(())
}
#[test]
fn test_exists_js_format_1_525450a8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* @flow */\n\nmodule.exports = {\n  numVal: 42\n};")?;
    assert_eq!(
        formatted,
        "/* @flow */\n\nmodule.exports = {\n  numVal: 42,\n};"
    );
    Ok(())
}
