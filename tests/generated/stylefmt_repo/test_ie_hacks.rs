#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_ie_hacks_css_format_1_fa5b1f6f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("* html p {font-size: 5em; }\n.class {\n*zoom: 1;_width: 200px;\n+color:red;\n*+color:red;\ncolor:red\\\\9;\ncolor:red\\\\0;\ncolor:red\\\\9\\\\0;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "* html p {\n  font-size: 5em;\n}\n.class {\n  *zoom: 1;\n  _width: 200px;\n  +color: red;\n  *+color: red;\n  color: red\\\\9;\n  color: red\\\\0;\n  color: red\\\\9\\\\0;\n}");
}
