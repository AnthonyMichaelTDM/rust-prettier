#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_53d85635() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var React = require('react');\n\nclass C extends React.PureComponent {\n  props: { x: number };\n}\n(<C />); // error (\\`x\\` is a required prop)") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var React = require(\"react\");\n\nclass C extends React.PureComponent {\n  props: { x: number };\n}\n<C />; // error (\\`x\\` is a required prop)");
}
