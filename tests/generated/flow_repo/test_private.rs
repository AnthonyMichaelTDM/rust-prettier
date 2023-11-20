#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_private_js_format_1_a4b67164() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A {\n  x: number;\n  _x: string;\n  __x: number;\n  constructor() { this.x = 0; this._x = \"\"; this.__x = 0; }\n}\n\nclass B extends A {\n  foo() {\n    var x: number = this.x;\n    var _x: string = this._x;\n    var __x: number = this.__x;\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class A {\n  x: number;\n  _x: string;\n  __x: number;\n  constructor() {\n    this.x = 0;\n    this._x = \"\";\n    this.__x = 0;\n  }\n}\n\nclass B extends A {\n  foo() {\n    var x: number = this.x;\n    var _x: string = this._x;\n    var __x: number = this.__x;\n  }\n}");
}
