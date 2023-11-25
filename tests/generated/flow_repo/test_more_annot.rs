#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_client_object_js_format_1_9c516d5e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("var o = require('./object');\n\nvar a:number = o.w.z.y;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "var o = require(\"./object\");\n\nvar a: number = o.w.z.y;"
    );
}
#[test]
fn test_object_js_format_1_40670ea4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var o1 = { x: 0, y: \"\" };\nvar o2 = { z: o1 }\n\nvar o3 = {};\no3.w = o2;\n\n//declare var exports: { w: any };\n\nmodule.exports = o3;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var o1 = { x: 0, y: \"\" };\nvar o2 = { z: o1 };\n\nvar o3 = {};\no3.w = o2;\n\n//declare var exports: { w: any };\n\nmodule.exports = o3;");
}
#[test]
fn test_proto_js_format_1_1b2b9452() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function Foo() { this.x = 0; }\nFoo.prototype.m = function() { }\n\nvar o1: { x: number; m(): void } = new Foo();\n\nvar o2: Foo = new Foo();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function Foo() {\n  this.x = 0;\n}\nFoo.prototype.m = function () {};\n\nvar o1: { x: number, m(): void } = new Foo();\n\nvar o2: Foo = new Foo();");
}
#[test]
fn test_super_js_format_1_3ece38d1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "class C { m() { } }\nclass D extends C { }\n\nvar d: { +m: () => void } = new D();",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "class C {\n  m() {}\n}\nclass D extends C {}\n\nvar d: { +m: () => void } = new D();"
    );
}
