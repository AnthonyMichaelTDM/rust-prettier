#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_53d85635() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var React = require('react');\n\nclass C extends React.PureComponent {\n  props: { x: number };\n}\n(<C />); // error (\\`x\\` is a required prop)") ? ;
    assert_eq ! (formatted , "var React = require(\"react\");\n\nclass C extends React.PureComponent {\n  props: { x: number };\n}\n<C />; // error (\\`x\\` is a required prop)");
    Ok(())
}
