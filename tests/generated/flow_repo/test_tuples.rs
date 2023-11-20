#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_array_js_format_1_458f7394() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nfunction foo(x: Array<number>): [number, ?number] {\n  return x; // Error, can't enforce arity when flowing array to tuple\n}\n\nfunction foo(x: Array<number>): [number, ?number] {\n  return [x[0], x[1]]; // OK. This is unsound, but at least arity is enforced\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nfunction foo(x: Array<number>): [number, ?number] {\n  return x; // Error, can't enforce arity when flowing array to tuple\n}\n\nfunction foo(x: Array<number>): [number, ?number] {\n  return [x[0], x[1]]; // OK. This is unsound, but at least arity is enforced\n}");
}
#[test]
fn test_optional_js_format_1_3189e013() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n([0, undefined]: [number, ?string]); // Ok, correct arity\n([0]: [number, ?string]); // Error, arity is enforced\n\n([]: [?number, string]); // error, since second element is not marked optional") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n([0, undefined]: [number, ?string]); // Ok, correct arity\n([0]: [number, ?string]); // Error, arity is enforced\n\n([]: [?number, string]); // error, since second element is not marked optional");
}
#[test]
fn test_spread_js_babel_flow_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_spread_js_format_1_47041b98() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("type A = [...S];\n\ntype B = [...foo: S];");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "type A = [...S];\n\ntype B = [...foo: S];");
}
#[test]
fn test_too_few_js_format_1_f6d03650() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nfunction foo(a: [Object, Object]) {}\n\nfoo([ {} ]); // error, too few elements in array passed to a tuple") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nfunction foo(a: [Object, Object]) {}\n\nfoo([{}]); // error, too few elements in array passed to a tuple");
}
#[test]
fn test_tuples_js_format_1_7b0665a3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var a: [] = [];\nvar b: [] = [123]; // Error - arity mismatch\nvar c: [number] = []; // nope\nvar d: [number, string] = [123,'duck'];\nvar e: [number, string,] = [123,'duck'];\nvar f: [number, string] = [123, 456];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var a: [] = [];\nvar b: [] = [123]; // Error - arity mismatch\nvar c: [number] = []; // nope\nvar d: [number, string] = [123, \"duck\"];\nvar e: [number, string] = [123, \"duck\"];\nvar f: [number, string] = [123, 456];");
}
