#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_arrays_js_format_1_808ca406() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function foo(x) { return [x, x > 0, \"number \" + x]; }\n\nvar [n, b, s] = foo(42);\nn * s.length;") ? ;
    assert_eq ! (formatted , "function foo(x) {\n  return [x, x > 0, \"number \" + x];\n}\n\nvar [n, b, s] = foo(42);\nn * s.length;");
    Ok(())
}
#[test]
fn test_export_class_js_format_1_0b91c587() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class C {\n    x: number;\n    constructor(x: number) { this.x = x; }\n}\n\nmodule.exports = C;") ? ;
    assert_eq ! (formatted , "class C {\n  x: number;\n  constructor(x: number) {\n    this.x = x;\n  }\n}\n\nmodule.exports = C;");
    Ok(())
}
#[test]
fn test_generics_js_format_1_129c24ef() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "/* @flow */\n\nvar r: number = 0;\nfunction foo<X>(x: X): X { r = x; return x; }",
    )?;
    assert_eq!(
        formatted,
        "/* @flow */\n\nvar r: number = 0;\nfunction foo<X>(x: X): X {\n  r = x;\n  return x;\n}"
    );
    Ok(())
}
#[test]
fn test_import_class_js_format_1_cd6b8549() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("var C = require('./export_class');\n\nvar c = new C(\"\");")?;
    assert_eq!(
        formatted,
        "var C = require(\"./export_class\");\n\nvar c = new C(\"\");"
    );
    Ok(())
}
#[test]
fn test_locals_js_format_1_243ce8ca() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nfunction foo() {\n    var x = 0;\n    var y = x;\n}\n\nfunction bar(x: ?string): number {\n    if (x == null) x = \"\";\n    return x.length;\n}") ? ;
    assert_eq ! (formatted , "/* @flow */\n\nfunction foo() {\n  var x = 0;\n  var y = x;\n}\n\nfunction bar(x: ?string): number {\n  if (x == null) x = \"\";\n  return x.length;\n}");
    Ok(())
}
#[test]
fn test_objects_js_format_1_e3bf55f9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function C() { this.x = 0; }\nC.prototype.foo = function() { return this.x; }\n\nvar c = new C();\nvar x: string = c.foo();\n\nfunction foo() { return this.y; }\nfunction bar() { return this.foo(); }\nvar o = { y: \"\", foo: foo, bar: bar };\nvar o2 = { y: 0, foo: foo, bar: bar };\n\no.bar();\nvar y: number = o2.bar();") ? ;
    assert_eq ! (formatted , "function C() {\n  this.x = 0;\n}\nC.prototype.foo = function () {\n  return this.x;\n};\n\nvar c = new C();\nvar x: string = c.foo();\n\nfunction foo() {\n  return this.y;\n}\nfunction bar() {\n  return this.foo();\n}\nvar o = { y: \"\", foo: foo, bar: bar };\nvar o2 = { y: 0, foo: foo, bar: bar };\n\no.bar();\nvar y: number = o2.bar();");
    Ok(())
}
