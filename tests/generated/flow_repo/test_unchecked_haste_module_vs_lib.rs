use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_buffer_js_format_1_255d1364() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule buffer\n *\n * Not in flow.\n * If this module is successfully imported/required, its\n * type will be Any, so arbitraty uses won't cause errors.\n * However, if a library module declaration is bound to\n * the same name as an unchecked module, it will be used\n * to satisfy imports/requires instead.\n */\n\nexport var INSPECT_MAX_BYTES = 0;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule buffer\n *\n * Not in flow.\n * If this module is successfully imported/required, its\n * type will be Any, so arbitraty uses won't cause errors.\n * However, if a library module declaration is bound to\n * the same name as an unchecked module, it will be used\n * to satisfy imports/requires instead.\n */\n\nexport var INSPECT_MAX_BYTES = 0;");
}
#[test]
fn test_test_js_format_1_a59709d3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * Copyright 2004-present Facebook. All Rights Reserved.\n * @flow\n */\n\n/* 'buffer' is the name of both an unchecked module in this directory,\n * and a module declared in library file node.js.\n * If the require below resolves to the unchecked module, the mistyping\n * that follows will cause no errors, but if we resolve to the library\n * instead, we'll get the desired error.\n */\nvar buffer = require(\"buffer\");\nvar x: string = buffer.INSPECT_MAX_BYTES; // error, number ~/> string") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * Copyright 2004-present Facebook. All Rights Reserved.\n * @flow\n */\n\n/* 'buffer' is the name of both an unchecked module in this directory,\n * and a module declared in library file node.js.\n * If the require below resolves to the unchecked module, the mistyping\n * that follows will cause no errors, but if we resolve to the library\n * instead, we'll get the desired error.\n */\nvar buffer = require(\"buffer\");\nvar x: string = buffer.INSPECT_MAX_BYTES; // error, number ~/> string");
}
