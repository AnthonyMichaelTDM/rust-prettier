#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_keyvalue_js_format_1_c3dedc51() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nlet tests = [\n  function(x: { [key: number]: string }) {\n    (x[\"\"]: number);\n  }\n];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nlet tests = [\n  function (x: { [key: number]: string }) {\n    (x[\"\"]: number);\n  },\n];");
}
