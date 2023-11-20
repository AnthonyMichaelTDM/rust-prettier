#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_declare_class_js_format_1_f0063767() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare class C {\n    static x: number;\n    static foo(x: number): void;\n\n    constructor(x: string): void;\n}\n\nC.x = \"\";\nC.foo(\"\");\n\n(C.name: string);\n(C.name: number); // error, it's a string\n\ndeclare class D extends C { }\nnew D(123); // error, number ~> string") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare class C {\n  static x: number;\n  static foo(x: number): void;\n\n  constructor(x: string): void;\n}\n\nC.x = \"\";\nC.foo(\"\");\n\n(C.name: string);\n(C.name: number); // error, it's a string\n\ndeclare class D extends C {}\nnew D(123); // error, number ~> string");
}
