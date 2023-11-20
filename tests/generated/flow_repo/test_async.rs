#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_async_js_format_1_ca0497be() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n// \"For async functions, a Promise<T> is returned,\n// and the type of return expressions must be T.\"\n//\n\nasync function f0(): Promise<number> {\n  return 1;\n}\n\nasync function f1(): Promise<bool> {\n  return 1;  // error, number != bool\n}\n\n// await: (p: Promise<T> | T) => T\n//\n\nasync function f2(p: Promise<number>): Promise<number> {\n  var x: number = await p;\n  var y: number = await 1;\n  return x + y;\n}\n\nasync function f3(p: Promise<number>): Promise<number> {\n  return await p;\n}\n\n// TODO: this is one of those bad generic errors, currently:\n// \"inconsistent use of library definitions\" with two core.js locs\nasync function f4(p: Promise<number>): Promise<bool> {\n  return await p; // error, number != bool\n}\n\n// async arrow functions\n//\n\nvar f5: () => Promise<number> = async () => await 1;\n\n// async methods\n//\n\nclass C {\n  async m() { return 1; }\n  async mt<T>(a: T): Promise<T> { return a; }\n  static async m(a): void { await a; } // error, void != Promise<void>\n  static async mt<T>(a: T): Promise<T> { return a; }\n}\n\n// async function props\n\nvar obj = { f: async () => await 1 };\nvar objf : () => Promise<number> = obj.f;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n// \"For async functions, a Promise<T> is returned,\n// and the type of return expressions must be T.\"\n//\n\nasync function f0(): Promise<number> {\n  return 1;\n}\n\nasync function f1(): Promise<boolean> {\n  return 1; // error, number != bool\n}\n\n// await: (p: Promise<T> | T) => T\n//\n\nasync function f2(p: Promise<number>): Promise<number> {\n  var x: number = await p;\n  var y: number = await 1;\n  return x + y;\n}\n\nasync function f3(p: Promise<number>): Promise<number> {\n  return await p;\n}\n\n// TODO: this is one of those bad generic errors, currently:\n// \"inconsistent use of library definitions\" with two core.js locs\nasync function f4(p: Promise<number>): Promise<boolean> {\n  return await p; // error, number != bool\n}\n\n// async arrow functions\n//\n\nvar f5: () => Promise<number> = async () => await 1;\n\n// async methods\n//\n\nclass C {\n  async m() {\n    return 1;\n  }\n  async mt<T>(a: T): Promise<T> {\n    return a;\n  }\n  static async m(a): void {\n    await a;\n  } // error, void != Promise<void>\n  static async mt<T>(a: T): Promise<T> {\n    return a;\n  }\n}\n\n// async function props\n\nvar obj = { f: async () => await 1 };\nvar objf: () => Promise<number> = obj.f;");
}
#[test]
fn test_async_base_class_js_format_1_f643badc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// This is kind of weird, but it should parse. This works in babel without the\n// parens around (await promise). From the es6 and async/await specs I (nmote)\n// am not clear on whether it should. In any case it's a strange corner case\n// that is probably not important to support.\nclass C {};\n\nvar P: Promise<Class<C>> = new Promise(function (resolve, reject) {\n  resolve(C);\n});\n\nasync function foo() {\n  class Bar extends (await P) { }\n  return Bar;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// This is kind of weird, but it should parse. This works in babel without the\n// parens around (await promise). From the es6 and async/await specs I (nmote)\n// am not clear on whether it should. In any case it's a strange corner case\n// that is probably not important to support.\nclass C {}\n\nvar P: Promise<Class<C>> = new Promise(function (resolve, reject) {\n  resolve(C);\n});\n\nasync function foo() {\n  class Bar extends (await P) {}\n  return Bar;\n}");
}
#[test]
fn test_async_parse_js_format_1_77443f03() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("async function f() {}\nasync function ft<T>(a: T) {}\n\nclass C {\n  async m() {}\n  async mt<T>(a: T) {}\n  static async m(a) {}\n  static async mt<T>(a: T) {}\n}\n\nvar e = async function () {};\nvar et = async function<T> (a: T) {};\n\nvar n = new async function() {};\n\nvar o = { async m() {} };\nvar ot = { async m<T>(a: T) {} };\nvar oz = { async async() {} };\n\nvar x = { async : 5 };\nconsole.log(x.async);\n\nvar async = 3;\nvar y = { async };") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "async function f() {}\nasync function ft<T>(a: T) {}\n\nclass C {\n  async m() {}\n  async mt<T>(a: T) {}\n  static async m(a) {}\n  static async mt<T>(a: T) {}\n}\n\nvar e = async function () {};\nvar et = async function <T>(a: T) {};\n\nvar n = new (async function () {})();\n\nvar o = { async m() {} };\nvar ot = { async m<T>(a: T) {} };\nvar oz = { async async() {} };\n\nvar x = { async: 5 };\nconsole.log(x.async);\n\nvar async = 3;\nvar y = { async };");
}
#[test]
fn test_async_promise_js_format_1_29a796b4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("async function f(): Promise<number> {\n  return Promise.resolve(1);\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "async function f(): Promise<number> {\n  return Promise.resolve(1);\n}"
    );
}
#[test]
fn test_async_return_void_js_format_1_b833b340() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nasync function foo1(): Promise<string> {\n  return;\n}\n\nasync function foo2(): Promise<string> {\n  return undefined;\n}\n\nasync function foo3(): Promise<string> {\n  function bar() { }\n  return bar();\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nasync function foo1(): Promise<string> {\n  return;\n}\n\nasync function foo2(): Promise<string> {\n  return undefined;\n}\n\nasync function foo3(): Promise<string> {\n  function bar() {}\n  return bar();\n}");
}
#[test]
fn test_async_2_js_format_1_4c984d5e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n// misc basic\n\nfunction test1() {\n  async function foo() {\n    return 42;\n  }\n\n  async function bar() {\n    var a = await foo();\n    var b: number = a; // valid\n    var c: string = a; // Error: number ~> string\n  }\n}\n\n//\n// void returns:\n//\n\n// inference should produce return type Promise<void>\n// in the absence of an explicit return\n//\n\nfunction test2() {\n  async function voidoid1() {\n    console.log(\"HEY\");\n  }\n\n  var voidoid2: () => Promise<void> = voidoid1; // ok\n\n  var voidoid3: () => void = voidoid1; // error, void != Promise<void>\n}\n\n// annotated return type of Promise<void> should work\n//\n\nfunction test3() {\n  async function voidoid4(): Promise<void> { // ok\n    console.log(\"HEY\");\n  }\n}\n\n// other annotated return types should fail\n// (note: misannotated return types with explicit\n// return statements are covered in async.js)\n//\n\nfunction test4() {\n  async function voidoid5(): void { // error, void != Promise<void>\n    console.log(\"HEY\");\n  }\n}\n\nfunction test5() {\n  async function voidoid6()\n  : Promise<number> { // error, number != void\n    console.log(\"HEY\");\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n// misc basic\n\nfunction test1() {\n  async function foo() {\n    return 42;\n  }\n\n  async function bar() {\n    var a = await foo();\n    var b: number = a; // valid\n    var c: string = a; // Error: number ~> string\n  }\n}\n\n//\n// void returns:\n//\n\n// inference should produce return type Promise<void>\n// in the absence of an explicit return\n//\n\nfunction test2() {\n  async function voidoid1() {\n    console.log(\"HEY\");\n  }\n\n  var voidoid2: () => Promise<void> = voidoid1; // ok\n\n  var voidoid3: () => void = voidoid1; // error, void != Promise<void>\n}\n\n// annotated return type of Promise<void> should work\n//\n\nfunction test3() {\n  async function voidoid4(): Promise<void> {\n    // ok\n    console.log(\"HEY\");\n  }\n}\n\n// other annotated return types should fail\n// (note: misannotated return types with explicit\n// return statements are covered in async.js)\n//\n\nfunction test4() {\n  async function voidoid5(): void {\n    // error, void != Promise<void>\n    console.log(\"HEY\");\n  }\n}\n\nfunction test5() {\n  async function voidoid6(): Promise<number> {\n    // error, number != void\n    console.log(\"HEY\");\n  }\n}");
}
#[test]
fn test_async_3_js_format_1_bc6eb1ab() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n/**\n * test nested-promise unwrapping.\n * Note: currently we don't do this properly in the underlying\n * type of the Promise class, which causes spurious errors to\n * be raised here. Once that's fixed, the errors here will go\n * away.\n */\n\nasync function foo() {\n  return 42;\n}\n\nasync function bar() {\n  return foo();\n}\n\nasync function baz() {\n\n  // a should now be typed as number, but is in fact\n  // Promise<number> until nested-promise unwrap is fixed\n  var a = await bar();\n\n  // TODO this is valid code, but currently gives Promise ~> number error\n  // due to lack of transitive Promise unwrap.\n  var b: number = a;\n\n  // should be number ~> string error, but currently gives\n  // Promise ~> string error for the same reason\n  var c: string = a;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n/**\n * test nested-promise unwrapping.\n * Note: currently we don't do this properly in the underlying\n * type of the Promise class, which causes spurious errors to\n * be raised here. Once that's fixed, the errors here will go\n * away.\n */\n\nasync function foo() {\n  return 42;\n}\n\nasync function bar() {\n  return foo();\n}\n\nasync function baz() {\n  // a should now be typed as number, but is in fact\n  // Promise<number> until nested-promise unwrap is fixed\n  var a = await bar();\n\n  // TODO this is valid code, but currently gives Promise ~> number error\n  // due to lack of transitive Promise unwrap.\n  var b: number = a;\n\n  // should be number ~> string error, but currently gives\n  // Promise ~> string error for the same reason\n  var c: string = a;\n}");
}
#[test]
fn test_await_parse_js_format_1_1c53a874() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("async function f() { await 1; }\nasync function ft<T>(a: T) { await 1; }\n\nclass C {\n  async m() { await 1; }\n  async mt<T>(a: T) { await 1; }\n  static async m(a) { await 1; }\n  static async mt<T>(a: T) { await 1; }\n}\n\nvar e = async function () { await 1; };\nvar et = async function<T> (a: T) { await 1; };\n\nvar n = new async function() { await 1; };\n\nvar o = { async m() { await 1; } };\nvar ot = { async m<T>(a: T) { await 1; } };\nvar oz = { async async(async) { await async; } };\n\nvar x = { await : 5 };\nconsole.log(x.await);\n\nvar await = 3;\nvar y = { await };") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "async function f() {\n  await 1;\n}\nasync function ft<T>(a: T) {\n  await 1;\n}\n\nclass C {\n  async m() {\n    await 1;\n  }\n  async mt<T>(a: T) {\n    await 1;\n  }\n  static async m(a) {\n    await 1;\n  }\n  static async mt<T>(a: T) {\n    await 1;\n  }\n}\n\nvar e = async function () {\n  await 1;\n};\nvar et = async function <T>(a: T) {\n  await 1;\n};\n\nvar n = new (async function () {\n  await 1;\n})();\n\nvar o = {\n  async m() {\n    await 1;\n  },\n};\nvar ot = {\n  async m<T>(a: T) {\n    await 1;\n  },\n};\nvar oz = {\n  async async(async) {\n    await async;\n  },\n};\n\nvar x = { await: 5 };\nconsole.log(x.await);\n\nvar await = 3;\nvar y = { await };");
}
