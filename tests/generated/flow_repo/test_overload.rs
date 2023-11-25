use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_overload_js_format_1_26b1bfd0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * tests of overload selection\n *\n * @flow\n */\n\nvar x1: number = \"\".match(0)[0];\nvar x2: number = \"\".match(/pattern/)[0];\nvar x3: string = \"\".replace(/pattern/,\"...\");\nvar x4: number = \"\".split(/pattern/)[0];\n\ndeclare class C {\n    foo(x:number): number;\n    foo(x:string): string;\n\n    bar(x: { a: number }): number;\n    bar(x: { a: string }): string;\n}\n\nvar a = new C();\n\na.foo(0); // ok\na.foo(\"hey\"); // ok\na.foo(true); // error, function cannot be called on intersection type\n\na.bar({ a: 0 }); // ok\na.bar({ a: \"hey\" }); // ok\na.bar({ a: true }); // error, function cannot be called on intersection type\n\ndeclare var x: { a: boolean; } & { b: string };\n\na.bar(x); // error with nested intersection info (outer for bar, inner for x)\n\n/********** tests **************\ninterface Dummy<T> {\n    dumb(foo: (x:number) => number):number;\n    dumb(foo: (x:string) => string):string;\n\n    dumber<U>(bar: (x:T) => Array<U>):U;\n    dumber<U>(bar: (x:T) => U):Array<U>;\n}\n\nfunction foo(x:string):string { return x; }\nvar y:number = new Dummy().dumb(foo);\n\nfunction bar1(x:number):Array<string> { return []; }\nvar z1:number = new Dummy().dumber(bar1);\n\nfunction bar2(x:number):string { return \"...\"; }\nvar z2:Array<string> = new Dummy().dumber(bar2);\n*/") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * tests of overload selection\n *\n * @flow\n */\n\nvar x1: number = \"\".match(0)[0];\nvar x2: number = \"\".match(/pattern/)[0];\nvar x3: string = \"\".replace(/pattern/, \"...\");\nvar x4: number = \"\".split(/pattern/)[0];\n\ndeclare class C {\n  foo(x: number): number;\n  foo(x: string): string;\n\n  bar(x: { a: number }): number;\n  bar(x: { a: string }): string;\n}\n\nvar a = new C();\n\na.foo(0); // ok\na.foo(\"hey\"); // ok\na.foo(true); // error, function cannot be called on intersection type\n\na.bar({ a: 0 }); // ok\na.bar({ a: \"hey\" }); // ok\na.bar({ a: true }); // error, function cannot be called on intersection type\n\ndeclare var x: { a: boolean } & { b: string };\n\na.bar(x); // error with nested intersection info (outer for bar, inner for x)\n\n/********** tests **************\ninterface Dummy<T> {\n    dumb(foo: (x:number) => number):number;\n    dumb(foo: (x:string) => string):string;\n\n    dumber<U>(bar: (x:T) => Array<U>):U;\n    dumber<U>(bar: (x:T) => U):Array<U>;\n}\n\nfunction foo(x:string):string { return x; }\nvar y:number = new Dummy().dumb(foo);\n\nfunction bar1(x:number):Array<string> { return []; }\nvar z1:number = new Dummy().dumber(bar1);\n\nfunction bar2(x:number):string { return \"...\"; }\nvar z2:Array<string> = new Dummy().dumber(bar2);\n*/");
}
#[test]
fn test_test_js_format_1_34f00059() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function foo() {\n  var output = new FakeUint8Array();\n  output.set(new FakeUint8Array(), 0); // matches one of the overloads of set\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function foo() {\n  var output = new FakeUint8Array();\n  output.set(new FakeUint8Array(), 0); // matches one of the overloads of set\n}");
}
#[test]
fn test_test_2_js_format_1_eebe52d3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare class Foo {\n  bar(x: 'hmm'): number;\n  bar(x: string): string;\n}\nvar foo = new Foo;\n(foo.bar('hmm'): number); // OK\n(foo.bar('hmmm'): number); // error") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare class Foo {\n  bar(x: \"hmm\"): number;\n  bar(x: string): string;\n}\nvar foo = new Foo();\n(foo.bar(\"hmm\"): number); // OK\n(foo.bar(\"hmmm\"): number); // error");
}
#[test]
fn test_test_3_js_format_1_52e8e9d2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// passing a union-like thing into an overload is ok\n// if overload handles each branch of union-like thing\n\n// unions\ndeclare function f(x: string): void;\ndeclare function f(x: number): void;\ndeclare var x_f: string | number;\nf(x_f); // ok\n\n// maybe\ndeclare function g(x: null): void;\ndeclare function g(x: void): void;\ndeclare function g(x: string): void;\ndeclare var x_g: ?string;\ng(x_g); // ok\n\n// optional\ndeclare function h(x: void): void;\ndeclare function h(x: string): void;\ndeclare var x_h: {p?: string};\nh(x_h.p); // ok") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// passing a union-like thing into an overload is ok\n// if overload handles each branch of union-like thing\n\n// unions\ndeclare function f(x: string): void;\ndeclare function f(x: number): void;\ndeclare var x_f: string | number;\nf(x_f); // ok\n\n// maybe\ndeclare function g(x: null): void;\ndeclare function g(x: void): void;\ndeclare function g(x: string): void;\ndeclare var x_g: ?string;\ng(x_g); // ok\n\n// optional\ndeclare function h(x: void): void;\ndeclare function h(x: string): void;\ndeclare var x_h: { p?: string };\nh(x_h.p); // ok");
}
#[test]
fn test_union_js_format_1_3777ad5c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function foo<U> (x: $Either<Array<U>,U>): Array<U> { return []; }\n\nvar x1:number = foo(0)[0];\nvar x2:string = foo([\"\"])[0];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function foo<U>(x: $Either<Array<U>, U>): Array<U> {\n  return [];\n}\n\nvar x1: number = foo(0)[0];\nvar x2: string = foo([\"\"])[0];");
}
