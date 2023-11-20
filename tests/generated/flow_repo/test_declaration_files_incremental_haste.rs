#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_8bacac58() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @providesModule A */\nclass Implementation {}\nexport function foo(): Implementation { return new Implementation; }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @providesModule A */\nclass Implementation {}\nexport function foo(): Implementation {\n  return new Implementation();\n}");
}
#[test]
fn test_explicit_provides_module_different_name_js_format_1_dfc98316() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/*\n * @providesModule ExplicitProvidesModuleDifferentName\n * @flow\n */\n\nclass Implementation {}\nmodule.exports.fun = (): Implementation => new Implementation;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/*\n * @providesModule ExplicitProvidesModuleDifferentName\n * @flow\n */\n\nclass Implementation {}\nmodule.exports.fun = (): Implementation => new Implementation();");
}
#[test]
fn test_explicit_provides_module_same_name_js_format_1_997a3401() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/*\n * @providesModule ExplicitProvidesModuleSameName\n * @flow\n */\n\nclass Implementation {}\nmodule.exports.fun = (): Implementation => new Implementation;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/*\n * @providesModule ExplicitProvidesModuleSameName\n * @flow\n */\n\nclass Implementation {}\nmodule.exports.fun = (): Implementation => new Implementation();");
}
#[test]
fn test_implicit_provides_module_js_format_1_1cf78ccc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/*\n * @providesModule ImplicitProvidesModule\n * @flow\n */\n\nclass Implementation {}\nmodule.exports.fun = (): Implementation => new Implementation;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/*\n * @providesModule ImplicitProvidesModule\n * @flow\n */\n\nclass Implementation {}\nmodule.exports.fun = (): Implementation => new Implementation();");
}
#[test]
fn test_md_5_js_format_1_b084b9fa() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* @providesModule md5 */");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "/* @providesModule md5 */");
}
#[test]
fn test_test_js_format_1_678727c3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nvar Implicit = require('ImplicitProvidesModule');\n(Implicit.fun(): boolean); // Error: Either Implementation ~> boolean or Declaration ~> boolean\n\nvar ExplicitSameName = require('ExplicitProvidesModuleSameName');\n(ExplicitSameName.fun(): boolean); // Error: Either Implementation ~> boolean or Declaration ~> boolean\n\nvar ExplicitDifferentName = require('ExplicitProvidesModuleDifferentName');\n(ExplicitDifferentName.fun(): boolean); // Error: Either Implementation ~> boolean or Declaration ~> boolean") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nvar Implicit = require(\"ImplicitProvidesModule\");\n(Implicit.fun(): boolean); // Error: Either Implementation ~> boolean or Declaration ~> boolean\n\nvar ExplicitSameName = require(\"ExplicitProvidesModuleSameName\");\n(ExplicitSameName.fun(): boolean); // Error: Either Implementation ~> boolean or Declaration ~> boolean\n\nvar ExplicitDifferentName = require(\"ExplicitProvidesModuleDifferentName\");\n(ExplicitDifferentName.fun(): boolean); // Error: Either Implementation ~> boolean or Declaration ~> boolean");
}
