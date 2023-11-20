#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_class_js_format_1_5ee191cb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class C<T> {\n  a<A>(x:T, a:A) {\n    this.b(x); // ok\n    this.b(a); // error: A ~> incompatible instance of T\n  }\n\n  b(x:T) {}\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class C<T> {\n  a<A>(x: T, a: A) {\n    this.b(x); // ok\n    this.b(a); // error: A ~> incompatible instance of T\n  }\n\n  b(x: T) {}\n}");
}
#[test]
fn test_default_params_js_format_1_4e0201fc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function f<T>(a:T) {\n  function g<U>(b:U, c:T = a) {\n    function h(d:U = b) {}\n    h(); // ok\n    h(b); // ok\n    h(c); // err, T ~> U\n  }\n  g(0); // ok\n  g(0,a); // ok\n  g(0,0); // error: number ~> T\n}\nf(0); // ok") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function f<T>(a: T) {\n  function g<U>(b: U, c: T = a) {\n    function h(d: U = b) {}\n    h(); // ok\n    h(b); // ok\n    h(c); // err, T ~> U\n  }\n  g(0); // ok\n  g(0, a); // ok\n  g(0, 0); // error: number ~> T\n}\nf(0); // ok");
}
#[test]
fn test_method_shadow_js_format_1_a213b05c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// Ensure method type params properly shadow outer type params. Subclass ensures\n// the generated insttype has the correct tvars. Should behave the same for\n// classes, interfaces, and declared classes.\n\nclass A<T> {\n  x:T;\n  constructor(x:T) { this.x = x }\n  m<T>(x:T):A<T> { return new A(x) }\n}\n\nclass B<T> extends A<T> {\n  m<T>(x:T):B<T> { return new B(x) }\n}\n\ninterface C<T> {\n  m<T>(x:T):C<T>;\n}\n\ninterface D<T> extends C<T> {\n  m<T>(x:T):D<T>;\n}\n\ndeclare class E<T> {\n  m<T>(x:T):E<T>;\n}\n\ndeclare class F<T> extends E<T> {\n  m<T>(x:T):F<T>;\n}\n\n\n// Bounds can refer to parent type params (until they are shadowed).\n\nclass G<T> {\n  x:T;\n  constructor(x:T) { this.x = x }\n  m<T:T>(x:T):G<T> { return new G(x) } // T-as-bound is G's T\n}\n\ndeclare var g: G<number|string>;\ng.m(0); // ok\ng.m(true); // err, bool ~> number|string\n(g.m(\"\"): G<number>); // err, string ~> number\n\n\n// Shadow bounds incompatible with parent\n\nclass H<T> {\n    x:T;\n    m<T>(x:T) {\n        this.x = x; // err, m's T != H's T\n    }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// Ensure method type params properly shadow outer type params. Subclass ensures\n// the generated insttype has the correct tvars. Should behave the same for\n// classes, interfaces, and declared classes.\n\nclass A<T> {\n  x: T;\n  constructor(x: T) {\n    this.x = x;\n  }\n  m<T>(x: T): A<T> {\n    return new A(x);\n  }\n}\n\nclass B<T> extends A<T> {\n  m<T>(x: T): B<T> {\n    return new B(x);\n  }\n}\n\ninterface C<T> {\n  m<T>(x: T): C<T>;\n}\n\ninterface D<T> extends C<T> {\n  m<T>(x: T): D<T>;\n}\n\ndeclare class E<T> {\n  m<T>(x: T): E<T>;\n}\n\ndeclare class F<T> extends E<T> {\n  m<T>(x: T): F<T>;\n}\n\n// Bounds can refer to parent type params (until they are shadowed).\n\nclass G<T> {\n  x: T;\n  constructor(x: T) {\n    this.x = x;\n  }\n  m<T: T>(x: T): G<T> {\n    return new G(x);\n  } // T-as-bound is G's T\n}\n\ndeclare var g: G<number | string>;\ng.m(0); // ok\ng.m(true); // err, bool ~> number|string\n(g.m(\"\"): G<number>); // err, string ~> number\n\n// Shadow bounds incompatible with parent\n\nclass H<T> {\n  x: T;\n  m<T>(x: T) {\n    this.x = x; // err, m's T != H's T\n  }\n}");
}
