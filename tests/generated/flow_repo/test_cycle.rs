#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_27793955() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("var B = require('./B');\n\nclass A extends B { }\n\nmodule.exports = A;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "var B = require(\"./B\");\n\nclass A extends B {}\n\nmodule.exports = A;"
    );
}
#[test]
fn test_b_js_format_1_2d31e9fb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("var A = require('./A');\n\n//class B extends A { }\n\nmodule.exports = B;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "var A = require(\"./A\");\n\n//class B extends A { }\n\nmodule.exports = B;"
    );
}
