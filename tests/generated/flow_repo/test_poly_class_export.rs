#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_ebd5953d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("// @flow\n\nclass A<T> {\n  x: T\n}\n\nmodule.exports = A;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// @flow\n\nclass A<T> {\n  x: T;\n}\n\nmodule.exports = A;"
    );
}
#[test]
fn test_b_js_format_1_eb1522a2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nlet A = require('./A');\n\nclass B extends A<string> {\n  constructor() {\n    super();\n  }\n}\n\nmodule.exports = new B();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nlet A = require(\"./A\");\n\nclass B extends A<string> {\n  constructor() {\n    super();\n  }\n}\n\nmodule.exports = new B();");
}
#[test]
fn test_c_js_format_1_17650170() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n// This test exports a function whose return type is the class's \\`this\\` type.\n// It should be inferred (no annotation required).\n\nclass Foo {\n  foo(): this {\n    return this;\n  }\n}\n\nexport function f(x: Foo) {\n  return x.foo();\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n// This test exports a function whose return type is the class's \\`this\\` type.\n// It should be inferred (no annotation required).\n\nclass Foo {\n  foo(): this {\n    return this;\n  }\n}\n\nexport function f(x: Foo) {\n  return x.foo();\n}");
}
