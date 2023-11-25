#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_function_expression_js_format_1_0a0b4a07() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("(function() {}).length\ntypeof (function() {});\nexport default (function() {})();\n(function() {})()\\`\\`;\n(function() {})\\`\\`;\nnew (function() {});\n(function() {});\na = function f() {} || b;\n(function() {} && a);\na + function() {};\nnew function() {};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "(function () {}).length;\ntypeof function () {};\nexport default (function () {})();\n(function () {})()\\`\\`;\n(function () {})\\`\\`;\nnew (function () {})();\n(function () {});\na = function f() {} || b;\n(function () {}) && a;\na + function () {};\nnew (function () {})();");
}
#[test]
fn test_issue_10277_js_format_1_c981fef8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "(fold => fold)(fmap => algebra => function doFold(v) {return algebra(fmap(doFold)(v))})",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "((fold) => fold)(\n  (fmap) => (algebra) =>\n    function doFold(v) {\n      return algebra(fmap(doFold)(v));\n    },\n);");
}
