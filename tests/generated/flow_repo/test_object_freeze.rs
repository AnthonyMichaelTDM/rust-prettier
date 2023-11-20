#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_object_freeze_js_format_1_14998c24() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nvar foo = Object.freeze({bar: '12345'});\nfoo.bar = '23456'; // error\n\nObject.assign(foo, {bar: '12345'}); // error\n\nvar baz = {baz: 12345};\nvar bliffl = Object.freeze({bar: '12345', ...baz});\nbliffl.bar = '23456'; // error\nbliffl.baz = 3456; // error\nbliffl.corge; // error\nbliffl.constructor = baz; // error\nbliffl.toString = function() {}; // error\n\nbaz.baz = 0;\n\nvar x : number = Object.freeze(123);\n\nvar xx : { x: number } = Object.freeze({ x: \"error\" })") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nvar foo = Object.freeze({ bar: \"12345\" });\nfoo.bar = \"23456\"; // error\n\nObject.assign(foo, { bar: \"12345\" }); // error\n\nvar baz = { baz: 12345 };\nvar bliffl = Object.freeze({ bar: \"12345\", ...baz });\nbliffl.bar = \"23456\"; // error\nbliffl.baz = 3456; // error\nbliffl.corge; // error\nbliffl.constructor = baz; // error\nbliffl.toString = function () {}; // error\n\nbaz.baz = 0;\n\nvar x: number = Object.freeze(123);\n\nvar xx: { x: number } = Object.freeze({ x: \"error\" });");
}
