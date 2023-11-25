#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_bar_js_format_1_90aa8a23() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\n/* @providesModule Bar */\n\nvar Qux = require('Qux');\n\nclass Bar {\n  y:number;\n  self:Bar;\n  constructor(y:number) {\n    this.y = y;\n    this.self = this;\n  }\n\n  bar(z:string,u:string):string {\n    new Qux().w = \"?\";\n    return z;\n  }\n}\n\nmodule.exports = Bar;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @providesModule Bar */\n\nvar Qux = require(\"Qux\");\n\nclass Bar {\n  y: number;\n  self: Bar;\n  constructor(y: number) {\n    this.y = y;\n    this.self = this;\n  }\n\n  bar(z: string, u: string): string {\n    new Qux().w = \"?\";\n    return z;\n  }\n}\n\nmodule.exports = Bar;");
}
#[test]
fn test_foo_js_format_1_a5398e31() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\n/* @providesModule Foo */\n\nvar Bar = require('Bar');\nvar Qux = require('Qux');\n\nclass Foo extends Qux {\n  x:string;\n  constructor(x:string) {\n    this.x = x;\n  }\n\n  foo(y:string,z):number {\n    this.x = y;\n    var u = new Foo(\"...\").qux();\n    var v = new Bar(y);\n    v.self = v;\n    return v.bar(z,u);\n  }\n\n  fooqux(x:string) {\n    this.x;\n  }\n}\n\nmodule.exports = Foo;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @providesModule Foo */\n\nvar Bar = require(\"Bar\");\nvar Qux = require(\"Qux\");\n\nclass Foo extends Qux {\n  x: string;\n  constructor(x: string) {\n    this.x = x;\n  }\n\n  foo(y: string, z): number {\n    this.x = y;\n    var u = new Foo(\"...\").qux();\n    var v = new Bar(y);\n    v.self = v;\n    return v.bar(z, u);\n  }\n\n  fooqux(x: string) {\n    this.x;\n  }\n}\n\nmodule.exports = Foo;");
}
#[test]
fn test_qux_js_format_1_3d4724cb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\n/* @providesModule Qux */\n\nclass Qux {\n  w:number;\n\n  qux() { return this.w; }\n\n  fooqux(x:number) { }\n}\n\nmodule.exports = Qux;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @providesModule Qux */\n\nclass Qux {\n  w: number;\n\n  qux() {\n    return this.w;\n  }\n\n  fooqux(x: number) {}\n}\n\nmodule.exports = Qux;");
}
