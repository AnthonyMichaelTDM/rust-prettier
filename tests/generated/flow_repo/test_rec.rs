#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_issue_598_js_format_1_3673ff8d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\ntype F<A> = { foo<B>(x: A): F<B> }\ndeclare function foo(x: any): F<any>;\n({ foo }: F<any>);\n\nfunction bar(y: F<number>): F<string> { return y; }\nfunction bar1<X>(y: F<X>): F<any> { return y; }\nfunction bar2<X>(y: F<any>): F<X> { return y; }\n\ntype Functor<A> = {\n  map<B>(f: (val: A) => B): Functor<B>\n}\n\nfunction identity<A>(val: A): Functor<A> {\n  return {\n    map<B>(f: (_: typeof val) => B): Functor<B> { return identity(f(val)) }\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\ntype F<A> = { foo<B>(x: A): F<B> };\ndeclare function foo(x: any): F<any>;\n({ foo }: F<any>);\n\nfunction bar(y: F<number>): F<string> {\n  return y;\n}\nfunction bar1<X>(y: F<X>): F<any> {\n  return y;\n}\nfunction bar2<X>(y: F<any>): F<X> {\n  return y;\n}\n\ntype Functor<A> = {\n  map<B>(f: (val: A) => B): Functor<B>,\n};\n\nfunction identity<A>(val: A): Functor<A> {\n  return {\n    map<B>(f: (_: typeof val) => B): Functor<B> {\n      return identity(f(val));\n    },\n  };\n}");
}
#[test]
fn test_issue_1228_js_format_1_7bf8b861() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\ntype Task <error, value>\n  = { chain<tagged>(next:(input:value) => Task<error, tagged>):\n          Task<error, tagged>\n    }\n\nfunction id(x: Task<any,any>): Task<any,any> { return x; }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\ntype Task<error, value> = {\n  chain<tagged>(\n    next: (input: value) => Task<error, tagged>,\n  ): Task<error, tagged>,\n};\n\nfunction id(x: Task<any, any>): Task<any, any> {\n  return x;\n}");
}
#[test]
fn test_test_js_format_1_2e518318() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class P<X> { x: X; } // this is like Promise\n\ntype Pstar<X> = X | Pstar<P<X>>; // this is like Promise*\n\nvar p: P<number> = new P;\n(p.x: string); // error\n\nvar pstar: Pstar<number> = 0; // OK\n(pstar: number); // error, but limit potentially unbounded number of errors!\n                 // e.g., P<number> ~/~ number, P<P<number>> ~/~ number, ...\n\npstar = p; // OK\n(pstar.x: string); // error\n\npstar = (new P: P<P<number>>); // OK\n(pstar.x: string); // error") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class P<X> {\n  x: X;\n} // this is like Promise\n\ntype Pstar<X> = X | Pstar<P<X>>; // this is like Promise*\n\nvar p: P<number> = new P();\n(p.x: string); // error\n\nvar pstar: Pstar<number> = 0; // OK\n(pstar: number); // error, but limit potentially unbounded number of errors!\n// e.g., P<number> ~/~ number, P<P<number>> ~/~ number, ...\n\npstar = p; // OK\n(pstar.x: string); // error\n\npstar = (new P(): P<P<number>>); // OK\n(pstar.x: string); // error");
}
#[test]
fn test_test_2_js_format_1_e1150d7f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var a = []; // Array<X> ~> a\nfunction bar() {\n  a = a.concat([]); // terminate despite expanding types:\n  // a ~> .concat(Array<Y>)\n  // Array<X> ~> .concat(Array<Y>)\n  // Array<X|Y> ~> a\n  // Array<X|Y> ~> .concat(Array<Y>)\n  // Array<X|Y|Y> ~> a\n};\n\nclass A<X> {\n  x: A<A<X>>;\n}\nvar a_ = new A;\nfunction foo0() {\n  a_ = a_.x; // terminate despite expanding types\n}\n\ntype T<X> = { y: S<X> };\ntype S<X> = T<S<X>>;\nfunction foo1(b: S<*>) {\n  b = b.y; // terminate despite expanding types, OK\n  // S<*> = { y: S<S<*>> }\n  // Both S<S<*>> and S<*> expand to { y: { y: ... }}.\n}\n\nclass D<X> { }\nclass B<X> extends D<X> { }\nclass C<X> extends B<X> { }\n((new C: C<number>): D<string>) // error: number ~/~ string") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var a = []; // Array<X> ~> a\nfunction bar() {\n  a = a.concat([]); // terminate despite expanding types:\n  // a ~> .concat(Array<Y>)\n  // Array<X> ~> .concat(Array<Y>)\n  // Array<X|Y> ~> a\n  // Array<X|Y> ~> .concat(Array<Y>)\n  // Array<X|Y|Y> ~> a\n}\n\nclass A<X> {\n  x: A<A<X>>;\n}\nvar a_ = new A();\nfunction foo0() {\n  a_ = a_.x; // terminate despite expanding types\n}\n\ntype T<X> = { y: S<X> };\ntype S<X> = T<S<X>>;\nfunction foo1(b: S<*>) {\n  b = b.y; // terminate despite expanding types, OK\n  // S<*> = { y: S<S<*>> }\n  // Both S<S<*>> and S<*> expand to { y: { y: ... }}.\n}\n\nclass D<X> {}\nclass B<X> extends D<X> {}\nclass C<X> extends B<X> {}\n((new C(): C<number>): D<string>); // error: number ~/~ string");
}
#[test]
fn test_test_3_js_format_1_1d8b52a7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type I<X> = () => I<I<X>>;\ntype J<X> = () => J<J<X>>;\n\nfunction foo(x: I<number>): J<number> {\n  return x; // terminate despite expanding types, OK\n  // I<number> and J<number> both expand to () => () => ...\n}\n\ntype Q<X> = { x: X; }\ntype P<X> = () => Q<P<X>>;\n\nfunction bar(x: P<number>): () => P<number> {\n  return x; // terminate despite expanding types, error\n  // P<number> = () => { x: P<number> }\n  // () => P<number> = () => () => { x: P<number> }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type I<X> = () => I<I<X>>;\ntype J<X> = () => J<J<X>>;\n\nfunction foo(x: I<number>): J<number> {\n  return x; // terminate despite expanding types, OK\n  // I<number> and J<number> both expand to () => () => ...\n}\n\ntype Q<X> = { x: X };\ntype P<X> = () => Q<P<X>>;\n\nfunction bar(x: P<number>): () => P<number> {\n  return x; // terminate despite expanding types, error\n  // P<number> = () => { x: P<number> }\n  // () => P<number> = () => () => { x: P<number> }\n}");
}
#[test]
fn test_test_4_js_format_1_7d7f810f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("type T = T // cycle in type alias should not cause non-termination");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "type T = T; // cycle in type alias should not cause non-termination"
    );
}
#[test]
fn test_test_5_js_format_1_88b185d4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\ntype NestedArray<T> = Array<T | NestedArray<T>>;\n\nfunction flatten<T>(arrArg: NestedArray<T>) {\n  let arr = arrArg;\n  while (true) {\n    arr = Array.prototype.concat.apply([], arr);\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\ntype NestedArray<T> = Array<T | NestedArray<T>>;\n\nfunction flatten<T>(arrArg: NestedArray<T>) {\n  let arr = arrArg;\n  while (true) {\n    arr = Array.prototype.concat.apply([], arr);\n  }\n}");
}
