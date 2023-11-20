#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_961cc3f5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function foo(x: Array<Object>): Array<Object> {\n    return x.sort((a, b) => a.foo - b.foo);\n}\n\n// Make sure Object works with Object.keys()\nfunction bar(x: Object): Array<string> {\n  return Object.keys(x);\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function foo(x: Array<Object>): Array<Object> {\n  return x.sort((a, b) => a.foo - b.foo);\n}\n\n// Make sure Object works with Object.keys()\nfunction bar(x: Object): Array<string> {\n  return Object.keys(x);\n}");
}
