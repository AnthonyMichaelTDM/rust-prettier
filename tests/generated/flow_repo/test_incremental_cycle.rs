#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_a_js_format_1_d6707c2f() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer
        .format("// @flow\n\nclass A {\n  b: number;\n  c: string;\n}\n\nmodule.exports = A;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// @flow\n\nclass A {\n  b: number;\n  c: string;\n}\n\nmodule.exports = A;"
    );
}
#[test]
fn test_b_js_format_1_01dda3b4() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// @flow\nvar A = require ('./A');\nimport type C from './C';\n\nclass B extends A {\n  c: C;\n}\n\nmodule.exports = B;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\nvar A = require(\"./A\");\nimport type C from \"./C\";\n\nclass B extends A {\n  c: C;\n}\n\nmodule.exports = B;");
}
#[test]
fn test_c_js_format_1_4d4e6c9e() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// @flow\nvar A = require ('./A');\nimport type B from './B';\n\nclass C extends A {\n  b: B;\n}\n\nmodule.exports = C;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\nvar A = require(\"./A\");\nimport type B from \"./B\";\n\nclass C extends A {\n  b: B;\n}\n\nmodule.exports = C;");
}
