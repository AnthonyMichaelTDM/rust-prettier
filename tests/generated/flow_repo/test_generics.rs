#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_generics_js_format_1_9bbf7f92() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class C<X> {\n  x:X;\n  constructor(x:X) { this.x = x; }\n  get():X { return this.x; }\n}\n\nclass D<T> {\n  x:T;\n  m<S>(z:S,u:T,v):S {\n    this.x = u;\n    v.u = u;\n    return z;\n  }\n}\n\nvar d = new D();\nvar o = {};\nvar b = d.m(true,0,o);\nvar s:string = d.x;\nvar n:number = o.u;\n\nclass E<X> extends C<X> {\n    //x:X;\n    set(x:X):X { /*return x;*/ this.x = x; return /*this.x; */this.get(); }\n}\n\nvar e = new E(); // error: too few arguments to inherited constructor\nvar x:string = e.set(0);\n\nclass F<X> { }\nclass G<Y> extends F<Array<Y>> {}\nclass H<Z> extends G<Array<Z>> {\n    x:Z;\n    foo(x:Z) { this.x = x; }\n}\n\nvar h1 = new H();\nh1.foo([\"...\"]);\nvar h2:F<Array<Array<Array<number>>>> = h1;\n\nvar obj : Object<string, string> = {} // error, arity 0\nvar fn : Function<string> = function() { return 'foo'; } // error, arity 0") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class C<X> {\n  x: X;\n  constructor(x: X) {\n    this.x = x;\n  }\n  get(): X {\n    return this.x;\n  }\n}\n\nclass D<T> {\n  x: T;\n  m<S>(z: S, u: T, v): S {\n    this.x = u;\n    v.u = u;\n    return z;\n  }\n}\n\nvar d = new D();\nvar o = {};\nvar b = d.m(true, 0, o);\nvar s: string = d.x;\nvar n: number = o.u;\n\nclass E<X> extends C<X> {\n  //x:X;\n  set(x: X): X {\n    /*return x;*/ this.x = x;\n    return /*this.x; */ this.get();\n  }\n}\n\nvar e = new E(); // error: too few arguments to inherited constructor\nvar x: string = e.set(0);\n\nclass F<X> {}\nclass G<Y> extends F<Array<Y>> {}\nclass H<Z> extends G<Array<Z>> {\n  x: Z;\n  foo(x: Z) {\n    this.x = x;\n  }\n}\n\nvar h1 = new H();\nh1.foo([\"...\"]);\nvar h2: F<Array<Array<Array<number>>>> = h1;\n\nvar obj: Object<string, string> = {}; // error, arity 0\nvar fn: Function<string> = function () {\n  return \"foo\";\n}; // error, arity 0");
}
