use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_dba31140() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("class A {\n  foo(x:number):void { }\n}\n\nmodule.exports = A;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "class A {\n  foo(x: number): void {}\n}\n\nmodule.exports = A;"
    );
}
#[test]
fn test_b_js_format_1_ba6d0b53() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var A = require('./A');\n\nclass B extends A { }\n\nlet b = new B();\n(b.foo: number); // error, number !~> function\n\nmodule.exports = B;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var A = require(\"./A\");\n\nclass B extends A {}\n\nlet b = new B();\n(b.foo: number); // error, number !~> function\n\nmodule.exports = B;");
}
#[test]
fn test_c_js_format_1_58c3ebc1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var B = require('./B');\n\nclass C extends B {\n  foo(x:string):void { }\n}\n\nlet c = new C();\n(c.foo: number); // error, number !~> function\n\nmodule.exports = C;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var B = require(\"./B\");\n\nclass C extends B {\n  foo(x: string): void {}\n}\n\nlet c = new C();\n(c.foo: number); // error, number !~> function\n\nmodule.exports = C;");
}
#[test]
fn test_d_js_format_1_8ae4c59d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("class D { }\nclass E { }\nnew E().x");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "class D {}\nclass E {}\nnew E().x;");
}
#[test]
fn test_class_shapes_js_format_1_e13d89e3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\ntype Foo = {\n  a: string;    // exists in TestClass\n  b: string;    // doesn't exist\n  c?: ?string;  // exists in TestClass, optional\n  d?: number;   // doesn't exist\n}\n\nclass TestClass {\n  a: string;\n  c: ?string;\n}\n\nvar x = new TestClass();\n\nx.a; // ok\nx.b; // error, TestClass has no b\nx.c; // ok\nx.d; // error, TestClass has no d\n\nvar y : Foo = x;\ny.b; // error, doesn't exist in TestClass\ny.d; // ok, it's optional\n\nclass Test2Superclass {\n  a: number;  // conflicts with cast to Foo\n  c: ?number; // conflicts with cast to Foo\n}\nclass Test2Class extends Test2Superclass {\n  b: number;  // conflicts with cast to Foo\n}\n\nvar z = new Test2Class();\nvar w : Foo = z;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\ntype Foo = {\n  a: string, // exists in TestClass\n  b: string, // doesn't exist\n  c?: ?string, // exists in TestClass, optional\n  d?: number, // doesn't exist\n};\n\nclass TestClass {\n  a: string;\n  c: ?string;\n}\n\nvar x = new TestClass();\n\nx.a; // ok\nx.b; // error, TestClass has no b\nx.c; // ok\nx.d; // error, TestClass has no d\n\nvar y: Foo = x;\ny.b; // error, doesn't exist in TestClass\ny.d; // ok, it's optional\n\nclass Test2Superclass {\n  a: number; // conflicts with cast to Foo\n  c: ?number; // conflicts with cast to Foo\n}\nclass Test2Class extends Test2Superclass {\n  b: number; // conflicts with cast to Foo\n}\n\nvar z = new Test2Class();\nvar w: Foo = z;");
}
#[test]
fn test_expr_js_format_1_8c166f51() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var Bar = class Foo {\n  static factory(): Foo { // OK: Foo is a type in this scope\n    return new Foo()      // OK: Foo is a runtime binding in this scope\n  }\n};\n\nvar bar1: Bar = new Bar() // OK\nvar bar2: Bar = Bar.factory() // OK\n\n// NB: Don't write expected errors using Foo to avoid error collapse hiding an\n// unexpected failure in the above code.\n\nvar B = class Baz { }\nvar b = new Baz(); // error: Baz is not a runtime binding in this scope\n\nvar C = class Qux { }\nvar c: Qux = new C(); // error: Qux is not a type in this scope\n\n// OK: anon classes create no binding, but can be bound manually\nvar Anon = class { }\nvar anon: Anon = new Anon();\n\nclass Alias { }\nvar _Alias = class Alias {\n  static factory(): Alias {\n    return new Alias();\n  }\n}\nvar alias1: Alias = new _Alias(); // error: bad pun\nvar alias2: Alias = _Alias.factory(); // error: bad pun") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var Bar = class Foo {\n  static factory(): Foo {\n    // OK: Foo is a type in this scope\n    return new Foo(); // OK: Foo is a runtime binding in this scope\n  }\n};\n\nvar bar1: Bar = new Bar(); // OK\nvar bar2: Bar = Bar.factory(); // OK\n\n// NB: Don't write expected errors using Foo to avoid error collapse hiding an\n// unexpected failure in the above code.\n\nvar B = class Baz {};\nvar b = new Baz(); // error: Baz is not a runtime binding in this scope\n\nvar C = class Qux {};\nvar c: Qux = new C(); // error: Qux is not a type in this scope\n\n// OK: anon classes create no binding, but can be bound manually\nvar Anon = class {};\nvar anon: Anon = new Anon();\n\nclass Alias {}\nvar _Alias = class Alias {\n  static factory(): Alias {\n    return new Alias();\n  }\n};\nvar alias1: Alias = new _Alias(); // error: bad pun\nvar alias2: Alias = _Alias.factory(); // error: bad pun");
}
#[test]
fn test_extends_any_js_format_1_876b49c0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const Base: any = class {}\nclass Derived1 extends Base {}\nclass Derived2 extends Derived1 {\n    m() {}\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const Base: any = class {};\nclass Derived1 extends Base {}\nclass Derived2 extends Derived1 {\n  m() {}\n}");
}
#[test]
fn test_loc_js_format_1_5db3ba46() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("/* @flow */\n\ntype Foo = number\n\nclass Foo {} // error, shadows type Foo");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/* @flow */\n\ntype Foo = number;\n\nclass Foo {} // error, shadows type Foo"
    );
}
#[test]
fn test_statics_js_format_1_898cba7a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nclass C {\n  static p: string;\n}\nC.p = \"hi\";\n\n// Class static fields are compatible with object types\n(C: {p:string}); // ok\n(C: {p:number}); // errors, string ~> number & vice versa (unify)\n\ndeclare var o: {p:number};\n(o: Class<C>); // error, object type incompatible with class type") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nclass C {\n  static p: string;\n}\nC.p = \"hi\";\n\n// Class static fields are compatible with object types\n(C: { p: string }); // ok\n(C: { p: number }); // errors, string ~> number & vice versa (unify)\n\ndeclare var o: { p: number };\n(o: Class<C>); // error, object type incompatible with class type");
}
