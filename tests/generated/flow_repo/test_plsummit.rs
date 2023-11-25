#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_arrays_js_format_1_808ca406() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function foo(x) { return [x, x > 0, \"number \" + x]; }\n\nvar [n, b, s] = foo(42);\nn * s.length;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function foo(x) {\n  return [x, x > 0, \"number \" + x];\n}\n\nvar [n, b, s] = foo(42);\nn * s.length;");
}
#[test]
fn test_export_class_js_format_1_0b91c587() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class C {\n    x: number;\n    constructor(x: number) { this.x = x; }\n}\n\nmodule.exports = C;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class C {\n  x: number;\n  constructor(x: number) {\n    this.x = x;\n  }\n}\n\nmodule.exports = C;");
}
#[test]
fn test_generics_js_format_1_129c24ef() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("/* @flow */\n\nvar r: number = 0;\nfunction foo<X>(x: X): X { r = x; return x; }");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/* @flow */\n\nvar r: number = 0;\nfunction foo<X>(x: X): X {\n  r = x;\n  return x;\n}"
    );
}
#[test]
fn test_import_class_js_format_1_cd6b8549() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("var C = require('./export_class');\n\nvar c = new C(\"\");");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "var C = require(\"./export_class\");\n\nvar c = new C(\"\");"
    );
}
#[test]
fn test_locals_js_format_1_243ce8ca() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nfunction foo() {\n    var x = 0;\n    var y = x;\n}\n\nfunction bar(x: ?string): number {\n    if (x == null) x = \"\";\n    return x.length;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nfunction foo() {\n  var x = 0;\n  var y = x;\n}\n\nfunction bar(x: ?string): number {\n  if (x == null) x = \"\";\n  return x.length;\n}");
}
#[test]
fn test_objects_js_format_1_e3bf55f9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function C() { this.x = 0; }\nC.prototype.foo = function() { return this.x; }\n\nvar c = new C();\nvar x: string = c.foo();\n\nfunction foo() { return this.y; }\nfunction bar() { return this.foo(); }\nvar o = { y: \"\", foo: foo, bar: bar };\nvar o2 = { y: 0, foo: foo, bar: bar };\n\no.bar();\nvar y: number = o2.bar();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function C() {\n  this.x = 0;\n}\nC.prototype.foo = function () {\n  return this.x;\n};\n\nvar c = new C();\nvar x: string = c.foo();\n\nfunction foo() {\n  return this.y;\n}\nfunction bar() {\n  return this.foo();\n}\nvar o = { y: \"\", foo: foo, bar: bar };\nvar o2 = { y: 0, foo: foo, bar: bar };\n\no.bar();\nvar y: number = o2.bar();");
}
