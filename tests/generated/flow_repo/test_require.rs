#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_b_js_format_1_d525903c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* @flow */\n\nexports.numberValue = 42;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "/* @flow */\n\nexports.numberValue = 42;");
}
#[test]
fn test_c_js_format_1_e3121712() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* @flow */");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "/* @flow */");
}
#[test]
fn test_e_js_format_1_77a6bbe2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\n// Local \\`exports\\` var is just a ref to module.exports, so mutating the original\n// value will affect the exports object but re-binding it makes it useless and\n// does not affect the exports value.\nmodule.exports = {\n  numberValue: 42\n};\n\nexports = {stringValue: ''};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\n// Local \\`exports\\` var is just a ref to module.exports, so mutating the original\n// value will affect the exports object but re-binding it makes it useless and\n// does not affect the exports value.\nmodule.exports = {\n  numberValue: 42,\n};\n\nexports = { stringValue: \"\" };");
}
#[test]
fn test_provides_module_a_js_format_1_0316b2ae() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("/**\n * @providesModule A\n * @flow\n */\n\nexports.numberValue = 42;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/**\n * @providesModule A\n * @flow\n */\n\nexports.numberValue = 42;"
    );
}
#[test]
fn test_provides_module_d_js_format_1_acd1bf89() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/**\n * @providesModule D\n * @flow\n */");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "/**\n * @providesModule D\n * @flow\n */");
}
#[test]
fn test_not_builtin_require_js_format_1_fbed2cbb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("// @flow\n\nfunction require(x: string) {}\nrequire(\"not a module name\");");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// @flow\n\nfunction require(x: string) {}\nrequire(\"not a module name\");"
    );
}
#[test]
fn test_not_builtin_require_2_js_format_1_f1d93d21() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("// @flow\n\ntype require = number;\nvar a: require = 42;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// @flow\n\ntype require = number;\nvar a: require = 42;"
    );
}
#[test]
fn test_require_js_format_1_999b9bb0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nfunction takesANumber(num: number): void {}\nfunction takesAString(str: string): void {}\n\n// @providesModule\nvar A = require(\"A\");\ntakesANumber(A.numberValue);\ntakesAString(A.numberValue);\n\n// File path\nvar B = require(\"./B\");\ntakesANumber(B.numberValue);\ntakesAString(B.numberValue);\n\n// C.js exists, but not as a providesModule\nrequire(\"C\");\n\n// @providesModule D exists, but not as a filename\nrequire(\"./D\");\n\n// E exports an object with a numVal property\nvar E = require('./E');\nvar e_1: number = E.numberValue;\nE.stringValue; // Error: The E exports obj has no 'stringValue' property\n\n// We require that the param passed to require() be a string literal to support\n// guaranteed static extraction\nvar a = './E';\nrequire(a); // Error: Param must be string literal\nrequire(\\`./E\\`); // template literals are ok...\nrequire(\\`\\${'./E'}\\`); // error: but only if they have no expressions\n\n// require.call is allowed but circumverts Flow's static analysis\nrequire.call(null, \"DoesNotExist\");") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nfunction takesANumber(num: number): void {}\nfunction takesAString(str: string): void {}\n\n// @providesModule\nvar A = require(\"A\");\ntakesANumber(A.numberValue);\ntakesAString(A.numberValue);\n\n// File path\nvar B = require(\"./B\");\ntakesANumber(B.numberValue);\ntakesAString(B.numberValue);\n\n// C.js exists, but not as a providesModule\nrequire(\"C\");\n\n// @providesModule D exists, but not as a filename\nrequire(\"./D\");\n\n// E exports an object with a numVal property\nvar E = require(\"./E\");\nvar e_1: number = E.numberValue;\nE.stringValue; // Error: The E exports obj has no 'stringValue' property\n\n// We require that the param passed to require() be a string literal to support\n// guaranteed static extraction\nvar a = \"./E\";\nrequire(a); // Error: Param must be string literal\nrequire(\\`./E\\`); // template literals are ok...\nrequire(\\`\\${\"./E\"}\\`); // error: but only if they have no expressions\n\n// require.call is allowed but circumverts Flow's static analysis\nrequire.call(null, \"DoesNotExist\");");
}
