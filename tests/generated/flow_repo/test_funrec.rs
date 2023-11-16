#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_funrec_js_format_1_7f4e2dfa() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("function bar(x) { return x; }\nfunction foo() {\n    return function bound() {\n        return bar(bound);\n    };\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function bar(x) {\n  return x;\n}\nfunction foo() {\n  return function bound() {\n    return bar(bound);\n  };\n}");
}
