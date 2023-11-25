#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_imp_js_format_1_8f9afb45() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("/* @flow */\n\nvar imp = require('./obj_annot');\nimp({ name: \"imp\" });");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/* @flow */\n\nvar imp = require(\"./obj_annot\");\nimp({ name: \"imp\" });"
    );
}
#[test]
fn test_obj_annot_js_format_1_0156e882() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nfunction foo(param: { name: string; }): number {\n    return param.id;\n}\n\nfoo({ name: \"test\" });\n\nmodule.exports = foo;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nfunction foo(param: { name: string }): number {\n  return param.id;\n}\n\nfoo({ name: \"test\" });\n\nmodule.exports = foo;");
}
