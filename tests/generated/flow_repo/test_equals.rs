#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_equals_js_format_1_b7f96cee() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\n(1 == 1);\n(\"foo\" == \"bar\");\n(1 == null);\n(null == 1);\n(1 == \"\"); // error\n(\"\" == 1); // error\n\nvar x = (null : ?number);\n(x == 1);\n(1 == x);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\n1 == 1;\n\"foo\" == \"bar\";\n1 == null;\nnull == 1;\n1 == \"\"; // error\n\"\" == 1; // error\n\nvar x = (null: ?number);\nx == 1;\n1 == x;");
}
