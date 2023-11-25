use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_index_js_format_1_bbe36057() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\n(require('Module1'): boolean);\nrequire('Module2');\nrequire('Module3');\nrequire('Module4');") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\n(require(\"Module1\"): boolean);\nrequire(\"Module2\");\nrequire(\"Module3\");\nrequire(\"Module4\");");
}
