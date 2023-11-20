#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_class_expr_js_format_1_87a50f17() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n// issue #1191\n\nconst Thing = class Thing {\n  zark() {\n    this.x = 123; // error: property not found (must be declared)\n  }\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n// issue #1191\n\nconst Thing = class Thing {\n  zark() {\n    this.x = 123; // error: property not found (must be declared)\n  }\n};");
}
#[test]
fn test_contra_js_format_1_1da24ac8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// Counterexample with contravariant this type\n\nclass C {\n  next: this; // error (see below for exploit): \\`this\\` should only appear in\n              // covariant positions\n}\n\nclass D extends C { }\n\nvar d = new D();\n(d: C).next = new C;\n(d.next: D); // sneaky\n\nclass A {\n  foo<X: this>(that: X) { } // error: can't hide contravariance using a bound\n}\n\nclass B extends A {\n  foo<Y: this>(that: Y) { } // error (see above, catches hidden override)\n}\n\n// covariance checks on this type in invariant positions\n\nclass Invariant {\n  out_object(): { _: this } { return { _: this }; }\n  in_object(_: { _: this }) { }\n  inout_object: { _: this };\n\n  out_array(): Array<this> { return [this]; }\n  in_array(_: Array<this>) { }\n  inout_array: Array<this>;\n}\n\n// covariance checks on this type as type args\n\nclass Misc {\n  // Set<X> has invariant X\n  out_set(): Set<this> { return new Set().add(this); }\n  in_set(_: Set<this>) { }\n  inout_set: Set<this>;\n\n  // Promise<X> has covariant X\n  async out_promise(): Promise<this> { return this; }\n  in_promise(_: Promise<this>) { }\n  inout_promise: Promise<this>;\n\n  // Generator<X,Y,Z> has covariant X, covariant Y, contravariant Z\n  *out_generator(): Generator<this,this,this> {\n    yield this;\n    return this;\n  }\n  in_generator(_: Generator<this,this,this>) { }\n  inout_generator: Generator<this,this,this>;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// Counterexample with contravariant this type\n\nclass C {\n  next: this; // error (see below for exploit): \\`this\\` should only appear in\n  // covariant positions\n}\n\nclass D extends C {}\n\nvar d = new D();\n(d: C).next = new C();\n(d.next: D); // sneaky\n\nclass A {\n  foo<X: this>(that: X) {} // error: can't hide contravariance using a bound\n}\n\nclass B extends A {\n  foo<Y: this>(that: Y) {} // error (see above, catches hidden override)\n}\n\n// covariance checks on this type in invariant positions\n\nclass Invariant {\n  out_object(): { _: this } {\n    return { _: this };\n  }\n  in_object(_: { _: this }) {}\n  inout_object: { _: this };\n\n  out_array(): Array<this> {\n    return [this];\n  }\n  in_array(_: Array<this>) {}\n  inout_array: Array<this>;\n}\n\n// covariance checks on this type as type args\n\nclass Misc {\n  // Set<X> has invariant X\n  out_set(): Set<this> {\n    return new Set().add(this);\n  }\n  in_set(_: Set<this>) {}\n  inout_set: Set<this>;\n\n  // Promise<X> has covariant X\n  async out_promise(): Promise<this> {\n    return this;\n  }\n  in_promise(_: Promise<this>) {}\n  inout_promise: Promise<this>;\n\n  // Generator<X,Y,Z> has covariant X, covariant Y, contravariant Z\n  *out_generator(): Generator<this, this, this> {\n    yield this;\n    return this;\n  }\n  in_generator(_: Generator<this, this, this>) {}\n  inout_generator: Generator<this, this, this>;\n}");
}
#[test]
fn test_export_js_format_1_210008c2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export class A1 {\n  foo(): this { return this; }\n  bar(): this { return this; }\n}\n\nexport class A2<X> {\n  foo(): this { return this; }\n  bar(): this { return this; }\n  qux(x: X): X { return x; }\n}\n\nexport class A3<X> extends A2<X> {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export class A1 {\n  foo(): this {\n    return this;\n  }\n  bar(): this {\n    return this;\n  }\n}\n\nexport class A2<X> {\n  foo(): this {\n    return this;\n  }\n  bar(): this {\n    return this;\n  }\n  qux(x: X): X {\n    return x;\n  }\n}\n\nexport class A3<X> extends A2<X> {}");
}
#[test]
fn test_generics_js_format_1_4ffe1219() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class Generic<X> {\n  clone(): Generic<X> { return this; }\n}\n\nclass Implicit<X> { arg: X; val: X; }\nclass ImplicitNumber extends Implicit { arg: number; }\n\n(new ImplicitNumber().val: string) // error: number ~> string") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class Generic<X> {\n  clone(): Generic<X> {\n    return this;\n  }\n}\n\nclass Implicit<X> {\n  arg: X;\n  val: X;\n}\nclass ImplicitNumber extends Implicit {\n  arg: number;\n}\n\n(new ImplicitNumber().val: string); // error: number ~> string");
}
#[test]
fn test_import_js_format_1_da2a2cb2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// Check that imports are handled properly with this types\n\nimport { A1 } from './export';\nimport type { A2 } from './export';\nimport { A3 } from './export';\n\nclass B1 extends A1 {\n  foo(): B1 { return new B1(); } // error\n}\n\n(new B1().bar(): B1); // OK\n\nclass B3<X> extends A3<X> {\n  foo(): B3<X> { return new B3(); } // error\n}\n\n(new B3().bar(): B3<*>); // OK\n(new B3().qux(0): string); // error\n\n(new B3().bar(): A2<*>); // OK\n((new B3().bar(): B3<string>): A2<number>); // error\n((new B3(): A2<number>).qux(0): string); // error") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// Check that imports are handled properly with this types\n\nimport { A1 } from \"./export\";\nimport type { A2 } from \"./export\";\nimport { A3 } from \"./export\";\n\nclass B1 extends A1 {\n  foo(): B1 {\n    return new B1();\n  } // error\n}\n\n(new B1().bar(): B1); // OK\n\nclass B3<X> extends A3<X> {\n  foo(): B3<X> {\n    return new B3();\n  } // error\n}\n\n(new B3().bar(): B3<*>); // OK\n(new B3().qux(0): string); // error\n\n(new B3().bar(): A2<*>); // OK\n((new B3().bar(): B3<string>): A2<number>); // error\n((new B3(): A2<number>).qux(0): string); // error");
}
#[test]
fn test_interface_js_format_1_ae033af3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("interface I { xs: Array<this>; }\ninterface J { f(): J; }\nclass C {\n  xs: Array<C>;\n  f(): C { return this; }\n}\nfunction foo(c: C): I { return c; }\nfunction bar(c: C): J { return c; }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "interface I {\n  xs: Array<this>;\n}\ninterface J {\n  f(): J;\n}\nclass C {\n  xs: Array<C>;\n  f(): C {\n    return this;\n  }\n}\nfunction foo(c: C): I {\n  return c;\n}\nfunction bar(c: C): J {\n  return c;\n}");
}
#[test]
fn test_lib_client_js_format_1_b4349f5a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("(new DoublyLinkedList().prev(): DoublyLinkedList);\n(new DoublyLinkedList().next(): DoublyLinkedList)\n\nvar MiniImmutable = require(\"mini-immutable\");\nclass C {\n  map: MiniImmutable.OrderedMap<number,string>;\n  update() {\n    this.map = this.map.set(0,\"\");\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "(new DoublyLinkedList().prev(): DoublyLinkedList);\n(new DoublyLinkedList().next(): DoublyLinkedList);\n\nvar MiniImmutable = require(\"mini-immutable\");\nclass C {\n  map: MiniImmutable.OrderedMap<number, string>;\n  update() {\n    this.map = this.map.set(0, \"\");\n  }\n}");
}
#[test]
fn test_self_js_format_1_49876064() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A {\n  foo() { return this; } // return of foo is not annotated to get around\n                         // substituting this below\n  bar(): this { return new A().foo(); } // same as returning : A, so error\n  qux(): this { return this.bar(); } // OK (don't cascade errors)\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class A {\n  foo() {\n    return this;\n  } // return of foo is not annotated to get around\n  // substituting this below\n  bar(): this {\n    return new A().foo();\n  } // same as returning : A, so error\n  qux(): this {\n    return this.bar();\n  } // OK (don't cascade errors)\n}");
}
#[test]
fn test_statics_js_format_1_de0147ad() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// supporting \\`this\\` type in statics\n\nclass A {\n  static make(): this { // factory method, whose return type \\`this\\` (still)\n                        // describes instances of A or subclasses of A: the\n                        // meaning of the \\`this\\` type is not changed simply by\n                        // switching into a static context\n    return new this; // but in a static context, the value \\`this\\` is bound to\n                     // the class, instead of instances of the class\n  }\n}\nclass B extends A { } // inherits statics method too, with \\`this\\` bound to the class\n\n(A.make(): A); // OK\n(B.make(): B); // OK\n(B.make(): A); // OK\n(A.make(): B); // error") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// supporting \\`this\\` type in statics\n\nclass A {\n  static make(): this {\n    // factory method, whose return type \\`this\\` (still)\n    // describes instances of A or subclasses of A: the\n    // meaning of the \\`this\\` type is not changed simply by\n    // switching into a static context\n    return new this(); // but in a static context, the value \\`this\\` is bound to\n    // the class, instead of instances of the class\n  }\n}\nclass B extends A {} // inherits statics method too, with \\`this\\` bound to the class\n\n(A.make(): A); // OK\n(B.make(): B); // OK\n(B.make(): A); // OK\n(A.make(): B); // error");
}
#[test]
fn test_test_js_format_1_36071817() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// Examples without \\`this\\` types (compare with examples below)\n\nclass Base {\n  foo() { return this; }\n  qux() { return new Base; }\n\n  bar() { return this; }\n  bar_caller() { return this.bar(); }\n}\n\nclass Inherit extends Base { }\n\nclass Override extends Base {\n  foo() { return this; } // OK\n  qux() { return this; } // OK, too\n\n  bar() { return new Override; } // OK (cf. error below)\n}\n\nclass InheritOverride extends Override { }\n\n(new Inherit().foo(): Base);\n(new Inherit().foo(): Inherit); // error (cf. OK below)\n((new Inherit(): Base).foo(): Base);\n(new Override().foo(): Base);\n(new Override().foo(): Override); // OK\n((new Override(): Base).foo(): Base);\n\n(new InheritOverride().bar_caller(): InheritOverride); // error\n                                                       // blame flips below\n\n// Examples with \\`this\\` types (compare with examples above)\n\nclass Base2 {\n  foo(): this { return this; }\n  qux(): Base2 { return new Base2; }\n\n  bar(): this { return this; }\n  bar_caller(): this { return this.bar(); }\n\n  corge(that: this) { }\n  grault(that: Base2) { }\n}\n\nclass Inherit2 extends Base2 { }\n\nclass Override2 extends Base2 {\n  foo(): this { return this; } // OK\n  qux(): this { return this; } // OK, too\n\n  bar(): Override2 { return new Override2; } // error (cf. OK above)\n                                             // see exploit below\n\n  corge(that: this) { } // error\n                        // see exploit below\n  grault(that: this) { } // error, too\n}\n\nclass InheritOverride2 extends Override2 { }\n\n(new Inherit2().foo(): Base2);\n(new Inherit2().foo(): Inherit2); // OK (cf. error above)\n((new Inherit2(): Base2).foo(): Base2);\n(new Override2().foo(): Base2);\n(new Override2().foo(): Override2); // OK\n((new Override2(): Base2).foo(): Base2);\n\n(new InheritOverride2().bar_caller(): InheritOverride2); // exploits error above\n\n(new Override2(): Base2).corge(new Base2()); // exploits error above") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// Examples without \\`this\\` types (compare with examples below)\n\nclass Base {\n  foo() {\n    return this;\n  }\n  qux() {\n    return new Base();\n  }\n\n  bar() {\n    return this;\n  }\n  bar_caller() {\n    return this.bar();\n  }\n}\n\nclass Inherit extends Base {}\n\nclass Override extends Base {\n  foo() {\n    return this;\n  } // OK\n  qux() {\n    return this;\n  } // OK, too\n\n  bar() {\n    return new Override();\n  } // OK (cf. error below)\n}\n\nclass InheritOverride extends Override {}\n\n(new Inherit().foo(): Base);\n(new Inherit().foo(): Inherit); // error (cf. OK below)\n((new Inherit(): Base).foo(): Base);\n(new Override().foo(): Base);\n(new Override().foo(): Override); // OK\n((new Override(): Base).foo(): Base);\n\n(new InheritOverride().bar_caller(): InheritOverride); // error\n// blame flips below\n\n// Examples with \\`this\\` types (compare with examples above)\n\nclass Base2 {\n  foo(): this {\n    return this;\n  }\n  qux(): Base2 {\n    return new Base2();\n  }\n\n  bar(): this {\n    return this;\n  }\n  bar_caller(): this {\n    return this.bar();\n  }\n\n  corge(that: this) {}\n  grault(that: Base2) {}\n}\n\nclass Inherit2 extends Base2 {}\n\nclass Override2 extends Base2 {\n  foo(): this {\n    return this;\n  } // OK\n  qux(): this {\n    return this;\n  } // OK, too\n\n  bar(): Override2 {\n    return new Override2();\n  } // error (cf. OK above)\n  // see exploit below\n\n  corge(that: this) {} // error\n  // see exploit below\n  grault(that: this) {} // error, too\n}\n\nclass InheritOverride2 extends Override2 {}\n\n(new Inherit2().foo(): Base2);\n(new Inherit2().foo(): Inherit2); // OK (cf. error above)\n((new Inherit2(): Base2).foo(): Base2);\n(new Override2().foo(): Base2);\n(new Override2().foo(): Override2); // OK\n((new Override2(): Base2).foo(): Base2);\n\n(new InheritOverride2().bar_caller(): InheritOverride2); // exploits error above\n\n(new Override2(): Base2).corge(new Base2()); // exploits error above");
}
