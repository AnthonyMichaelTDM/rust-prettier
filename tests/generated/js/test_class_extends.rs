#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_complex_js_format_1_9f67d618() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class loooooooooooooooooooong1 extends foooooooo(foooooooo(foooooooo(foooooooo(foooooooo(foooooooo(foooooooo(foooooooo()))))))) {}\n\nclass loooooooooooooooooooong2 extends function (make, model, year, owner) {\n  this.make = make;\n  this.model = model;\n  this.year = year;\n  this.owner = owner;\n} {}\n\nclass loooooooooooooooooooong3 extends class {\n  cconstructor(make, model, year, owner) {\n    this.make = make;\n    this.model = model;\n    this.year = year;\n    this.owner = owner;\n  }\n} {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class loooooooooooooooooooong1 extends foooooooo(\n  foooooooo(foooooooo(foooooooo(foooooooo(foooooooo(foooooooo(foooooooo())))))),\n) {}\n\nclass loooooooooooooooooooong2 extends function (make, model, year, owner) {\n  this.make = make;\n  this.model = model;\n  this.year = year;\n  this.owner = owner;\n} {}\n\nclass loooooooooooooooooooong3 extends class {\n  cconstructor(make, model, year, owner) {\n    this.make = make;\n    this.model = model;\n    this.year = year;\n    this.owner = owner;\n  }\n} {}");
}
#[test]
fn test_extends_js_format_1_f00976f6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// \"ArrowFunctionExpression\"\nclass a1 extends (() => {}) {}\n\n// \"AssignmentExpression\"\nclass a2 extends (b = c) {}\n\n// \"AwaitExpression\"\nasync function f() {\n  class a extends (await b) {}\n}\n\n// \"BinaryExpression\"\nclass a3 extends (b + c) {}\n\n// \"CallExpression\"\nclass a4 extends b() {}\n\n// \"ClassExpression\"\nclass a5 extends class {} {}\n\n// \"ConditionalExpression\"\nclass a6 extends (b ? c : d) {}\n\n// \"FunctionExpression\"\nclass a7 extends (function() {}) {}\n\n// \"LogicalExpression\"\nclass a8 extends (b || c) {}\n\n// \"MemberExpression\"\nclass a9 extends b.c {}\n\n// \"NewExpression\"\nclass a10 extends (new B()) {}\n\n// \"ObjectExpression\"\nclass a11 extends ({}) {}\n\n// \"SequenceExpression\"\nclass a12 extends (b, c) {}\n\n// \"TaggedTemplateExpression\"\nclass a13 extends \\`\\` {}\n\n// \"UnaryExpression\"\nclass a14 extends (void b) {}\n\n// \"UpdateExpression\"\nclass a15 extends (++b) {}\n\n// \"YieldExpression\"\nfunction* f2() {\n  // Flow has a bug parsing it.\n  // class a extends (yield 1) {}\n}\n\nx = class extends (++b) {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// \"ArrowFunctionExpression\"\nclass a1 extends (() => {}) {}\n\n// \"AssignmentExpression\"\nclass a2 extends (b = c) {}\n\n// \"AwaitExpression\"\nasync function f() {\n  class a extends (await b) {}\n}\n\n// \"BinaryExpression\"\nclass a3 extends (b + c) {}\n\n// \"CallExpression\"\nclass a4 extends b() {}\n\n// \"ClassExpression\"\nclass a5 extends class {} {}\n\n// \"ConditionalExpression\"\nclass a6 extends (b ? c : d) {}\n\n// \"FunctionExpression\"\nclass a7 extends function () {} {}\n\n// \"LogicalExpression\"\nclass a8 extends (b || c) {}\n\n// \"MemberExpression\"\nclass a9 extends b.c {}\n\n// \"NewExpression\"\nclass a10 extends (new B()) {}\n\n// \"ObjectExpression\"\nclass a11 extends ({}) {}\n\n// \"SequenceExpression\"\nclass a12 extends (b, c) {}\n\n// \"TaggedTemplateExpression\"\nclass a13 extends \\`\\` {}\n\n// \"UnaryExpression\"\nclass a14 extends (void b) {}\n\n// \"UpdateExpression\"\nclass a15 extends (++b) {}\n\n// \"YieldExpression\"\nfunction* f2() {\n  // Flow has a bug parsing it.\n  // class a extends (yield 1) {}\n}\n\nx = class extends (++b) {};");
}
#[test]
fn test_tuple_and_record_js_acorn_format_1_d41d8cd9() {
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
fn test_tuple_and_record_js_espree_format_1_d41d8cd9() {
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
fn test_tuple_and_record_js_flow_format_1_d41d8cd9() {
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
fn test_tuple_and_record_js_meriyah_format_1_d41d8cd9() {
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
fn test_tuple_and_record_js_typescript_format_1_d41d8cd9() {
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
fn test_tuple_and_record_js_format_1_0afc9ce8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A extends {} {}\nclass B extends #{} {}\n\nclass C extends [] {}\nclass D extends #[] {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class A extends ({}) {}\nclass B extends #{} {}\n\nclass C extends [] {}\nclass D extends #[] {}");
}
