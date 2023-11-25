#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_300a1a70() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A {\n  static x: number;\n  static y: string;\n  static foo(x: number) { }\n  static bar(y: string) { }\n}\nA.qux = function(x: string) { } // error?\n\nclass B extends A {\n  static x: string; // error?\n  static foo(x: string) { } // error?\n  static main() {\n    B.x = 0; // error\n    B.x = \"\";\n    B.foo(0); // error\n    B.foo(\"\");\n    B.y = 0; // error\n    B.bar(0); // error\n    B.qux(0); // error\n  }\n  static create(): A {\n    return new this();\n  }\n\n  static badCreate(): number {\n    return new this(); // error B ~> number\n  }\n}\n\nclass C<X> {\n  static x: X;\n  static bar(x: X) { }\n  static create(): C<*> {\n    return new this();\n  }\n}\n\nclass D extends C<string> {\n  static main() {\n    D.foo(0); // error?\n\n    D.bar(0);\n  }\n}\n\nvar d: C<*> = D.create();\n(new A: typeof A);\n(B: typeof A);\n\nclass E {\n  static x: number;\n  static foo(): string {\n    this.bar(); // error\n    return this.x; // error\n  }\n}\n\n// note: above classdefs are sufficiently annotated to export\nmodule.exports = {\n  A: A, B: B, C: C, D: D, E: E\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class A {\n  static x: number;\n  static y: string;\n  static foo(x: number) {}\n  static bar(y: string) {}\n}\nA.qux = function (x: string) {}; // error?\n\nclass B extends A {\n  static x: string; // error?\n  static foo(x: string) {} // error?\n  static main() {\n    B.x = 0; // error\n    B.x = \"\";\n    B.foo(0); // error\n    B.foo(\"\");\n    B.y = 0; // error\n    B.bar(0); // error\n    B.qux(0); // error\n  }\n  static create(): A {\n    return new this();\n  }\n\n  static badCreate(): number {\n    return new this(); // error B ~> number\n  }\n}\n\nclass C<X> {\n  static x: X;\n  static bar(x: X) {}\n  static create(): C<*> {\n    return new this();\n  }\n}\n\nclass D extends C<string> {\n  static main() {\n    D.foo(0); // error?\n\n    D.bar(0);\n  }\n}\n\nvar d: C<*> = D.create();\n(new A(): typeof A);\n(B: typeof A);\n\nclass E {\n  static x: number;\n  static foo(): string {\n    this.bar(); // error\n    return this.x; // error\n  }\n}\n\n// note: above classdefs are sufficiently annotated to export\nmodule.exports = {\n  A: A,\n  B: B,\n  C: C,\n  D: D,\n  E: E,\n};");
}
