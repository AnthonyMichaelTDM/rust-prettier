use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_53b4040f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("/* @flow */\n\nmodule.exports.fun = (): string => 'hello there!';");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/* @flow */\n\nmodule.exports.fun = (): string => \"hello there!\";"
    );
}
#[test]
fn test_cjs_js_format_1_1c211780() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @flow\nmodule.exports = 42;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "// @flow\nmodule.exports = 42;");
}
#[test]
fn test_test_absolute_js_format_1_47f6ea7e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\n// This will require ./node_modules/B.js.flow\nvar B1 = require('B');\n(B1.fun(): string); // Error number ~> string\n\n// This will require ./node_modules/B.js.flow\nvar B2 = require('B.js');\n(B2.fun(): string); // Error number ~> string\n\nvar C = require('package_with_full_main');\n(C.fun(): string); // Error number ~> string\n\nvar D = require('package_with_partial_main');\n(D.fun(): string); // Error number ~> string\n\nvar E = require('package_with_no_package_json');\n(E.fun(): string); // Error number ~> string\n\nvar F = require('package_with_dir_main');\n(F.fun(): string); // Error number ~> string") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\n// This will require ./node_modules/B.js.flow\nvar B1 = require(\"B\");\n(B1.fun(): string); // Error number ~> string\n\n// This will require ./node_modules/B.js.flow\nvar B2 = require(\"B.js\");\n(B2.fun(): string); // Error number ~> string\n\nvar C = require(\"package_with_full_main\");\n(C.fun(): string); // Error number ~> string\n\nvar D = require(\"package_with_partial_main\");\n(D.fun(): string); // Error number ~> string\n\nvar E = require(\"package_with_no_package_json\");\n(E.fun(): string); // Error number ~> string\n\nvar F = require(\"package_with_dir_main\");\n(F.fun(): string); // Error number ~> string");
}
#[test]
fn test_test_relative_js_format_1_c1754cb7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\n// This will require ./A.js.flow\nvar A1 = require('./A');\n(A1.fun(): string); // Error number ~> string\n\n// This will require ./A.js.flow\nvar A2 = require('./A.js');\n(A2.fun(): string); // Error number ~> string\n\nvar CJS = require('./CJS.js');\n(CJS: string);\n(CJS: number); // Error: string ~> number") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\n// This will require ./A.js.flow\nvar A1 = require(\"./A\");\n(A1.fun(): string); // Error number ~> string\n\n// This will require ./A.js.flow\nvar A2 = require(\"./A.js\");\n(A2.fun(): string); // Error number ~> string\n\nvar CJS = require(\"./CJS.js\");\n(CJS: string);\n(CJS: number); // Error: string ~> number");
}
