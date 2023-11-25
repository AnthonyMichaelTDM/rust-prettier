#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_bar_js_format_1_f66ff083() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("var Bar = { x: 0 };\nmodule.exports = Bar;")?;
    assert_eq!(formatted, "var Bar = { x: 0 };\nmodule.exports = Bar;");
    Ok(())
}
#[test]
fn test_copy_properties_js_format_1_6d32cdd3() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nlet tests = [\n  // global\n  function() {\n    (copyProperties()); // error, unknown global\n  },\n\n  // annotation\n  function(copyProperties: Object$Assign) {\n    let result = {};\n    result.baz = false;\n    (copyProperties(\n      result,\n      { foo: 'a' },\n      { bar: 123 }\n    ): { foo: string, bar: number, baz: boolean });\n  },\n\n  // module from lib\n  function() {\n    const copyProperties = require('copyProperties');\n    let x = { foo: 'a' };\n    let y = { bar: 123 };\n    (copyProperties({}, x, y): { foo: string, bar: number });\n  },\n\n  // too few args\n  function(copyProperties: Object$Assign) {\n    copyProperties();\n    (copyProperties({ foo: 'a' }): { foo: number }); // err, num !~> string\n  },\n\n  // passed as a function\n  function(copyProperties: Object$Assign) {\n    function x(cb: Function) {}\n    x(copyProperties);\n  }\n];") ? ;
    assert_eq ! (formatted , "// @flow\n\nlet tests = [\n  // global\n  function () {\n    copyProperties(); // error, unknown global\n  },\n\n  // annotation\n  function (copyProperties: Object$Assign) {\n    let result = {};\n    result.baz = false;\n    (copyProperties(result, { foo: \"a\" }, { bar: 123 }): {\n      foo: string,\n      bar: number,\n      baz: boolean,\n    });\n  },\n\n  // module from lib\n  function () {\n    const copyProperties = require(\"copyProperties\");\n    let x = { foo: \"a\" };\n    let y = { bar: 123 };\n    (copyProperties({}, x, y): { foo: string, bar: number });\n  },\n\n  // too few args\n  function (copyProperties: Object$Assign) {\n    copyProperties();\n    (copyProperties({ foo: \"a\" }): { foo: number }); // err, num !~> string\n  },\n\n  // passed as a function\n  function (copyProperties: Object$Assign) {\n    function x(cb: Function) {}\n    x(copyProperties);\n  },\n];");
    Ok(())
}
#[test]
fn test_invariant_js_format_1_e29d8637() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nlet tests = [\n  function() {\n    let x: ?string = null;\n    invariant(x, 'truthy only'); // error, forgot to require invariant\n  },\n\n  function(invariant: Function) {\n    let x: ?string = null;\n    invariant(x);\n    (x: string);\n  }\n]") ? ;
    assert_eq ! (formatted , "/* @flow */\n\nlet tests = [\n  function () {\n    let x: ?string = null;\n    invariant(x, \"truthy only\"); // error, forgot to require invariant\n  },\n\n  function (invariant: Function) {\n    let x: ?string = null;\n    invariant(x);\n    (x: string);\n  },\n];");
    Ok(())
}
#[test]
fn test_lib_js_format_1_71c7ff7b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare module \"copyProperties\" {\n  declare var exports: Object$Assign;\n}\n\ndeclare module \"mergeInto\" {\n  declare var exports: $Facebookism$MergeInto;\n}\n\ndeclare module \"mixin\" {\n  declare var exports: $Facebookism$Mixin;\n}") ? ;
    assert_eq ! (formatted , "declare module \"copyProperties\" {\n  declare var exports: Object$Assign;\n}\n\ndeclare module \"mergeInto\" {\n  declare var exports: $Facebookism$MergeInto;\n}\n\ndeclare module \"mixin\" {\n  declare var exports: $Facebookism$Mixin;\n}");
    Ok(())
}
#[test]
fn test_merge_into_js_format_1_356cc7eb() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nlet tests = [\n  // global\n  function() {\n    (mergeInto()); // error, unknown global\n  },\n\n  // annotation\n  function(mergeInto: $Facebookism$MergeInto) {\n    let result = {};\n    result.baz = false;\n    (mergeInto(result, { foo: 'a' }, { bar: 123 }): void);\n    (result: { foo: string, bar: number, baz: boolean });\n  },\n\n  // module from lib\n  function() {\n    const mergeInto = require('mergeInto');\n    let result: { foo?: string, bar?: number, baz: boolean } = { baz: false };\n    (mergeInto(result, { foo: 'a' }, { bar: 123 }): void);\n  },\n\n  // too few args\n  function(mergeInto: $Facebookism$MergeInto) {\n    mergeInto();\n  },\n\n  // passed as a function\n  function(mergeInto: $Facebookism$MergeInto) {\n    function x(cb: Function) {}\n    x(mergeInto);\n  }\n];") ? ;
    assert_eq ! (formatted , "// @flow\n\nlet tests = [\n  // global\n  function () {\n    mergeInto(); // error, unknown global\n  },\n\n  // annotation\n  function (mergeInto: $Facebookism$MergeInto) {\n    let result = {};\n    result.baz = false;\n    (mergeInto(result, { foo: \"a\" }, { bar: 123 }): void);\n    (result: { foo: string, bar: number, baz: boolean });\n  },\n\n  // module from lib\n  function () {\n    const mergeInto = require(\"mergeInto\");\n    let result: { foo?: string, bar?: number, baz: boolean } = { baz: false };\n    (mergeInto(result, { foo: \"a\" }, { bar: 123 }): void);\n  },\n\n  // too few args\n  function (mergeInto: $Facebookism$MergeInto) {\n    mergeInto();\n  },\n\n  // passed as a function\n  function (mergeInto: $Facebookism$MergeInto) {\n    function x(cb: Function) {}\n    x(mergeInto);\n  },\n];");
    Ok(())
}
#[test]
fn test_test_js_format_1_d34bafd3() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var Bar = require('./Bar');\nvar mixin = require('mixin');\n\nclass Foo extends mixin(Bar) {\n  m() {\n    var x: string = this.x;\n    this.y = \"\";\n  }\n}") ? ;
    assert_eq ! (formatted , "var Bar = require(\"./Bar\");\nvar mixin = require(\"mixin\");\n\nclass Foo extends mixin(Bar) {\n  m() {\n    var x: string = this.x;\n    this.y = \"\";\n  }\n}");
    Ok(())
}
