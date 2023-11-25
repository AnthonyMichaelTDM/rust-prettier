#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_foo_js_format_1_807360b0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/** @flow */\n/* this require routes to nothing, because\n   our node_modules/underscore directory\n   has been excluded. If package.json files\n   are being excluded properly, we'll get\n   'Required module not found'.\n */\nvar _ = require('underscore');\n\nfunction foo(): string {\n  return _.foo();\n}\n\nmodule.exports = foo;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/** @flow */\n/* this require routes to nothing, because\n   our node_modules/underscore directory\n   has been excluded. If package.json files\n   are being excluded properly, we'll get\n   'Required module not found'.\n */\nvar _ = require(\"underscore\");\n\nfunction foo(): string {\n  return _.foo();\n}\n\nmodule.exports = foo;");
}
