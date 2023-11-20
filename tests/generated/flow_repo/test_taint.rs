#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_adder_js_format_1_20ecb8b5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nfunction f(x : $Tainted<number>, y : $Tainted<number>) {\n  var z : $Tainted<number> = x + y;\n}\nfunction f1(x : $Tainted<number>, y : number) {\n  var z : $Tainted<number> = x + y;\n}\nfunction f2(x : number, y : $Tainted<number>) {\n  var z : $Tainted<number> = x + y;\n}\n// This should cause an error.\nfunction f3(x : $Tainted<number>, y : number) {\n  var z : number = x + y;\n}\n// This should cause an error.\nfunction f4(x : number, y : $Tainted<number>) {\n  var z : number = x + y;\n}\n// This should cause an error.\nfunction f5(x : number, y : $Tainted<number>) {\n  var z : string = x + y;\n}\n// This should cause an error.\nfunction f6(x : string, y : $Tainted<number>) {\n  var z : string = x + y;\n}\n// This should cause an error.\nfunction f7(x : $Tainted<string>, y : $Tainted<number>) {\n  var z : string = x + y;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nfunction f(x: $Tainted<number>, y: $Tainted<number>) {\n  var z: $Tainted<number> = x + y;\n}\nfunction f1(x: $Tainted<number>, y: number) {\n  var z: $Tainted<number> = x + y;\n}\nfunction f2(x: number, y: $Tainted<number>) {\n  var z: $Tainted<number> = x + y;\n}\n// This should cause an error.\nfunction f3(x: $Tainted<number>, y: number) {\n  var z: number = x + y;\n}\n// This should cause an error.\nfunction f4(x: number, y: $Tainted<number>) {\n  var z: number = x + y;\n}\n// This should cause an error.\nfunction f5(x: number, y: $Tainted<number>) {\n  var z: string = x + y;\n}\n// This should cause an error.\nfunction f6(x: string, y: $Tainted<number>) {\n  var z: string = x + y;\n}\n// This should cause an error.\nfunction f7(x: $Tainted<string>, y: $Tainted<number>) {\n  var z: string = x + y;\n}");
}
#[test]
fn test_any_object_js_format_1_d4f206a5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nlet tests = [\n  // setting a property\n  function(x: $Tainted<string>, y: string) {\n    let obj: Object = {};\n    obj.foo = x; // error, taint ~> any\n    obj[y] = x; // error, taint ~> any\n  },\n\n  // getting a property\n  function() {\n    let obj: Object = { foo: 'foo' };\n    (obj.foo: $Tainted<string>); // ok\n  },\n\n  // calling a method\n  function(x: $Tainted<string>) {\n    let obj: Object = {};\n    obj.foo(x); // error, taint ~> any\n\n    let foo = obj.foo;\n    foo(x); // error, taint ~> any\n  },\n];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nlet tests = [\n  // setting a property\n  function (x: $Tainted<string>, y: string) {\n    let obj: Object = {};\n    obj.foo = x; // error, taint ~> any\n    obj[y] = x; // error, taint ~> any\n  },\n\n  // getting a property\n  function () {\n    let obj: Object = { foo: \"foo\" };\n    (obj.foo: $Tainted<string>); // ok\n  },\n\n  // calling a method\n  function (x: $Tainted<string>) {\n    let obj: Object = {};\n    obj.foo(x); // error, taint ~> any\n\n    let foo = obj.foo;\n    foo(x); // error, taint ~> any\n  },\n];");
}
#[test]
fn test_call_object_property_js_format_1_f1625b21() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nfunction foo(x : $Tainted<string>, o : Object) {\n  // Error\n  o.f(x);\n}\nfunction foo1(x : $Tainted<string>, o : {f : (y : $Tainted<string>) => void}) {\n  o.f(x);\n}\nfunction foo2(o1 : Object, o2 : {t : $Tainted<string>}) {\n  o1.f(o2.t);\n}\nfunction foo3<T>(x : $Tainted<T>, o : {f : (y : $Tainted<T>) => void}) {\n  o.f(x);\n}\nfunction f_foo1(x : $Tainted<string>, f : Function) {\n  // Error\n  f(x);\n}\nfunction f_foo2(f1 : Function, o : {t : $Tainted<string>}) {\n  f1(o.t);\n}\nfunction f_foo3(f1 : Function, o1 : Object, o2 : {t : $Tainted<string>}) {\n  (f1(o1))(o2.t);\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nfunction foo(x: $Tainted<string>, o: Object) {\n  // Error\n  o.f(x);\n}\nfunction foo1(x: $Tainted<string>, o: { f: (y: $Tainted<string>) => void }) {\n  o.f(x);\n}\nfunction foo2(o1: Object, o2: { t: $Tainted<string> }) {\n  o1.f(o2.t);\n}\nfunction foo3<T>(x: $Tainted<T>, o: { f: (y: $Tainted<T>) => void }) {\n  o.f(x);\n}\nfunction f_foo1(x: $Tainted<string>, f: Function) {\n  // Error\n  f(x);\n}\nfunction f_foo2(f1: Function, o: { t: $Tainted<string> }) {\n  f1(o.t);\n}\nfunction f_foo3(f1: Function, o1: Object, o2: { t: $Tainted<string> }) {\n  f1(o1)(o2.t);\n}");
}
#[test]
fn test_comparator_js_format_1_b1e422ed() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n// Should cause an error.\nfunction f(x : $Tainted<string>, y : $Tainted<number>) {\n  var z : $Tainted<bool> = x < y;\n}\n// Should cause an error.\nfunction f1(x : string, y : $Tainted<number>) {\n  var z : $Tainted<bool> = x < y;\n}\n// Should cause an error.\nfunction f2(x : $Tainted<string>, y : number) {\n  var z : $Tainted<bool> = x < y;\n}\n// Note: We allow removing Taint when two tainted\n// values are compared.\nfunction f3(x : $Tainted<string>, y : string) {\n  var z : bool = x < y;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n// Should cause an error.\nfunction f(x: $Tainted<string>, y: $Tainted<number>) {\n  var z: $Tainted<boolean> = x < y;\n}\n// Should cause an error.\nfunction f1(x: string, y: $Tainted<number>) {\n  var z: $Tainted<boolean> = x < y;\n}\n// Should cause an error.\nfunction f2(x: $Tainted<string>, y: number) {\n  var z: $Tainted<boolean> = x < y;\n}\n// Note: We allow removing Taint when two tainted\n// values are compared.\nfunction f3(x: $Tainted<string>, y: string) {\n  var z: boolean = x < y;\n}");
}
#[test]
fn test_function_js_format_1_68aa1336() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nlet tests = [\n  // flows any to each param\n  function(x: any, y: $Tainted<string>) {\n    x(y); // error, taint ~> any\n  },\n\n  // calling \\`any\\` returns \\`any\\`\n  function(x: any, y: $Tainted<string>) {\n    let z = x();\n    z(y);\n  }\n];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nlet tests = [\n  // flows any to each param\n  function (x: any, y: $Tainted<string>) {\n    x(y); // error, taint ~> any\n  },\n\n  // calling \\`any\\` returns \\`any\\`\n  function (x: any, y: $Tainted<string>) {\n    let z = x();\n    z(y);\n  },\n];");
}
#[test]
fn test_globals_js_format_1_63d7eee1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nclass A {\n  f(x : $Tainted<FakeLocation>) {\n    fakeDocument.location = x; // error\n    doStuff(x); // ok\n  }\n  f1(x : $Tainted<FakeLocation>) {\n    // TODO(rcastano): should cause an error.\n    window.fakeLocation = x;\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nclass A {\n  f(x: $Tainted<FakeLocation>) {\n    fakeDocument.location = x; // error\n    doStuff(x); // ok\n  }\n  f1(x: $Tainted<FakeLocation>) {\n    // TODO(rcastano): should cause an error.\n    window.fakeLocation = x;\n  }\n}");
}
#[test]
fn test_lib_js_format_1_afc87fab() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare class FakeLocation {\n  assign(url: string): void;\n}\n\ndeclare class FakeDocument {\n  location: FakeLocation;\n}\n\ndeclare function doStuff(x: $Tainted<any>): void;\n\ndeclare var fakeDocument: FakeDocument;\ndeclare var fakeLocation: FakeLocation;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare class FakeLocation {\n  assign(url: string): void;\n}\n\ndeclare class FakeDocument {\n  location: FakeLocation;\n}\n\ndeclare function doStuff(x: $Tainted<any>): void;\n\ndeclare var fakeDocument: FakeDocument;\ndeclare var fakeLocation: FakeLocation;");
}
#[test]
fn test_taint_1_js_format_1_c108c233() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/*\n *\n * @flow\n */\nclass A {\n  f(tainted : $Tainted<string>) {\n    // This shouldn't give a warning (both are tainted)\n    var also_tainted : $Tainted<string> = tainted;\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/*\n *\n * @flow\n */\nclass A {\n  f(tainted: $Tainted<string>) {\n    // This shouldn't give a warning (both are tainted)\n    var also_tainted: $Tainted<string> = tainted;\n  }\n}");
}
#[test]
fn test_taint_2_js_format_1_6cd8c349() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/*\n *\n * @flow\n */\nclass A {\n  f(tainted : $Tainted<string>) {\n    // This *should* give a warning.\n    fakeDocument.location.assign(tainted);\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/*\n *\n * @flow\n */\nclass A {\n  f(tainted: $Tainted<string>) {\n    // This *should* give a warning.\n    fakeDocument.location.assign(tainted);\n  }\n}");
}
#[test]
fn test_taint_3_js_format_1_63d3481b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/*\n *\n * @flow\n */\nclass A {\n  f(tainted : $Tainted<string>) {\n    // The Tainted annotation should still flow.\n    var safe = tainted;\n    // This should give a warning.\n    var loc : string = safe;\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/*\n *\n * @flow\n */\nclass A {\n  f(tainted: $Tainted<string>) {\n    // The Tainted annotation should still flow.\n    var safe = tainted;\n    // This should give a warning.\n    var loc: string = safe;\n  }\n}");
}
#[test]
fn test_taint_4_js_format_1_d37da97e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/*\n *\n * @flow\n */\n\nvar safe : string = \"safe\";\n// This should be allowed.\nvar tainted : $Tainted<string> = safe\n\nfunction f(x : $Tainted<any>) {\n  // Should cause error.\n  var y : any = x;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/*\n *\n * @flow\n */\n\nvar safe: string = \"safe\";\n// This should be allowed.\nvar tainted: $Tainted<string> = safe;\n\nfunction f(x: $Tainted<any>) {\n  // Should cause error.\n  var y: any = x;\n}");
}
#[test]
fn test_use_types_js_format_1_3a8a9e23() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/*\n * @flow\n */\n\n// Should cause an error.\nfunction foo (x : $Tainted<number>) {\n  var should_fail : number = x * 42;\n}\n// Should cause an error.\nfunction foo1 (x : $Tainted<{f: number}>) {\n  var ok : number = x.f;\n}\n// Should cause an error.\nfunction foo2 (o : {f (y:number):number}, t: $Tainted<number>) {\n  return o.f(t);\n}\n\nfunction foo3 (x : $Tainted<{f: number}>) {\n  var also_tainted : $Tainted<number> = x.f;\n}\n// Should cause an error.\nfunction foo4 (a : $Tainted<Array<string>>) {\n  var trusted : string = a[0];\n}\n// Type error.\nfunction foo5 (a : $Tainted<Array<string>>) {\n  var trusted_number : number = a[0];\n}\n\nfunction foo6 (a : $Tainted<Array<string>>) {\n  var trusted : $Tainted<string> = a[0];\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/*\n * @flow\n */\n\n// Should cause an error.\nfunction foo(x: $Tainted<number>) {\n  var should_fail: number = x * 42;\n}\n// Should cause an error.\nfunction foo1(x: $Tainted<{ f: number }>) {\n  var ok: number = x.f;\n}\n// Should cause an error.\nfunction foo2(o: { f(y: number): number }, t: $Tainted<number>) {\n  return o.f(t);\n}\n\nfunction foo3(x: $Tainted<{ f: number }>) {\n  var also_tainted: $Tainted<number> = x.f;\n}\n// Should cause an error.\nfunction foo4(a: $Tainted<Array<string>>) {\n  var trusted: string = a[0];\n}\n// Type error.\nfunction foo5(a: $Tainted<Array<string>>) {\n  var trusted_number: number = a[0];\n}\n\nfunction foo6(a: $Tainted<Array<string>>) {\n  var trusted: $Tainted<string> = a[0];\n}");
}
