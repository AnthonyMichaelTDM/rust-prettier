#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_explicit_provides_module_different_name_js_format_1_a843989f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/*\n * @providesModule ExplicitProvidesModuleDifferentName\n * @flow\n */\n\nmodule.exports.fun = (): string => \"hello there\";") ? ;
    assert_eq ! (formatted , "/*\n * @providesModule ExplicitProvidesModuleDifferentName\n * @flow\n */\n\nmodule.exports.fun = (): string => \"hello there\";");
    Ok(())
}
#[test]
fn test_explicit_provides_module_same_name_js_format_1_30b08278() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/*\n * @providesModule ExplicitProvidesModuleSameName\n * @flow\n */\n\nmodule.exports.fun = (): string => \"hello there\";") ? ;
    assert_eq ! (formatted , "/*\n * @providesModule ExplicitProvidesModuleSameName\n * @flow\n */\n\nmodule.exports.fun = (): string => \"hello there\";");
    Ok(())
}
#[test]
fn test_implicit_provides_module_js_format_1_b3376373() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/*\n * @providesModule ImplicitProvidesModule\n * @flow\n */\n\nmodule.exports.fun = (): string => \"hello there\";") ? ;
    assert_eq ! (formatted , "/*\n * @providesModule ImplicitProvidesModule\n * @flow\n */\n\nmodule.exports.fun = (): string => \"hello there\";");
    Ok(())
}
#[test]
fn test_md_5_js_format_1_b084b9fa() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* @providesModule md5 */")?;
    assert_eq!(formatted, "/* @providesModule md5 */");
    Ok(())
}
#[test]
fn test_test_js_format_1_97eaf7ab() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nvar Implicit = require('ImplicitProvidesModule');\n(Implicit.fun(): string);\n\nvar ExplicitSameName = require('ExplicitProvidesModuleSameName');\n(ExplicitSameName.fun(): string);\n\nvar ExplicitDifferentName = require('ExplicitProvidesModuleDifferentName');\n(ExplicitDifferentName.fun(): string);") ? ;
    assert_eq ! (formatted , "/* @flow */\n\nvar Implicit = require(\"ImplicitProvidesModule\");\n(Implicit.fun(): string);\n\nvar ExplicitSameName = require(\"ExplicitProvidesModuleSameName\");\n(ExplicitSameName.fun(): string);\n\nvar ExplicitDifferentName = require(\"ExplicitProvidesModuleDifferentName\");\n(ExplicitDifferentName.fun(): string);");
    Ok(())
}
