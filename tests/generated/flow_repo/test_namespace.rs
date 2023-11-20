#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_client_js_format_1_8632a8a4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("var ns = require('./namespace')\n\nvar bar: string = ns.foo");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "var ns = require(\"./namespace\");\n\nvar bar: string = ns.foo;"
    );
}
#[test]
fn test_namespace_js_format_1_b40369fd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/*@flow*/// import type { T } from '...'\ntype T = (x:number) => void;\nvar f: T = function(x:string): void { }\n\ntype Map<X,Y> = (x:X) => Y;\n\nfunction bar<U,V>(x:U, f:Map<U,V>): V {\n    return f(x);\n}\n\nvar y:number = bar(0, x => \"\");\n\ntype Seq = number | Array<Seq>;\nvar s1:Seq = [0,[0]];\nvar s2:Seq = [[\"\"]];\n\nmodule.exports = { foo: (\"\": number) };") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/*@flow*/ // import type { T } from '...'\ntype T = (x: number) => void;\nvar f: T = function (x: string): void {};\n\ntype Map<X, Y> = (x: X) => Y;\n\nfunction bar<U, V>(x: U, f: Map<U, V>): V {\n  return f(x);\n}\n\nvar y: number = bar(0, (x) => \"\");\n\ntype Seq = number | Array<Seq>;\nvar s1: Seq = [0, [0]];\nvar s2: Seq = [[\"\"]];\n\nmodule.exports = { foo: (\"\": number) };");
}
