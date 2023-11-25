#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_6000d079() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class Implementation {}\nexport function foo(): Implementation { return new Implementation; }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class Implementation {}\nexport function foo(): Implementation {\n  return new Implementation();\n}");
}
#[test]
fn test_test_absolute_js_format_1_1ecf8d20() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\n// This will require ./node_modules/B.js.flow\nvar B1 = require('B');\n(B1.fun(): boolean); // Error either Implementation ~> boolean or Declaration ~> boolean\n\n// This will require ./node_modules/B.js.flow\nvar B2 = require('B.js');\n(B2.fun(): boolean); // Error either Implementation ~> boolean or Declaration ~> boolean\n\nvar C = require('package_with_full_main');\n(C.fun(): boolean); // Error either Implementation ~> boolean or Declaration ~> boolean\n\nvar D = require('package_with_partial_main');\n(D.fun(): boolean); // Error either Implementation ~> boolean or Declaration ~> boolean\n\nvar E = require('package_with_no_package_json');\n(E.fun(): boolean); // Error either Implementation ~> boolean or Declaration ~> boolean\n\nvar F = require('package_with_dir_main');\n(F.fun(): boolean); // Error either Implementation ~> boolean or Declaration ~> boolean\n\n// This will require ./node_modules/B.js.flow\nvar B1 = require('B');\n(B1.fun(): boolean); // Error either Implementation ~> boolean or Declaration ~> boolean\n\n// This will require ./node_modules/B.js.flow\nvar B2 = require('B.js');\n(B2.fun(): boolean); // Error either Implementation ~> boolean or Declaration ~> boolean\n\nvar C = require('package_with_full_main');\n(C.fun(): boolean); // Error either Implementation ~> boolean or Declaration ~> boolean\n\nvar D = require('package_with_partial_main');\n(D.fun(): boolean); // Error either Implementation ~> boolean or Declaration ~> boolean\n\nvar E = require('package_with_no_package_json');\n(E.fun(): boolean); // Error either Implementation ~> boolean or Declaration ~> boolean\n\nvar F = require('package_with_dir_main');\n(F.fun(): boolean); // Error either Implementation ~> boolean or Declaration ~> boolean") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\n// This will require ./node_modules/B.js.flow\nvar B1 = require(\"B\");\n(B1.fun(): boolean); // Error either Implementation ~> boolean or Declaration ~> boolean\n\n// This will require ./node_modules/B.js.flow\nvar B2 = require(\"B.js\");\n(B2.fun(): boolean); // Error either Implementation ~> boolean or Declaration ~> boolean\n\nvar C = require(\"package_with_full_main\");\n(C.fun(): boolean); // Error either Implementation ~> boolean or Declaration ~> boolean\n\nvar D = require(\"package_with_partial_main\");\n(D.fun(): boolean); // Error either Implementation ~> boolean or Declaration ~> boolean\n\nvar E = require(\"package_with_no_package_json\");\n(E.fun(): boolean); // Error either Implementation ~> boolean or Declaration ~> boolean\n\nvar F = require(\"package_with_dir_main\");\n(F.fun(): boolean); // Error either Implementation ~> boolean or Declaration ~> boolean\n\n// This will require ./node_modules/B.js.flow\nvar B1 = require(\"B\");\n(B1.fun(): boolean); // Error either Implementation ~> boolean or Declaration ~> boolean\n\n// This will require ./node_modules/B.js.flow\nvar B2 = require(\"B.js\");\n(B2.fun(): boolean); // Error either Implementation ~> boolean or Declaration ~> boolean\n\nvar C = require(\"package_with_full_main\");\n(C.fun(): boolean); // Error either Implementation ~> boolean or Declaration ~> boolean\n\nvar D = require(\"package_with_partial_main\");\n(D.fun(): boolean); // Error either Implementation ~> boolean or Declaration ~> boolean\n\nvar E = require(\"package_with_no_package_json\");\n(E.fun(): boolean); // Error either Implementation ~> boolean or Declaration ~> boolean\n\nvar F = require(\"package_with_dir_main\");\n(F.fun(): boolean); // Error either Implementation ~> boolean or Declaration ~> boolean");
}
#[test]
fn test_test_relative_js_format_1_73a2260b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import { foo } from './A';\n\n(foo(): boolean); // Error: either Implementation ~> boolean or Definition ~> boolean") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import { foo } from \"./A\";\n\n(foo(): boolean); // Error: either Implementation ~> boolean or Definition ~> boolean");
}
