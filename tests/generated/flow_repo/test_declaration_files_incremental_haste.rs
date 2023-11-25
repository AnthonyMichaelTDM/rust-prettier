#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_8bacac58() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @providesModule A */\nclass Implementation {}\nexport function foo(): Implementation { return new Implementation; }") ? ;
    assert_eq ! (formatted , "/* @providesModule A */\nclass Implementation {}\nexport function foo(): Implementation {\n  return new Implementation();\n}");
    Ok(())
}
#[test]
fn test_explicit_provides_module_different_name_js_format_1_dfc98316() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/*\n * @providesModule ExplicitProvidesModuleDifferentName\n * @flow\n */\n\nclass Implementation {}\nmodule.exports.fun = (): Implementation => new Implementation;") ? ;
    assert_eq ! (formatted , "/*\n * @providesModule ExplicitProvidesModuleDifferentName\n * @flow\n */\n\nclass Implementation {}\nmodule.exports.fun = (): Implementation => new Implementation();");
    Ok(())
}
#[test]
fn test_explicit_provides_module_same_name_js_format_1_997a3401() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/*\n * @providesModule ExplicitProvidesModuleSameName\n * @flow\n */\n\nclass Implementation {}\nmodule.exports.fun = (): Implementation => new Implementation;") ? ;
    assert_eq ! (formatted , "/*\n * @providesModule ExplicitProvidesModuleSameName\n * @flow\n */\n\nclass Implementation {}\nmodule.exports.fun = (): Implementation => new Implementation();");
    Ok(())
}
#[test]
fn test_implicit_provides_module_js_format_1_1cf78ccc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/*\n * @providesModule ImplicitProvidesModule\n * @flow\n */\n\nclass Implementation {}\nmodule.exports.fun = (): Implementation => new Implementation;") ? ;
    assert_eq ! (formatted , "/*\n * @providesModule ImplicitProvidesModule\n * @flow\n */\n\nclass Implementation {}\nmodule.exports.fun = (): Implementation => new Implementation();");
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
fn test_test_js_format_1_678727c3() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nvar Implicit = require('ImplicitProvidesModule');\n(Implicit.fun(): boolean); // Error: Either Implementation ~> boolean or Declaration ~> boolean\n\nvar ExplicitSameName = require('ExplicitProvidesModuleSameName');\n(ExplicitSameName.fun(): boolean); // Error: Either Implementation ~> boolean or Declaration ~> boolean\n\nvar ExplicitDifferentName = require('ExplicitProvidesModuleDifferentName');\n(ExplicitDifferentName.fun(): boolean); // Error: Either Implementation ~> boolean or Declaration ~> boolean") ? ;
    assert_eq ! (formatted , "/* @flow */\n\nvar Implicit = require(\"ImplicitProvidesModule\");\n(Implicit.fun(): boolean); // Error: Either Implementation ~> boolean or Declaration ~> boolean\n\nvar ExplicitSameName = require(\"ExplicitProvidesModuleSameName\");\n(ExplicitSameName.fun(): boolean); // Error: Either Implementation ~> boolean or Declaration ~> boolean\n\nvar ExplicitDifferentName = require(\"ExplicitProvidesModuleDifferentName\");\n(ExplicitDifferentName.fun(): boolean); // Error: Either Implementation ~> boolean or Declaration ~> boolean");
    Ok(())
}
