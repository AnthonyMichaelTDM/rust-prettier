#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_ccdd8e05() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @flow\nvar a: string = 0;\nmodule.exports = a;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// @flow\nvar a: string = 0;\nmodule.exports = a;"
    );
}
#[test]
fn test_b_js_format_1_1837df07() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("// @flow\nvar a = require('./a');\nvar b: number = a;\nmodule.exports = b;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// @flow\nvar a = require(\"./a\");\nvar b: number = a;\nmodule.exports = b;"
    );
}
#[test]
fn test_c_js_format_1_fbec86c4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("// @flow\nvar b = require('./b');\nvar c: string = b;\nmodule.exports = c;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// @flow\nvar b = require(\"./b\");\nvar c: string = b;\nmodule.exports = c;"
    );
}
