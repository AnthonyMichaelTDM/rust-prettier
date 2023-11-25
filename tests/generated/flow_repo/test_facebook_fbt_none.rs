#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_main_js_format_1_a8c7d48d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\nvar React = require('react');\n(<fbt />: React.Element<*>);\n(<fbt />: number); // Error: ReactElement ~> number") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\nvar React = require(\"react\");\n(<fbt />: React.Element<*>);\n(<fbt />: number); // Error: ReactElement ~> number");
}
