#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_class_static_js_format_1_9072c222() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class B {\n    static foo(): string { return \"\"; }\n}\n\nclass C extends B {\n    static bar(): string { return \"\"; }\n}\n\nvar x: number = C.bar();\nvar y: number = C.foo();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class B {\n  static foo(): string {\n    return \"\";\n  }\n}\n\nclass C extends B {\n  static bar(): string {\n    return \"\";\n  }\n}\n\nvar x: number = C.bar();\nvar y: number = C.foo();");
}
