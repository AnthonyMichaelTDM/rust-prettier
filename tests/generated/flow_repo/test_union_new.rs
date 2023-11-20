#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_issue_815_js_format_1_76bc0671() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\ntype T = A|B;\nclass U {};\ndeclare var children: U;\n(children: T|U);\nclass A {};\nclass B {};\n\ntype VirtualElement = Thunk|VirtualNode;\ntype Child = VirtualElement;\ntype Children = Array<Child>;\n\n\nclass Thunk {}\nclass VirtualNode {\n  children: Child|Children;\n  constructor(type, children/*:Children*/) {\n    this.children = children.length === 1 ? children[0] :\n      children;\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\ntype T = A | B;\nclass U {}\ndeclare var children: U;\n(children: T | U);\nclass A {}\nclass B {}\n\ntype VirtualElement = Thunk | VirtualNode;\ntype Child = VirtualElement;\ntype Children = Array<Child>;\n\nclass Thunk {}\nclass VirtualNode {\n  children: Child | Children;\n  constructor(type, children: Children) {\n    this.children = children.length === 1 ? children[0] : children;\n  }\n}");
}
#[test]
fn test_issue_824_js_format_1_643ad0ae() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import { B, C } from \"./issue-824-helper\";\n\ntype K = B | C;\n\ntype I = {\n  which(): number;\n};\n\nexport default class A {\n  static foo(p: K): bool {\n    return false;\n  }\n  static bar(p: I & K): bool {\n    return this.foo(p);\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import { B, C } from \"./issue-824-helper\";\n\ntype K = B | C;\n\ntype I = {\n  which(): number,\n};\n\nexport default class A {\n  static foo(p: K): boolean {\n    return false;\n  }\n  static bar(p: I & K): boolean {\n    return this.foo(p);\n  }\n}");
}
#[test]
fn test_issue_824_helper_js_format_1_87a557db() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import A from \"./issue-824\";\n\nexport class B extends A {\n  which(): number {\n    return 1;\n  }\n}\nexport class C extends A {\n  which(): number {\n    return 2;\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import A from \"./issue-824\";\n\nexport class B extends A {\n  which(): number {\n    return 1;\n  }\n}\nexport class C extends A {\n  which(): number {\n    return 2;\n  }\n}");
}
#[test]
fn test_issue_1349_js_format_1_e76035ce() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nvar bar: Array<{b: ?boolean, c: number} | {b: boolean}> = [\n  {b: true, c: 123},\n  {b: true}\n];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nvar bar: Array<{ b: ?boolean, c: number } | { b: boolean }> = [\n  { b: true, c: 123 },\n  { b: true },\n];");
}
#[test]
fn test_issue_1371_js_format_1_9e6f9c45() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function create(a: any): { type: 'B', data: number } | { type: 'A', data: string }\n{\n  return {\n    type: 'A',\n    data: a\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function create(\n  a: any,\n): { type: \"B\", data: number } | { type: \"A\", data: string } {\n  return {\n    type: \"A\",\n    data: a,\n  };\n}");
}
#[test]
fn test_issue_1455_js_format_1_d03d6231() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\nimport type {Foobar} from \"./issue-1455-helper\"\n\nfunction create(content: ?Foobar | String | Array<String>) {\n}\n\nfunction node(content: ?Foobar | String | Array<String>) {\n  create(content)\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\nimport type { Foobar } from \"./issue-1455-helper\";\n\nfunction create(content: ?Foobar | String | Array<String>) {}\n\nfunction node(content: ?Foobar | String | Array<String>) {\n  create(content);\n}");
}
#[test]
fn test_issue_1455_helper_js_format_1_33df0d66() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* @flow */\n\nexport class Foobar { }");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "/* @flow */\n\nexport class Foobar {}");
}
#[test]
fn test_issue_1462_i_js_format_1_36e992e5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Common = {\n};\n\ntype A = Common & {\n  type: 'A',\n  foo: number\n};\n\ntype B = Common & {\n  type: 'B',\n  foo: Array<number>\n}\n\ntype MyType = A | B;\n\nfunction print(x: number) {\n  console.log(x);\n}\n\nfunction printAll(val: MyType) {\n  print(val.foo);  // <--- foo could be an array\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type Common = {};\n\ntype A = Common & {\n  type: \"A\",\n  foo: number,\n};\n\ntype B = Common & {\n  type: \"B\",\n  foo: Array<number>,\n};\n\ntype MyType = A | B;\n\nfunction print(x: number) {\n  console.log(x);\n}\n\nfunction printAll(val: MyType) {\n  print(val.foo); // <--- foo could be an array\n}");
}
#[test]
fn test_issue_1462_ii_js_format_1_49dfe332() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Common = {\n};\n\ntype A = {\n  type: 'A',\n  foo: number\n} & Common;\n\ntype B = {\n  type: 'B',\n  foo: Array<number>\n} & Common;\n\ntype MyType = A | B;\n\n\nfunction print(x: number) {\n  console.log(x);\n}\n\nfunction printAll(val: MyType) {\n  if (val.type === 'A') {\n    print(val.foo);\n  } else {\n    val.foo.forEach(print);\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type Common = {};\n\ntype A = {\n  type: \"A\",\n  foo: number,\n} & Common;\n\ntype B = {\n  type: \"B\",\n  foo: Array<number>,\n} & Common;\n\ntype MyType = A | B;\n\nfunction print(x: number) {\n  console.log(x);\n}\n\nfunction printAll(val: MyType) {\n  if (val.type === \"A\") {\n    print(val.foo);\n  } else {\n    val.foo.forEach(print);\n  }\n}");
}
#[test]
fn test_issue_1664_js_format_1_a013cacf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\ntype DataBase = {\n  id: string,\n  name: string,\n};\n\ntype UserData = DataBase & {\n  kind: \"user\",\n};\n\ntype SystemData = DataBase & {\n  kind: \"system\",\n}\n\ntype Data = UserData | SystemData;\n\nconst data: Data = {\n  id: \"\",\n  name: \"\",\n  kind: \"system\",\n}\n\nif (data.kind === \"system\") {\n  (data: SystemData);\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\ntype DataBase = {\n  id: string,\n  name: string,\n};\n\ntype UserData = DataBase & {\n  kind: \"user\",\n};\n\ntype SystemData = DataBase & {\n  kind: \"system\",\n};\n\ntype Data = UserData | SystemData;\n\nconst data: Data = {\n  id: \"\",\n  name: \"\",\n  kind: \"system\",\n};\n\nif (data.kind === \"system\") {\n  (data: SystemData);\n}");
}
#[test]
fn test_issue_1759_js_format_1_06a8f5a5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\ntype X = ({a:true} & {b:string}) | ({a:false} & {c:string});\n//type X = {a:true, b:string} | {a:false, c:string}; // this works.\n\nfunction hello(x:X): string {\n  if (x.a === true) return x.b; else return x.c;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\ntype X = ({ a: true } & { b: string }) | ({ a: false } & { c: string });\n//type X = {a:true, b:string} | {a:false, c:string}; // this works.\n\nfunction hello(x: X): string {\n  if (x.a === true) return x.b;\n  else return x.c;\n}");
}
#[test]
fn test_issue_2232_js_format_1_dfe4ba28() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\ndeclare type Entity<T> = {\n  id: T,\n  name: string\n}\n\ndeclare type StringEntity = Entity<string>\n\n\ndeclare type Foo = StringEntity & {\n  bars: Object,\n  kind: 1\n}\ndeclare type EmptyFoo = StringEntity &  {\n  bars: null,\n  kind: 2\n}\n\nfunction test(f: Foo| EmptyFoo) {\n  if (f.kind === 1) {\n    (f: Foo)\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\ndeclare type Entity<T> = {\n  id: T,\n  name: string,\n};\n\ndeclare type StringEntity = Entity<string>;\n\ndeclare type Foo = StringEntity & {\n  bars: Object,\n  kind: 1,\n};\ndeclare type EmptyFoo = StringEntity & {\n  bars: null,\n  kind: 2,\n};\n\nfunction test(f: Foo | EmptyFoo) {\n  if (f.kind === 1) {\n    (f: Foo);\n  }\n}");
}
#[test]
fn test_test_1_js_format_1_1a9572ba() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @noflow\n\n/**\n * Test that shows how the implementation of union types is broken\n */\n\n//////////////////////////////\n// example with object types\n//////////////////////////////\n\nfunction obj(a: A1 | A2) {\n  return a.x;\n}\n\nconst obj_result = obj({ x: \"\" }); // currently an error! (expect it to be OK)\n\n// Type definitions used above are defined below, but in an order that\n// deliberately makes their full resolution as lazy as possible. The call above\n// blocks until A1 is partially resolved. Since the argument partially matches\n// A1, that branch is selected. Later, that branch errors, but other branches\n// have been lost by then.\n\ntype A1 = { x: B1 };\ntype A2 = { x: B2 };\n\ntype B1 = number;\ntype B2 = string;\n\n(obj_result: B1 | B2);\n\n///////////////////////////////////////\n// similar example with function types\n///////////////////////////////////////\n\nfunction fun(a: A3 | A4) {\n  return a();\n}\n\nconst fun_result = fun(() => \"\");\n\ntype A3 = () => B3;\ntype A4 = () => B4;\n\ntype B3 = number;\ntype B4 = string;\n\n(fun_result: B3 | B4);\n\n/////////////////////////////////////////////\n// similar example with class instance types\n/////////////////////////////////////////////\n\nfunction inst(a: A5 | A6) { }\n\nclass B5 { }\nclass B6 { }\ninst([new B6]);\n\ntype A5 = B5[];\ntype A6 = B6[];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @noflow\n\n/**\n * Test that shows how the implementation of union types is broken\n */\n\n//////////////////////////////\n// example with object types\n//////////////////////////////\n\nfunction obj(a: A1 | A2) {\n  return a.x;\n}\n\nconst obj_result = obj({ x: \"\" }); // currently an error! (expect it to be OK)\n\n// Type definitions used above are defined below, but in an order that\n// deliberately makes their full resolution as lazy as possible. The call above\n// blocks until A1 is partially resolved. Since the argument partially matches\n// A1, that branch is selected. Later, that branch errors, but other branches\n// have been lost by then.\n\ntype A1 = { x: B1 };\ntype A2 = { x: B2 };\n\ntype B1 = number;\ntype B2 = string;\n\n(obj_result: B1 | B2);\n\n///////////////////////////////////////\n// similar example with function types\n///////////////////////////////////////\n\nfunction fun(a: A3 | A4) {\n  return a();\n}\n\nconst fun_result = fun(() => \"\");\n\ntype A3 = () => B3;\ntype A4 = () => B4;\n\ntype B3 = number;\ntype B4 = string;\n\n(fun_result: B3 | B4);\n\n/////////////////////////////////////////////\n// similar example with class instance types\n/////////////////////////////////////////////\n\nfunction inst(a: A5 | A6) {}\n\nclass B5 {}\nclass B6 {}\ninst([new B6()]);\n\ntype A5 = B5[];\ntype A6 = B6[];");
}
#[test]
fn test_test_2_js_format_1_69628aed() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @noflow\n\n/**\n * Test that shows how the implementation of union types is broken\n */\n\n//////////////////////////////\n// example with object types\n//////////////////////////////\n\nfunction obj(a: { x: number } | { x: string }) { }\n\nobj(({ x: \"\" }: A1));\n\ntype A1 = { x: B1 };\n\ntype B1 = string;\n\n///////////////////////////////////////\n// similar example with function types\n///////////////////////////////////////\n\nfunction fun(a: (() => number) | (() => string)) { }\n\nfun(((() => \"\"): A2));\n\ntype A2 = () => B2;\n\ntype B2 = string;\n\n/////////////////////////////////////////////////////\n// similar example with generic class instance types\n/////////////////////////////////////////////////////\n\nclass C<X> { }\n\nfunction inst(a: C<number> | C<string>) { }\n\ninst((new C: A3));\n\ntype A3 = C<B3>;\n\ntype B3 = string;\n\n/////////////////////////////////////////////\n// similar example with generic type aliases\n/////////////////////////////////////////////\n\nfunction alias(a: T<number> | T<string>) { }\nalias({ x: (x: V<B4>) => { } });\n\ntype T<X> = { x: U<X> }\ntype U<X> = (x: V<X>) => void;\ntype V<X> = X;\n\ntype B4 = string;\n\n// class statics\n\nfunction stat(a: { x: number } | { x: string }) { }\n\nclass D {\n  static x: B5;\n}\n\nstat(D);\n\ntype B5 = string;\n\n// tuples\n\nfunction tup(a: [number,boolean] | [string,boolean]) { }\n\ntup(([\"\",false]: A6));\n\ntype A6 = [B6,boolean];\ntype B6 = string;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @noflow\n\n/**\n * Test that shows how the implementation of union types is broken\n */\n\n//////////////////////////////\n// example with object types\n//////////////////////////////\n\nfunction obj(a: { x: number } | { x: string }) {}\n\nobj(({ x: \"\" }: A1));\n\ntype A1 = { x: B1 };\n\ntype B1 = string;\n\n///////////////////////////////////////\n// similar example with function types\n///////////////////////////////////////\n\nfunction fun(a: (() => number) | (() => string)) {}\n\nfun((() => \"\": A2));\n\ntype A2 = () => B2;\n\ntype B2 = string;\n\n/////////////////////////////////////////////////////\n// similar example with generic class instance types\n/////////////////////////////////////////////////////\n\nclass C<X> {}\n\nfunction inst(a: C<number> | C<string>) {}\n\ninst((new C(): A3));\n\ntype A3 = C<B3>;\n\ntype B3 = string;\n\n/////////////////////////////////////////////\n// similar example with generic type aliases\n/////////////////////////////////////////////\n\nfunction alias(a: T<number> | T<string>) {}\nalias({ x: (x: V<B4>) => {} });\n\ntype T<X> = { x: U<X> };\ntype U<X> = (x: V<X>) => void;\ntype V<X> = X;\n\ntype B4 = string;\n\n// class statics\n\nfunction stat(a: { x: number } | { x: string }) {}\n\nclass D {\n  static x: B5;\n}\n\nstat(D);\n\ntype B5 = string;\n\n// tuples\n\nfunction tup(a: [number, boolean] | [string, boolean]) {}\n\ntup(([\"\", false]: A6));\n\ntype A6 = [B6, boolean];\ntype B6 = string;");
}
#[test]
fn test_test_3_js_format_1_e7af40a3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @noflow\n\n/**\n * Test that shows how the implementation of union types is broken\n */\n\n///////////////////////////////\n// example with function types\n///////////////////////////////\n\nfunction fun(a: ((x: number) => void) | ((x: string) => void)) { }\n\nfun((((x) => {}): A1));\n\ntype A1 = (x: B1) => void;\n\ntype B1 = string;\n\n////////////////////////////\n// example with array types\n////////////////////////////\n\nfunction arr(a: number[] | string[]) { }\n\narr(([]: A2));\n\ntype A2 = B2[];\n\ntype B2 = string;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @noflow\n\n/**\n * Test that shows how the implementation of union types is broken\n */\n\n///////////////////////////////\n// example with function types\n///////////////////////////////\n\nfunction fun(a: ((x: number) => void) | ((x: string) => void)) {}\n\nfun(((x) => {}: A1));\n\ntype A1 = (x: B1) => void;\n\ntype B1 = string;\n\n////////////////////////////\n// example with array types\n////////////////////////////\n\nfunction arr(a: number[] | string[]) {}\n\narr(([]: A2));\n\ntype A2 = B2[];\n\ntype B2 = string;");
}
#[test]
fn test_test_4_js_format_1_0049b02d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @noflow\n\n/**\n * Test that shows how the implementation of union types is broken\n */\n\n///////////////////////////////\n// example with function types\n///////////////////////////////\n\nfunction fun(a: ((x: number) => void) | ((x: string) => void)) { }\n\nconst a1 = ((x) => {}: A1);\nfun(a1);\n\nfunction fun_call(x: string) { a1(x); }\n\ntype A1 = (x: B1) => void;\n\ntype B1 = string;\n\n////////////////////////////\n// example with array types\n////////////////////////////\n\nfunction arr(a: number[] | string[]) { }\n\nconst a2 = ([]: A2);\narr(a2);\n\nfunction arr_set(x: string, i: number) { a2[i] = x; }\nfunction arr_get(i: number): string { return a2[i]; }\n\ntype A2 = B2[];\n\ntype B2 = string;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @noflow\n\n/**\n * Test that shows how the implementation of union types is broken\n */\n\n///////////////////////////////\n// example with function types\n///////////////////////////////\n\nfunction fun(a: ((x: number) => void) | ((x: string) => void)) {}\n\nconst a1 = ((x) => {}: A1);\nfun(a1);\n\nfunction fun_call(x: string) {\n  a1(x);\n}\n\ntype A1 = (x: B1) => void;\n\ntype B1 = string;\n\n////////////////////////////\n// example with array types\n////////////////////////////\n\nfunction arr(a: number[] | string[]) {}\n\nconst a2 = ([]: A2);\narr(a2);\n\nfunction arr_set(x: string, i: number) {\n  a2[i] = x;\n}\nfunction arr_get(i: number): string {\n  return a2[i];\n}\n\ntype A2 = B2[];\n\ntype B2 = string;");
}
#[test]
fn test_test_5_js_format_1_a41ac0ce() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @noflow\n\n/**\n * Test that shows how the implementation of union types is broken\n */\n\n///////////////////////////////\n// example with function types\n///////////////////////////////\n\nfunction fun(a: ((x: number) => number) | ((x: string) => string)) { }\n\nfunction a1(x) { return x; }\nfun(a1);\n\nfunction fun_call(x: string): string { return a1(x); }\n\n/////////////////////////////\n// example with array types\n/////////////////////////////\n\nfunction arr(a: number[] | string[]) { }\n\nvar a2 = [];\narr(a2);\n\nfunction arr_set(x: string, i: number) { a2[i] = x; }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @noflow\n\n/**\n * Test that shows how the implementation of union types is broken\n */\n\n///////////////////////////////\n// example with function types\n///////////////////////////////\n\nfunction fun(a: ((x: number) => number) | ((x: string) => string)) {}\n\nfunction a1(x) {\n  return x;\n}\nfun(a1);\n\nfunction fun_call(x: string): string {\n  return a1(x);\n}\n\n/////////////////////////////\n// example with array types\n/////////////////////////////\n\nfunction arr(a: number[] | string[]) {}\n\nvar a2 = [];\narr(a2);\n\nfunction arr_set(x: string, i: number) {\n  a2[i] = x;\n}");
}
#[test]
fn test_test_6_js_format_1_451f1a0a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @noflow\n\n/**\n * Test that shows how the implementation of union types is broken\n */\n\n//////////////////////////////////////////\n// example with generic class inheritance\n//////////////////////////////////////////\n\nfunction inst(a: E<B4>): C<number> | C<string> { return a; }\n\nconst mk_C = () => C;\nconst mk_D = () => D;\nconst mk_E = () => E;\n\ntype B4 = string;\n\nconst _D = mk_D();\nclass E<X> extends _D<X> { }\n\nconst _C = mk_C();\nclass D<X> extends _C<X> { }\n\nclass C<X> { }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @noflow\n\n/**\n * Test that shows how the implementation of union types is broken\n */\n\n//////////////////////////////////////////\n// example with generic class inheritance\n//////////////////////////////////////////\n\nfunction inst(a: E<B4>): C<number> | C<string> {\n  return a;\n}\n\nconst mk_C = () => C;\nconst mk_D = () => D;\nconst mk_E = () => E;\n\ntype B4 = string;\n\nconst _D = mk_D();\nclass E<X> extends _D<X> {}\n\nconst _C = mk_C();\nclass D<X> extends _C<X> {}\n\nclass C<X> {}");
}
#[test]
fn test_test_7_js_format_1_636eb9b5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @noflow\n\n/**\n * Test that shows how the implementation of union types is broken\n */\n\n////////////////////\n// recursive types\n////////////////////\n\nfunction rec(x: F1 | F2) { }\nrec({ x: 0 });\n\ntype F1 = G1;\ntype F2 = G2;\ntype G1 = { x: H1, y?: G1 };\ntype G2 = { x: H2, y?: G2 };\ntype H1 = string;\ntype H2 = number;\n\n///////////////////////////////\n// polymorphic recursive types\n///////////////////////////////\n\nfunction polyrec(x: PF<number> | PF<string>) { }\nrec({ x: 0 });\n\ntype PF<X> = PG<X>;\ntype PG<X> = { x: X, y?: PG<X> };") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @noflow\n\n/**\n * Test that shows how the implementation of union types is broken\n */\n\n////////////////////\n// recursive types\n////////////////////\n\nfunction rec(x: F1 | F2) {}\nrec({ x: 0 });\n\ntype F1 = G1;\ntype F2 = G2;\ntype G1 = { x: H1, y?: G1 };\ntype G2 = { x: H2, y?: G2 };\ntype H1 = string;\ntype H2 = number;\n\n///////////////////////////////\n// polymorphic recursive types\n///////////////////////////////\n\nfunction polyrec(x: PF<number> | PF<string>) {}\nrec({ x: 0 });\n\ntype PF<X> = PG<X>;\ntype PG<X> = { x: X, y?: PG<X> };");
}
#[test]
fn test_test_8_js_format_1_329862cd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @noflow\n\n/**\n * Test that shows how the implementation of union types is broken\n */\n\n//////////////////////\n// nested union types\n//////////////////////\n\nfunction rec(x: F1 | F2) { }\nrec({ x: 0 });\n\ntype F1 = G1 | G1_;\ntype F2 = G2 | G2_;\ntype G1 = { x: H1 };\ntype G1_ = { x: H1_ };\ntype G2 = { x: H2 };\ntype G2_ = { x: H2_ };\ntype H1 = boolean;\ntype H1_ = string;\ntype H2 = boolean;\ntype H2_ = number;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @noflow\n\n/**\n * Test that shows how the implementation of union types is broken\n */\n\n//////////////////////\n// nested union types\n//////////////////////\n\nfunction rec(x: F1 | F2) {}\nrec({ x: 0 });\n\ntype F1 = G1 | G1_;\ntype F2 = G2 | G2_;\ntype G1 = { x: H1 };\ntype G1_ = { x: H1_ };\ntype G2 = { x: H2 };\ntype G2_ = { x: H2_ };\ntype H1 = boolean;\ntype H1_ = string;\ntype H2 = boolean;\ntype H2_ = number;");
}
#[test]
fn test_test_9_js_format_1_5dbd76e9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @noflow\n\n/**\n * Test that shows how the implementation of union types is broken\n */\n\n////////////////\n// interference\n////////////////\n\nfunction square(x? = 0) {\n  return x * x;\n}\n\nfunction foo(f: ((_: ?number) => ?number) | (() => void)) { }\nfoo((x): number => square(x))") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @noflow\n\n/**\n * Test that shows how the implementation of union types is broken\n */\n\n////////////////\n// interference\n////////////////\n\nfunction square(x? = 0) {\n  return x * x;\n}\n\nfunction foo(f: ((_: ?number) => ?number) | (() => void)) {}\nfoo((x): number => square(x));");
}
#[test]
fn test_test_10_js_format_1_b01dfd49() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @noflow\n\nfunction id<X>(x: X): X { return x; }\n\n/////////////////////////\n// primitive annotations\n/////////////////////////\n\nfunction check_prim(_: number | string) { }\n\n// ok\ncheck_prim(\"\");\n\n// ...even when they \"flow\" in\ncheck_prim(id(\"\"));\n\n//////////////////////////////\n// class instance annotations\n//////////////////////////////\n\nclass C { }\nclass D { }\nfunction check_inst(_: C | D) { }\n\n// ok\ncheck_inst(new D);\n\n// ...even when they \"flow\" in\ncheck_inst(id(new C));\n\n////////////////////////\n// function annotations\n////////////////////////\n\nfunction check_fun(_: ((_: number) => number) | ((_: string) => string)) { }\n\n// help!\ncheck_fun((x) => x);\n\n//////////////////////\n// object annotations\n//////////////////////\n\nfunction check_obj(_: { x: number } | { x: string }) { }\n\n// ok\ncheck_obj({ x: \"\" });\n\n// help!\ncheck_obj({ x: id(\"\") });\n\n/////////////////////\n// array annotations\n/////////////////////\n\nfunction check_arr(_: number[] | string[]) { }\n\n// help! (unlike objects, array literals' element types are always open)\ncheck_arr([\"\"]);\n\n// help!\ncheck_arr([id(\"\")]);\n\n//////////////////////////////////////\n// generic class instance annotations\n//////////////////////////////////////\n\nclass P<X> { }\nfunction check_poly_inst(_: P<number> | P<string>) { }\n\n// help!\ncheck_poly_inst(new P);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @noflow\n\nfunction id<X>(x: X): X {\n  return x;\n}\n\n/////////////////////////\n// primitive annotations\n/////////////////////////\n\nfunction check_prim(_: number | string) {}\n\n// ok\ncheck_prim(\"\");\n\n// ...even when they \"flow\" in\ncheck_prim(id(\"\"));\n\n//////////////////////////////\n// class instance annotations\n//////////////////////////////\n\nclass C {}\nclass D {}\nfunction check_inst(_: C | D) {}\n\n// ok\ncheck_inst(new D());\n\n// ...even when they \"flow\" in\ncheck_inst(id(new C()));\n\n////////////////////////\n// function annotations\n////////////////////////\n\nfunction check_fun(_: ((_: number) => number) | ((_: string) => string)) {}\n\n// help!\ncheck_fun((x) => x);\n\n//////////////////////\n// object annotations\n//////////////////////\n\nfunction check_obj(_: { x: number } | { x: string }) {}\n\n// ok\ncheck_obj({ x: \"\" });\n\n// help!\ncheck_obj({ x: id(\"\") });\n\n/////////////////////\n// array annotations\n/////////////////////\n\nfunction check_arr(_: number[] | string[]) {}\n\n// help! (unlike objects, array literals' element types are always open)\ncheck_arr([\"\"]);\n\n// help!\ncheck_arr([id(\"\")]);\n\n//////////////////////////////////////\n// generic class instance annotations\n//////////////////////////////////////\n\nclass P<X> {}\nfunction check_poly_inst(_: P<number> | P<string>) {}\n\n// help!\ncheck_poly_inst(new P());");
}
#[test]
fn test_test_11_js_format_1_80dd6983() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @noflow\n\n// disjoint unions\n\nfunction length(list: List) {\n  if (list.kind === \"cons\") return length(list.next) + 1;\n  else return 0;\n}\n\n\nlength({ kind: \"nil\" });\nlength({ kind: \"cons\" }); // missing \\`next\\`\nlength({ kind: \"cons\", next: { kind: \"nil\" } });\nlength({ kind: \"empty\" }); // \\`kind\\` not found\n\ntype List = Nil | Cons;\ntype Nil = { kind: \"nil\" };\ntype Cons = { kind: \"cons\", next: List };") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @noflow\n\n// disjoint unions\n\nfunction length(list: List) {\n  if (list.kind === \"cons\") return length(list.next) + 1;\n  else return 0;\n}\n\nlength({ kind: \"nil\" });\nlength({ kind: \"cons\" }); // missing \\`next\\`\nlength({ kind: \"cons\", next: { kind: \"nil\" } });\nlength({ kind: \"empty\" }); // \\`kind\\` not found\n\ntype List = Nil | Cons;\ntype Nil = { kind: \"nil\" };\ntype Cons = { kind: \"cons\", next: List };");
}
#[test]
fn test_test_12_js_format_1_245b67af() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @noflow\n\n// polymorphic recursive types\n\ntype F<X> = { f: F<X>, x: X }\ntype G = { x: number }\ntype H = { x: string }\n\nfunction rec(x: F<string>): G | H { return x; }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @noflow\n\n// polymorphic recursive types\n\ntype F<X> = { f: F<X>, x: X };\ntype G = { x: number };\ntype H = { x: string };\n\nfunction rec(x: F<string>): G | H {\n  return x;\n}");
}
#[test]
fn test_test_13_js_format_1_d3cdd4b2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @noflow\n\n/* ensure there are no unintended side effects when trying branches */\n\n({type: 'B', id: 'hi'}: {\n  type: 'A';\n  id: ?string;\n} | {\n  type: 'B';\n  id: string;\n});") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @noflow\n\n/* ensure there are no unintended side effects when trying branches */\n\n({ type: \"B\", id: \"hi\" }:\n  | {\n      type: \"A\",\n      id: ?string,\n    }\n  | {\n      type: \"B\",\n      id: string,\n    });");
}
#[test]
fn test_test_14_js_format_1_afd89d00() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @noflow\n\n// annotations\n\ndeclare class C<X> {\n  get(): X;\n}\n\nfunction union(o: { x: string } | { x: number }) { }\n\nfunction foo(c: C<number>) {\n  union({ x: c.get() });\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @noflow\n\n// annotations\n\ndeclare class C<X> {\n  get(): X;\n}\n\nfunction union(o: { x: string } | { x: number }) {}\n\nfunction foo(c: C<number>) {\n  union({ x: c.get() });\n}");
}
#[test]
fn test_test_15_js_format_1_76fd0a32() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @noflow\n\n// functions as objects\n\nfunction foo<X>(target: EventTarget) {\n  target.addEventListener('click', (e) => {});\n}\n\ndeclare class EventTarget {\n  addEventListener(type: 'foo', listener: KeyboardEventHandler): void;\n  addEventListener(type: string, listener: EventHandler): void;\n}\n\ndeclare class Event { }\ndeclare class KeyboardEvent { }\n\ntype EventHandler = (event: Event) => mixed\ntype KeyboardEventHandler = (event: KeyboardEvent) => mixed\n\n// example where globals are not yet resolved\n\nfunction bar(x: (() => void) | { x: number }) { }\n\nbar(() => { });") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @noflow\n\n// functions as objects\n\nfunction foo<X>(target: EventTarget) {\n  target.addEventListener(\"click\", (e) => {});\n}\n\ndeclare class EventTarget {\n  addEventListener(type: \"foo\", listener: KeyboardEventHandler): void;\n  addEventListener(type: string, listener: EventHandler): void;\n}\n\ndeclare class Event {}\ndeclare class KeyboardEvent {}\n\ntype EventHandler = (event: Event) => mixed;\ntype KeyboardEventHandler = (event: KeyboardEvent) => mixed;\n\n// example where globals are not yet resolved\n\nfunction bar(x: (() => void) | { x: number }) {}\n\nbar(() => {});");
}
#[test]
fn test_test_16_js_format_1_b871b308() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @noflow\n\n// annotations\n\ntype T = number | (() => string);\ntype Foo = T | (() => bool);\n\ntype Bar = number | (() => string) | (() => bool);\n\nfunction foo(x: Foo) { }\nfoo(() => qux());\n\nfunction bar(x: Bar) { }\nbar(() => qux());\n\nvar x = false;\nfunction qux() { return x; }\nx = \"\";") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @noflow\n\n// annotations\n\ntype T = number | (() => string);\ntype Foo = T | (() => boolean);\n\ntype Bar = number | (() => string) | (() => boolean);\n\nfunction foo(x: Foo) {}\nfoo(() => qux());\n\nfunction bar(x: Bar) {}\nbar(() => qux());\n\nvar x = false;\nfunction qux() {\n  return x;\n}\nx = \"\";");
}
#[test]
fn test_test_17_js_format_1_9898eba4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("// @noflow\n\n// Array#concat\n\n[].concat([]);\n\n([].concat([0,1])[1]: string)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// @noflow\n\n// Array#concat\n\n[].concat([]);\n\n([].concat([0, 1])[1]: string);"
    );
}
#[test]
fn test_test_18_js_format_1_67ae7a96() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @noflow\n\n// method overloads\n\ndeclare class C {\n  m(x: number): void;\n  m(x: string): void;\n}\n\nfunction f() { return 0; }\n\nnew C().m(f());") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @noflow\n\n// method overloads\n\ndeclare class C {\n  m(x: number): void;\n  m(x: string): void;\n}\n\nfunction f() {\n  return 0;\n}\n\nnew C().m(f());");
}
#[test]
fn test_test_19_js_format_1_864da224() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @noflow\n\n// constructor overloads\n\nfunction m<X>() {\n  return new D();\n}\n\ndeclare class D {\n  constructor(_: void): void;\n  constructor(_: null): void;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @noflow\n\n// constructor overloads\n\nfunction m<X>() {\n  return new D();\n}\n\ndeclare class D {\n  constructor(_: void): void;\n  constructor(_: null): void;\n}");
}
#[test]
fn test_test_20_js_format_1_d5315a51() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @noflow\n\n// Array#reduce\n\n[0,1].reduce((x,y,i) => y);\n\n[\"a\", \"b\"].reduce(\n  (regex, representation, index) => {\n    return regex + (index ? '|' : '') + '(' + representation + ')';\n  },\n  '',\n);\n\n[\"\"].reduce((acc,str) => acc * str.length);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @noflow\n\n// Array#reduce\n\n[0, 1].reduce((x, y, i) => y);\n\n[\"a\", \"b\"].reduce((regex, representation, index) => {\n  return regex + (index ? \"|\" : \"\") + \"(\" + representation + \")\";\n}, \"\");\n\n[\"\"].reduce((acc, str) => acc * str.length);");
}
#[test]
fn test_test_21_js_format_1_80bd62b7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @noflow\n\n// annotations for disjoint unions\n\ntype T =\n  | { type: \"FOO\", x: number }\n  | { type: \"BAR\", x: string }\n\n({ type: (bar(): \"BAR\"), x: str() }: T);\n\n({ type: bar(), x: str() }: T);\n\n({ type: bar(), x: (str(): string) }: T);\n\nfunction bar() {\n  return \"BAR\";\n}\n\nfunction str() {\n  return \"hello\";\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @noflow\n\n// annotations for disjoint unions\n\ntype T = { type: \"FOO\", x: number } | { type: \"BAR\", x: string };\n\n({ type: (bar(): \"BAR\"), x: str() }: T);\n\n({ type: bar(), x: str() }: T);\n\n({ type: bar(), x: (str(): string) }: T);\n\nfunction bar() {\n  return \"BAR\";\n}\n\nfunction str() {\n  return \"hello\";\n}");
}
#[test]
fn test_test_22_js_format_1_0c92e585() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @noflow\n\n// refinement of disjoint unions\n\ntype Empty = { }\n\ntype Success = {\n  type: 'SUCCESS';\n  result: string;\n};\n\ntype Error = {\n  type: 'ERROR';\n} & Empty;\n\nexport type T = Success | Error;\n\nfunction foo(x: T) {\n  if (x.type === 'SUCCESS') return x.result;\n  else return x.result;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @noflow\n\n// refinement of disjoint unions\n\ntype Empty = {};\n\ntype Success = {\n  type: \"SUCCESS\",\n  result: string,\n};\n\ntype Error = {\n  type: \"ERROR\",\n} & Empty;\n\nexport type T = Success | Error;\n\nfunction foo(x: T) {\n  if (x.type === \"SUCCESS\") return x.result;\n  else return x.result;\n}");
}
#[test]
fn test_test_23_js_format_1_1e7730fc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @noflow\n\n// nested intersections (see also lib/test23_lib.js)\n\ntype NestedObj = { } & { dummy: SomeLibClass };\n\ntype Obj = NestedObj & { x: string };\n\nfunction foo(obj: Obj) {\n  obj.x; // should be OK\n  obj.x; // should also be OK (the check above shouldn't affect anything)\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @noflow\n\n// nested intersections (see also lib/test23_lib.js)\n\ntype NestedObj = {} & { dummy: SomeLibClass };\n\ntype Obj = NestedObj & { x: string };\n\nfunction foo(obj: Obj) {\n  obj.x; // should be OK\n  obj.x; // should also be OK (the check above shouldn't affect anything)\n}");
}
#[test]
fn test_test_24_js_format_1_c4eab840() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @noflow\n\n// scaling test for full type resolution\n\ndeclare class C {\n  addListener(event: string, listener: Function): C;\n  emit(event: string, ...args:Array<any>): boolean;\n  listeners(event: string): Array<Function>;\n  listenerCount(event: string): number;\n  on(event: string, listener: Function): C;\n  once(event: string, listener: Function): C;\n  removeAllListeners(event?: string): C;\n  removeListener(event: string, listener: Function): C;\n  setMaxListeners(n: number): void;\n}\n\ndeclare class D extends C {\n  listen(port: number, hostname?: string, backlog?: number, callback?: Function): D;\n  listen(path: string, callback?: Function): D;\n  listen(handle: Object, callback?: Function): D;\n  close(callback?: Function): D;\n  address(): number;\n  connections: number;\n  maxConnections: number;\n  getConnections(callback: Function): void;\n  ref():  D;\n  unref():  D;\n}\n\n(0: D | number);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @noflow\n\n// scaling test for full type resolution\n\ndeclare class C {\n  addListener(event: string, listener: Function): C;\n  emit(event: string, ...args: Array<any>): boolean;\n  listeners(event: string): Array<Function>;\n  listenerCount(event: string): number;\n  on(event: string, listener: Function): C;\n  once(event: string, listener: Function): C;\n  removeAllListeners(event?: string): C;\n  removeListener(event: string, listener: Function): C;\n  setMaxListeners(n: number): void;\n}\n\ndeclare class D extends C {\n  listen(\n    port: number,\n    hostname?: string,\n    backlog?: number,\n    callback?: Function,\n  ): D;\n  listen(path: string, callback?: Function): D;\n  listen(handle: Object, callback?: Function): D;\n  close(callback?: Function): D;\n  address(): number;\n  connections: number;\n  maxConnections: number;\n  getConnections(callback: Function): void;\n  ref(): D;\n  unref(): D;\n}\n\n(0: D | number);");
}
#[test]
fn test_test_25_js_format_1_c6f229fd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @noflow\n\n// termination test (see also lib/test25_lib.js)\n\nfunction foo(rows: Rows, set: Set<number>) {\n  return rows.reduce_rows(\n    (set, row) => row.reduce_row(\n      (set, i) => set.add(i),\n      set,\n    ),\n    set,\n  );\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @noflow\n\n// termination test (see also lib/test25_lib.js)\n\nfunction foo(rows: Rows, set: Set<number>) {\n  return rows.reduce_rows(\n    (set, row) => row.reduce_row((set, i) => set.add(i), set),\n    set,\n  );\n}");
}
#[test]
fn test_test_26_js_format_1_f955d810() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @noflow\n\ndeclare function foo(x: number): number;\ndeclare function foo(x: string): string;\n\ndeclare var x: number | string;\n\n(foo(x): number | string);\n\ntype T = number | string;\ndeclare var y: T;\n\n(foo(y): T);\n\ndeclare class Record {\n  set(x: 'foo', y: number): void;\n  set(x: 'bar', y: string): void;\n}\n\nnew Record().set('foo', \"42\");") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @noflow\n\ndeclare function foo(x: number): number;\ndeclare function foo(x: string): string;\n\ndeclare var x: number | string;\n\n(foo(x): number | string);\n\ntype T = number | string;\ndeclare var y: T;\n\n(foo(y): T);\n\ndeclare class Record {\n  set(x: \"foo\", y: number): void;\n  set(x: \"bar\", y: string): void;\n}\n\nnew Record().set(\"foo\", \"42\");");
}
#[test]
fn test_test_27_js_format_1_ad0714b2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @noflow\n\ntype X = ({a:true} & {b:string}) | ({a:false} & {c:string});\n//type X = {a:true, b:string} | {a:false, c:string}; // this works.\n\nfunction hello1(x:X): string {\n  if (x.a === true) return x.b; else return x.c;\n}\n\nfunction hello2(x:X): string {\n  if (x.a === false) return x.c; else return x.b;\n}\n\nfunction hello3(x:X): string {\n  if (x.a) { return x.b; } else { return x.c; }\n}\n\nfunction hello4(x:X): string {\n  if (!x.a) { return x.c; } else { return x.b; }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @noflow\n\ntype X = ({ a: true } & { b: string }) | ({ a: false } & { c: string });\n//type X = {a:true, b:string} | {a:false, c:string}; // this works.\n\nfunction hello1(x: X): string {\n  if (x.a === true) return x.b;\n  else return x.c;\n}\n\nfunction hello2(x: X): string {\n  if (x.a === false) return x.c;\n  else return x.b;\n}\n\nfunction hello3(x: X): string {\n  if (x.a) {\n    return x.b;\n  } else {\n    return x.c;\n  }\n}\n\nfunction hello4(x: X): string {\n  if (!x.a) {\n    return x.c;\n  } else {\n    return x.b;\n  }\n}");
}
#[test]
fn test_test_29_js_format_1_af6bd9a5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @noflow\n\n// Make sure caching doesn't cause a spurious successful match (e.g., when a\n// failed match is tried again). This may happen, e.g., when checking\n// polymorphic definitions, where the same code may be checked multiple times\n// with different instantiations.\n\ntype Row = { x: string };\n\ndeclare class D<T> {\n  reduce(\n    callbackfn: (previousValue: T, currentValue: T) => T,\n    initialValue: void\n  ): T;\n  reduce<U>(\n    callbackfn: (previousValue: U, currentValue: T) => U,\n    initialValue: U\n  ): U;\n}\n\nclass C {\n  foo(\n    rows: D<Row>,\n    minWidth: number,\n  ): number {\n    return rows.reduce(\n      (length, row) => 0,\n      minWidth,\n    );\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @noflow\n\n// Make sure caching doesn't cause a spurious successful match (e.g., when a\n// failed match is tried again). This may happen, e.g., when checking\n// polymorphic definitions, where the same code may be checked multiple times\n// with different instantiations.\n\ntype Row = { x: string };\n\ndeclare class D<T> {\n  reduce(\n    callbackfn: (previousValue: T, currentValue: T) => T,\n    initialValue: void,\n  ): T;\n  reduce<U>(\n    callbackfn: (previousValue: U, currentValue: T) => U,\n    initialValue: U,\n  ): U;\n}\n\nclass C {\n  foo(rows: D<Row>, minWidth: number): number {\n    return rows.reduce((length, row) => 0, minWidth);\n  }\n}");
}
#[test]
fn test_test_30_js_format_1_f0fb81d7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @noflow\n\nconst Constants = require('./test30-helper');\n\ntype ActionType =\n  | { type: 'foo', x: number }\n  | { type: 'bar', x: number }\n\n({ type: Constants.BAR, x: 0 }: ActionType);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @noflow\n\nconst Constants = require(\"./test30-helper\");\n\ntype ActionType = { type: \"foo\", x: number } | { type: \"bar\", x: number };\n\n({ type: Constants.BAR, x: 0 }: ActionType);");
}
#[test]
fn test_test_30_helper_js_format_1_764fc7ed() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("// @noflow\n\nmodule.exports = {\n  FOO: 'foo',\n  BAR: 'bar',\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// @noflow\n\nmodule.exports = {\n  FOO: \"foo\",\n  BAR: \"bar\",\n};"
    );
}
#[test]
fn test_test_31_js_format_1_23f3d8fb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @noflow\n\n// make sure tuples are type arguments (as used e.g. when viewing maps as\n// key/value iterables) work\n\ninterface SomeIterator<T> { }\n\ninterface SomeIterable<T> {\n  it(): SomeIterator<T>;\n}\n\ndeclare class SomeMap<K,V> {\n  it(): SomeIterator<[K,V]>;\n  set(k: K, v: V): void;\n}\n\ndeclare class ImmutableMap<K,V> { }\n\ndeclare function convert<K,V>(iter: SomeIterable<[K,V]>): ImmutableMap<K,V>;\n\nfunction foo(): ImmutableMap<string, boolean> {\n  const countersGlobalMap = new SomeMap();\n  countersGlobalMap.set(\"\", false);\n  return convert(countersGlobalMap);\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @noflow\n\n// make sure tuples are type arguments (as used e.g. when viewing maps as\n// key/value iterables) work\n\ninterface SomeIterator<T> {}\n\ninterface SomeIterable<T> {\n  it(): SomeIterator<T>;\n}\n\ndeclare class SomeMap<K, V> {\n  it(): SomeIterator<[K, V]>;\n  set(k: K, v: V): void;\n}\n\ndeclare class ImmutableMap<K, V> {}\n\ndeclare function convert<K, V>(iter: SomeIterable<[K, V]>): ImmutableMap<K, V>;\n\nfunction foo(): ImmutableMap<string, boolean> {\n  const countersGlobalMap = new SomeMap();\n  countersGlobalMap.set(\"\", false);\n  return convert(countersGlobalMap);\n}");
}
#[test]
fn test_test_32_js_format_1_a23bbdba() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n// make sure that full resolution jobs don't cache improperly to signal success\n// when they have failed earlier\n\nfunction foo(value: Indirect<string> | number): Indirect<string> | number {\n  const castedValue: number = typeof value === 'number' ? value : 0;\n  return castedValue;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n// make sure that full resolution jobs don't cache improperly to signal success\n// when they have failed earlier\n\nfunction foo(value: Indirect<string> | number): Indirect<string> | number {\n  const castedValue: number = typeof value === \"number\" ? value : 0;\n  return castedValue;\n}");
}
