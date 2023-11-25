#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_function_js_format_1_f4aad13a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nfunction Bar(x: number) {\n  this.x = x;\n}\nBar.prototype.getX = function() { return this.x; }\nBar.prototype.getY = function(): string { return this.y; }\n\nmodule.exports = Bar;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nfunction Bar(x: number) {\n  this.x = x;\n}\nBar.prototype.getX = function () {\n  return this.x;\n};\nBar.prototype.getY = function (): string {\n  return this.y;\n};\n\nmodule.exports = Bar;");
}
#[test]
fn test_proto_js_format_1_0081bdc7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function Foo() { }\nvar o = new Foo();\nvar x:number = o.x;\n\nFoo.prototype.m = function() { return this.x; }\n\nvar y:number = o.m();\no.x = \"...\";\n\nFoo.prototype = { m: function() { return false; } }\n\nvar export_o: { x:any; } = o; // awkward type cast\n\nmodule.exports = export_o;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function Foo() {}\nvar o = new Foo();\nvar x: number = o.x;\n\nFoo.prototype.m = function () {\n  return this.x;\n};\n\nvar y: number = o.m();\no.x = \"...\";\n\nFoo.prototype = {\n  m: function () {\n    return false;\n  },\n};\n\nvar export_o: { x: any } = o; // awkward type cast\n\nmodule.exports = export_o;");
}
#[test]
fn test_sealed_js_format_1_67a32026() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var o = require('./proto');\n\no.z = 0;\nvar x:string = o.x;\n\nvar Bar = require('./function');\nvar a = new Bar(234);\na.x = 123;\na.y = 'abc'; // error, needs to be declared in Bar's constructor\n(a.getX(): number);\n(a.getY(): string);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var o = require(\"./proto\");\n\no.z = 0;\nvar x: string = o.x;\n\nvar Bar = require(\"./function\");\nvar a = new Bar(234);\na.x = 123;\na.y = \"abc\"; // error, needs to be declared in Bar's constructor\n(a.getX(): number);\n(a.getY(): string);");
}
