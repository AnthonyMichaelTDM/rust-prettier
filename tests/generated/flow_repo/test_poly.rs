#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_annot_js_format_1_a1e2659e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A<X> { }\nnew A; // OK, implicitly inferred type args\nclass B extends A { } // OK, same as above\n\nfunction foo(b): A<any> { // ok but unsafe, caller may assume any type arg\n  return b ? (new A: A<number>): (new A: A<string>);\n}\n\nfunction bar(): A<*> { // error, * can't be {} and {x: string} at the same time\n  return (new A: A<{}>) || (new A: A<{x: string}>);\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class A<X> {}\nnew A(); // OK, implicitly inferred type args\nclass B extends A {} // OK, same as above\n\nfunction foo(b): A<any> {\n  // ok but unsafe, caller may assume any type arg\n  return b ? (new A(): A<number>) : (new A(): A<string>);\n}\n\nfunction bar(): A<*> {\n  // error, * can't be {} and {x: string} at the same time\n  return (new A(): A<{}>) || (new A(): A<{ x: string }>);\n}");
}
#[test]
fn test_implicit_bounded_instantiation_js_format_1_c426c54d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nclass Base {}\nclass Middle extends Base {}\nclass Child extends Middle {}\n\nclass C<T: Middle> {\n  meth(a: T): T {\n    return a;\n  }\n}\n\n// T is implicitly (bounded by) Middle in constructor call if not provided.\n// Explicit type arg is required in annotation - here a wildcard captures it.\nvar a: C<*> = new C();\n\na.meth(new Middle());\na.meth(new Child());\na.meth(42); // Error: number ~> Middle\na.meth(new Base()); // Error: Base ~> Middle") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nclass Base {}\nclass Middle extends Base {}\nclass Child extends Middle {}\n\nclass C<T: Middle> {\n  meth(a: T): T {\n    return a;\n  }\n}\n\n// T is implicitly (bounded by) Middle in constructor call if not provided.\n// Explicit type arg is required in annotation - here a wildcard captures it.\nvar a: C<*> = new C();\n\na.meth(new Middle());\na.meth(new Child());\na.meth(42); // Error: number ~> Middle\na.meth(new Base()); // Error: Base ~> Middle");
}
#[test]
fn test_issue_1029_js_format_1_0629efc3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n// naive unification causes combinatorial explosion here,\n// effectively hangs\n\ndeclare type Box<T> = {\n  map1<U>(f: (x: T) => U): Box<U>;\n  map2<U>(f: (x: T) => U): Box<U>;\n  map3<U>(f: (x: T) => U): Box<U>;\n  map4<U>(f: (x: T) => U): Box<U>;\n  map5<U>(f: (x: T) => U): Box<U>;\n}\n\ndeclare var bool: Box<boolean>;\n\ndeclare function unbox<A>(box: Box<A>): A\n\nunbox(bool);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n// naive unification causes combinatorial explosion here,\n// effectively hangs\n\ndeclare type Box<T> = {\n  map1<U>(f: (x: T) => U): Box<U>,\n  map2<U>(f: (x: T) => U): Box<U>,\n  map3<U>(f: (x: T) => U): Box<U>,\n  map4<U>(f: (x: T) => U): Box<U>,\n  map5<U>(f: (x: T) => U): Box<U>,\n};\n\ndeclare var bool: Box<boolean>;\n\ndeclare function unbox<A>(box: Box<A>): A;\n\nunbox(bool);");
}
#[test]
fn test_poly_js_format_1_c959c866() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class Foo<T> {\n    x:T;\n    constructor(x:T) { this.x = x; }\n}\n\nfunction bar<S>(foo:Foo<S>,y:S):Foo<S> { return new Foo(y); }\n\nvar P = {\n    bar: bar\n}\n\ndeclare var Q: {\n    bar<S>(foo:Foo<S>,y:S):Foo<S>;\n}\n\nvar foo = new Foo(0);\nvar x:string = foo.x;\nvar z:Foo<number> = Q.bar(foo,\"\");") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class Foo<T> {\n  x: T;\n  constructor(x: T) {\n    this.x = x;\n  }\n}\n\nfunction bar<S>(foo: Foo<S>, y: S): Foo<S> {\n  return new Foo(y);\n}\n\nvar P = {\n  bar: bar,\n};\n\ndeclare var Q: {\n  bar<S>(foo: Foo<S>, y: S): Foo<S>,\n};\n\nvar foo = new Foo(0);\nvar x: string = foo.x;\nvar z: Foo<number> = Q.bar(foo, \"\");");
}
#[test]
fn test_test_js_format_1_a5fe3efd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class C {\n  foo<X>(x: X): X { return x; }\n  foo_<X: number>(x: X): number { return x; }\n  bar<X>(x: X): X { return x; }\n  qux(x: number): number { return x; }\n}\nclass D extends C {\n  foo(x: number): number { return x; } // error (specialization, see below)\n  foo_(x: number): number { return x; } // OK, but only because the overridden foo accepts no more than number and returns exactly number\n  bar<X>(x: X): X { return x; } // OK\n  qux<X>(x: X): X { return x; } // OK (generalization)\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class C {\n  foo<X>(x: X): X {\n    return x;\n  }\n  foo_<X: number>(x: X): number {\n    return x;\n  }\n  bar<X>(x: X): X {\n    return x;\n  }\n  qux(x: number): number {\n    return x;\n  }\n}\nclass D extends C {\n  foo(x: number): number {\n    return x;\n  } // error (specialization, see below)\n  foo_(x: number): number {\n    return x;\n  } // OK, but only because the overridden foo accepts no more than number and returns exactly number\n  bar<X>(x: X): X {\n    return x;\n  } // OK\n  qux<X>(x: X): X {\n    return x;\n  } // OK (generalization)\n}");
}
