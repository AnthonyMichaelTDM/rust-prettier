#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_test_js_format_1_fd24be1e() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format("/* @flow */\nclass A {}\nclass B {}\n\nmodule.exports = { A, B };");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/* @flow */\nclass A {}\nclass B {}\n\nmodule.exports = { A, B };"
    );
}
#[test]
fn test_test_2_js_format_1_49f6569d() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("/* @flow */\nvar I = require(\"./test.js\");\n\nclass C extends I.A {}\n\nvar x: I.A = new C();\nvar y: I.B = new C();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\nvar I = require(\"./test.js\");\n\nclass C extends I.A {}\n\nvar x: I.A = new C();\nvar y: I.B = new C();");
}
#[test]
fn test_test_3_js_format_1_51103871() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("class A<X, Y, Z> {}\nclass B extends A<string, number, bool> {}\nclass C<X, Y, Z> extends B {}\n\nvar c: C<number, string, Array<bool>> = new C; // none of the type args matter\nvar a: A<string, number, Array<bool>> = c; // the third type arg is incorrect") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class A<X, Y, Z> {}\nclass B extends A<string, number, boolean> {}\nclass C<X, Y, Z> extends B {}\n\nvar c: C<number, string, Array<boolean>> = new C(); // none of the type args matter\nvar a: A<string, number, Array<boolean>> = c; // the third type arg is incorrect");
}
#[test]
fn test_test_4_js_format_1_5dbe7735() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("class C<X> { x: X; }\n\nfunction foo<X>(c: C<X>, x: X) { }\n\ntype O = { f: number };\n\nfoo((new C: C<O>), { f_: 0 });\n\nclass D extends C<O> {\n  bar() { this.x; }\n}\n\nfoo(new D, { f_: 0 });") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class C<X> {\n  x: X;\n}\n\nfunction foo<X>(c: C<X>, x: X) {}\n\ntype O = { f: number };\n\nfoo((new C(): C<O>), { f_: 0 });\n\nclass D extends C<O> {\n  bar() {\n    this.x;\n  }\n}\n\nfoo(new D(), { f_: 0 });");
}
