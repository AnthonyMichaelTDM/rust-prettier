#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_ebb7c7f8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nclass AImplementation {}\nexport function foo(): AImplementation { return new AImplementation(); }") ? ;
    assert_eq ! (formatted , "/* @flow */\n\nclass AImplementation {}\nexport function foo(): AImplementation {\n  return new AImplementation();\n}");
    Ok(())
}
#[test]
fn test_index_js_format_1_bd9a57c5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nvar A = require('A');\n(A.foo(): boolean); // Error: Either AImplementation ~> boolean or ADefinition ~> boolean\n\nvar test = require('test');\n(test.foo(): boolean); // Error: Either TestImplementation ~> boolean or TestDefinition ~> boolean") ? ;
    assert_eq ! (formatted , "/* @flow */\n\nvar A = require(\"A\");\n(A.foo(): boolean); // Error: Either AImplementation ~> boolean or ADefinition ~> boolean\n\nvar test = require(\"test\");\n(test.foo(): boolean); // Error: Either TestImplementation ~> boolean or TestDefinition ~> boolean");
    Ok(())
}
