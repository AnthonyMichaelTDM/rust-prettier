#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_annot_js_format_1_bf9e2592() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function foo(str:string, i:number):string {\n  return str;\n}\nvar bar: (str:number, i:number)=> string = foo;\n\nvar qux = function(str:string, i:number):number { return foo(str,i); }\n\nvar obj: {str:string; i:number; j:boolean} = {str: \"...\", i: \"...\", k: false};\n\nvar arr: Array<number> = [1,2,\"...\"];\n\n// array sugar\nvar array: number[] = [1,2,\"...\"];\n\nvar matrix: number[][] = [[1,2],[3,4]];\nvar matrix_parens: (number[])[] = matrix;\n\nvar nullable_array: ?number[] = null;\nvar nullable_array_parens: ?(number[]) = nullable_array;\n\nvar array_of_nullable: (?number)[] = [null, 3];\n\nvar array_of_tuple: [number, string][] = [[0, \"foo\"], [1, \"bar\"]];\nvar array_of_tuple_parens: ([number, string])[] = array_of_tuple;\n\ntype ObjType = { 'bar-foo': string; 'foo-bar': number; };\nvar test_obj: ObjType = { 'bar-foo': '23' };\n\n// param type annos are strict UBs like var type annos\nfunction param_anno(n:number):void {\n  n = \"hey\"; // error\n}\n\n// another error on param UB, more typical of www (mis)use-cases\n// this one cribbed from API.atlas.js\nfunction param_anno2(\n    batchRequests: Array<{method: string; path: string; params: ?Object}>,\n  ): void {\n\n    // error below, since we're assigning elements to batchRequests\n    // which lack a path property.\n    // just assign result to new var instead of reassigning to param.\n\n    // Transform the requests to the format the Graph API expects.\n    batchRequests = batchRequests.map((request) => {\n      return {\n        method: request.method,\n        params: request.params,\n        relative_url: request.path,\n      };\n    });\n    // ...\n  }\n\nvar toz : null = 3;\n\nvar zer : null = null;\n\nfunction foobar(n : ?number) : number | null | void { return n; }\nfunction barfoo(n : number | null | void) : ?number { return n; }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function foo(str: string, i: number): string {\n  return str;\n}\nvar bar: (str: number, i: number) => string = foo;\n\nvar qux = function (str: string, i: number): number {\n  return foo(str, i);\n};\n\nvar obj: { str: string, i: number, j: boolean } = {\n  str: \"...\",\n  i: \"...\",\n  k: false,\n};\n\nvar arr: Array<number> = [1, 2, \"...\"];\n\n// array sugar\nvar array: number[] = [1, 2, \"...\"];\n\nvar matrix: number[][] = [\n  [1, 2],\n  [3, 4],\n];\nvar matrix_parens: number[][] = matrix;\n\nvar nullable_array: ?(number[]) = null;\nvar nullable_array_parens: ?(number[]) = nullable_array;\n\nvar array_of_nullable: (?number)[] = [null, 3];\n\nvar array_of_tuple: [number, string][] = [\n  [0, \"foo\"],\n  [1, \"bar\"],\n];\nvar array_of_tuple_parens: [number, string][] = array_of_tuple;\n\ntype ObjType = { \"bar-foo\": string, \"foo-bar\": number };\nvar test_obj: ObjType = { \"bar-foo\": \"23\" };\n\n// param type annos are strict UBs like var type annos\nfunction param_anno(n: number): void {\n  n = \"hey\"; // error\n}\n\n// another error on param UB, more typical of www (mis)use-cases\n// this one cribbed from API.atlas.js\nfunction param_anno2(\n  batchRequests: Array<{ method: string, path: string, params: ?Object }>,\n): void {\n  // error below, since we're assigning elements to batchRequests\n  // which lack a path property.\n  // just assign result to new var instead of reassigning to param.\n\n  // Transform the requests to the format the Graph API expects.\n  batchRequests = batchRequests.map((request) => {\n    return {\n      method: request.method,\n      params: request.params,\n      relative_url: request.path,\n    };\n  });\n  // ...\n}\n\nvar toz: null = 3;\n\nvar zer: null = null;\n\nfunction foobar(n: ?number): number | null | void {\n  return n;\n}\nfunction barfoo(n: number | null | void): ?number {\n  return n;\n}");
}
#[test]
fn test_forward_ref_js_format_1_7d7f4d35() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("let myClassInstance: MyClass = null; // forward ref ok, null ~> class error\n\nfunction bar(): MyClass {\n  return null; // forward ref ok, null ~> class error\n}\n\nclass MyClass { } // looked up above\n\nfunction foo() {\n  let myClassInstance: MyClass = mk(); // ok (no confusion across scopes)\n  function mk() { return new MyClass(); }\n\n  class MyClass { } // looked up above\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "let myClassInstance: MyClass = null; // forward ref ok, null ~> class error\n\nfunction bar(): MyClass {\n  return null; // forward ref ok, null ~> class error\n}\n\nclass MyClass {} // looked up above\n\nfunction foo() {\n  let myClassInstance: MyClass = mk(); // ok (no confusion across scopes)\n  function mk() {\n    return new MyClass();\n  }\n\n  class MyClass {} // looked up above\n}");
}
#[test]
fn test_issue_530_js_format_1_1be7fb04() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("function foo(...args: any) { }\n\nmodule.exports = foo;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "function foo(...args: any) {}\n\nmodule.exports = foo;"
    );
}
#[test]
fn test_leak_js_format_1_a1632db3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/** @flow */\n\n/* This test documents an example we ran into of a type annotation leaking.\n *\n * When foo() calls bar(), we should make sure the type of x matches the type\n * annotation for y and stop. We should type the body of bar() with the type\n * annotation of y.\n *\n * However, the leaky type annotation meant that we were flowing x's type to y\n * and type checking the body of bar() using the stricter dictionary type,\n * leading to an error.\n */\n\ntype MyObj = Object;\n\nfunction foo(x: {[key: string]: mixed}) {\n  bar(x);\n}\n\nfunction bar(y: MyObj): string {\n  return y.foo;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/** @flow */\n\n/* This test documents an example we ran into of a type annotation leaking.\n *\n * When foo() calls bar(), we should make sure the type of x matches the type\n * annotation for y and stop. We should type the body of bar() with the type\n * annotation of y.\n *\n * However, the leaky type annotation meant that we were flowing x's type to y\n * and type checking the body of bar() using the stricter dictionary type,\n * leading to an error.\n */\n\ntype MyObj = Object;\n\nfunction foo(x: { [key: string]: mixed }) {\n  bar(x);\n}\n\nfunction bar(y: MyObj): string {\n  return y.foo;\n}");
}
#[test]
fn test_other_js_format_1_02c90e52() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("class C { }\nmodule.exports = (C: any);");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "class C {}\nmodule.exports = (C: any);");
}
#[test]
fn test_scope_js_format_1_fe5e0feb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Merge<T> = (a: T, b: T) => T;\n\n// hypothetical immutable map\ndeclare class Map<K,V> {\n  (): Map<K,V>;\n  insertWith(fn: Merge<V>, k: K, v: V): Map<K,V>;\n}\n\ndeclare function foldr<A,B>(fn: (a: A, b: B) => B, b: B, as: A[]): B;\n\nfunction insertMany<K,V>(merge: Merge<V>, vs: [K,V][], m: Map<K,V>): Map<K,V> {\n  function f([k,v]: [K,V], m: Map<K,V>): Map<K,V> {\n    return m.insertWith(merge, k, v)\n  }\n  return foldr(f, m, vs)\n}\n\nclass Foo<A> {\n  bar<B>() {\n    return function<C>(a: A, b: B, c: C): void {\n      ([a,b,c] : [A,B,C]);\n    }\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type Merge<T> = (a: T, b: T) => T;\n\n// hypothetical immutable map\ndeclare class Map<K, V> {\n  (): Map<K, V>;\n  insertWith(fn: Merge<V>, k: K, v: V): Map<K, V>;\n}\n\ndeclare function foldr<A, B>(fn: (a: A, b: B) => B, b: B, as: A[]): B;\n\nfunction insertMany<K, V>(\n  merge: Merge<V>,\n  vs: [K, V][],\n  m: Map<K, V>,\n): Map<K, V> {\n  function f([k, v]: [K, V], m: Map<K, V>): Map<K, V> {\n    return m.insertWith(merge, k, v);\n  }\n  return foldr(f, m, vs);\n}\n\nclass Foo<A> {\n  bar<B>() {\n    return function <C>(a: A, b: B, c: C): void {\n      ([a, b, c]: [A, B, C]);\n    };\n  }\n}");
}
#[test]
fn test_test_js_format_1_599f63e3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("var C = require('./other');\n((0: C): string);");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "var C = require(\"./other\");\n((0: C): string);"
    );
}
