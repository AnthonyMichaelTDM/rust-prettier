#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_client_js_format_1_8632a8a4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("var ns = require('./namespace')\n\nvar bar: string = ns.foo")?;
    assert_eq!(
        formatted,
        "var ns = require(\"./namespace\");\n\nvar bar: string = ns.foo;"
    );
    Ok(())
}
#[test]
fn test_namespace_js_format_1_b40369fd() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/*@flow*/// import type { T } from '...'\ntype T = (x:number) => void;\nvar f: T = function(x:string): void { }\n\ntype Map<X,Y> = (x:X) => Y;\n\nfunction bar<U,V>(x:U, f:Map<U,V>): V {\n    return f(x);\n}\n\nvar y:number = bar(0, x => \"\");\n\ntype Seq = number | Array<Seq>;\nvar s1:Seq = [0,[0]];\nvar s2:Seq = [[\"\"]];\n\nmodule.exports = { foo: (\"\": number) };") ? ;
    assert_eq ! (formatted , "/*@flow*/ // import type { T } from '...'\ntype T = (x: number) => void;\nvar f: T = function (x: string): void {};\n\ntype Map<X, Y> = (x: X) => Y;\n\nfunction bar<U, V>(x: U, f: Map<U, V>): V {\n  return f(x);\n}\n\nvar y: number = bar(0, (x) => \"\");\n\ntype Seq = number | Array<Seq>;\nvar s1: Seq = [0, [0]];\nvar s2: Seq = [[\"\"]];\n\nmodule.exports = { foo: (\"\": number) };");
    Ok(())
}
