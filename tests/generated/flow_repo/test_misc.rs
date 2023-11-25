#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_48be516c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\n/* @providesModule A */\n\nmodule.exports = {};\n\nvar A = {x:true, ...{}};\nmodule.exports.cls = A;\n\nfunction f(x:boolean) { }\nmodule.exports.fn = f;\n\nA.y = \"?\";\nA.x = A.y;\nf(A.x); // A.x is now a string, by def assign") ? ;
    assert_eq ! (formatted , "/* @providesModule A */\n\nmodule.exports = {};\n\nvar A = { x: true, ...{} };\nmodule.exports.cls = A;\n\nfunction f(x: boolean) {}\nmodule.exports.fn = f;\n\nA.y = \"?\";\nA.x = A.y;\nf(A.x); // A.x is now a string, by def assign");
    Ok(())
}
#[test]
fn test_b_js_format_1_d2b1e28d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\n/* @providesModule B */\n\nvar A = require('A').cls;\n\nfunction B() {\n  this.b = \"...\";\n}\n\nfunction f():number { return this.b; }\n\nB.prototype.s = 0;\nB.prototype.fn = f;\n\nmodule.exports = B;") ? ;
    assert_eq ! (formatted , "/* @providesModule B */\n\nvar A = require(\"A\").cls;\n\nfunction B() {\n  this.b = \"...\";\n}\n\nfunction f(): number {\n  return this.b;\n}\n\nB.prototype.s = 0;\nB.prototype.fn = f;\n\nmodule.exports = B;");
    Ok(())
}
#[test]
fn test_c_js_format_1_7bd1fcd2() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\n/* @providesModule C */\n\nvar B = require('B');\nvar f = require('A').fn;\n\nfunction C() {\n  var o = new B();\n  f(o.b);\n  f(o.s);\n  o.fn();\n}\n\nmodule.exports = C;") ? ;
    assert_eq ! (formatted , "/* @providesModule C */\n\nvar B = require(\"B\");\nvar f = require(\"A\").fn;\n\nfunction C() {\n  var o = new B();\n  f(o.b);\n  f(o.s);\n  o.fn();\n}\n\nmodule.exports = C;");
    Ok(())
}
#[test]
fn test_d_js_format_1_31cd7383() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\n/* @providesModule D */\n\nvar f = require('A').fn;\n\nfunction g():string { return this.i; }\n\nvar o = {fn: g, ...{}};\no.i = true;\n\nvar i = o.fn();\nf(i);\n\nmodule.exports = \"D for dummy\";") ? ;
    assert_eq ! (formatted , "/* @providesModule D */\n\nvar f = require(\"A\").fn;\n\nfunction g(): string {\n  return this.i;\n}\n\nvar o = { fn: g, ...{} };\no.i = true;\n\nvar i = o.fn();\nf(i);\n\nmodule.exports = \"D for dummy\";");
    Ok(())
}
#[test]
fn test_e_js_format_1_79fda66a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\n/* @providesModule E */\n\nfunction h(x:number) { }\nvar proto = { fn: h }\n\nvar o = Object.create(proto);\no.fn(false);\n\nmodule.exports = {obj: o};") ? ;
    assert_eq ! (formatted , "/* @providesModule E */\n\nfunction h(x: number) {}\nvar proto = { fn: h };\n\nvar o = Object.create(proto);\no.fn(false);\n\nmodule.exports = { obj: o };");
    Ok(())
}
#[test]
fn test_f_js_format_1_24fa6d3b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function fn2(x) { return x.length * 4; }\nfn2({length: 'hi'});\n\nfunction foo(x: Array<number>): string {\n  return x.length;\n}") ? ;
    assert_eq ! (formatted , "function fn2(x) {\n  return x.length * 4;\n}\nfn2({ length: \"hi\" });\n\nfunction foo(x: Array<number>): string {\n  return x.length;\n}");
    Ok(())
}
#[test]
fn test_g_js_format_1_826d4e3a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var a = { length: \"duck\" };\na.length = 123;\na.length();\n\nvar b = [ 123 ];\nb.length = \"duck\";\nb.length();") ? ;
    assert_eq ! (formatted , "var a = { length: \"duck\" };\na.length = 123;\na.length();\n\nvar b = [123];\nb.length = \"duck\";\nb.length();");
    Ok(())
}
