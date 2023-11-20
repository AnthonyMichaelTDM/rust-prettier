#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_boolean_js_format_1_e03a9c6f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n// Boolean (the class) tests. booleans (the literals) are not part of core.js\n\nlet tests = [\n  // constructor\n  function() {\n    new Boolean();\n    new Boolean(0);\n    new Boolean(-0);\n    new Boolean(null);\n    new Boolean(false);\n    new Boolean(NaN);\n    new Boolean(undefined);\n    new Boolean(\"\");\n  },\n\n  // toString\n  function() {\n    (true).toString();\n    let x: boolean = false;\n    x.toString();\n    (new Boolean(true)).toString();\n  },\n\n  // valueOf\n  function() {\n    ((new Boolean(0)).valueOf(): boolean);\n  },\n\n  // casting\n  function() {\n    Boolean();\n    Boolean(0);\n    Boolean(-0);\n    Boolean(null);\n    Boolean(false);\n    Boolean(NaN);\n    Boolean(undefined);\n    Boolean(\"\");\n  },\n];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n// Boolean (the class) tests. booleans (the literals) are not part of core.js\n\nlet tests = [\n  // constructor\n  function () {\n    new Boolean();\n    new Boolean(0);\n    new Boolean(-0);\n    new Boolean(null);\n    new Boolean(false);\n    new Boolean(NaN);\n    new Boolean(undefined);\n    new Boolean(\"\");\n  },\n\n  // toString\n  function () {\n    true.toString();\n    let x: boolean = false;\n    x.toString();\n    new Boolean(true).toString();\n  },\n\n  // valueOf\n  function () {\n    (new Boolean(0).valueOf(): boolean);\n  },\n\n  // casting\n  function () {\n    Boolean();\n    Boolean(0);\n    Boolean(-0);\n    Boolean(null);\n    Boolean(false);\n    Boolean(NaN);\n    Boolean(undefined);\n    Boolean(\"\");\n  },\n];");
}
#[test]
fn test_map_js_format_1_51c6fac4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nfunction* generator(): Iterable<[string, number]> {\n  while (true) {\n    yield ['foo', 123];\n  }\n}\n\nlet tests = [\n  // good constructors\n  function() {\n    let w = new Map();\n    let x = new Map(null);\n    let y = new Map([['foo', 123]]);\n    let z = new Map(generator());\n    let a: Map<string, number> = new Map();\n    let b: Map<string, number> = new Map([['foo', 123]]);\n    let c: Map<string, number> = new Map(generator());\n  },\n\n  // bad constructors\n  function() {\n    let x = new Map(['foo', 123]); // error\n    let y: Map<number, string> = new Map([['foo', 123]]); // error\n  },\n\n  // get()\n  function(x: Map<string, number>) {\n    (x.get('foo'): boolean); // error, string | void\n    x.get(123); // error, wrong key type\n  },\n];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nfunction* generator(): Iterable<[string, number]> {\n  while (true) {\n    yield [\"foo\", 123];\n  }\n}\n\nlet tests = [\n  // good constructors\n  function () {\n    let w = new Map();\n    let x = new Map(null);\n    let y = new Map([[\"foo\", 123]]);\n    let z = new Map(generator());\n    let a: Map<string, number> = new Map();\n    let b: Map<string, number> = new Map([[\"foo\", 123]]);\n    let c: Map<string, number> = new Map(generator());\n  },\n\n  // bad constructors\n  function () {\n    let x = new Map([\"foo\", 123]); // error\n    let y: Map<number, string> = new Map([[\"foo\", 123]]); // error\n  },\n\n  // get()\n  function (x: Map<string, number>) {\n    (x.get(\"foo\"): boolean); // error, string | void\n    x.get(123); // error, wrong key type\n  },\n];");
}
#[test]
fn test_regexp_js_format_1_646b48d9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nlet tests = [\n  // constructor\n  function() {\n    new RegExp('foo');\n    new RegExp(/foo/);\n    new RegExp('foo', 'i');\n    new RegExp('foo', 'ig');\n    new RegExp(/foo/, 'i'); // invalid in ES5, valid in ES6\n    new RegExp(/foo/g, 'i'); // invalid in ES5, valid in ES6\n  },\n\n  // called as a function (equivalent to the constructor per ES6 21.2.3)\n  function() {\n    RegExp('foo');\n    RegExp(/foo/);\n    RegExp('foo', 'i');\n    RegExp('foo', 'ig');\n    RegExp(/foo/, 'i'); // invalid in ES5, valid in ES6\n    RegExp(/foo/g, 'i'); // invalid in ES5, valid in ES6\n  },\n\n  // invalid flags\n  function() {\n    RegExp('foo', 'z'); // error\n    new RegExp('foo', 'z'); // error\n  }\n];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nlet tests = [\n  // constructor\n  function () {\n    new RegExp(\"foo\");\n    new RegExp(/foo/);\n    new RegExp(\"foo\", \"i\");\n    new RegExp(\"foo\", \"ig\");\n    new RegExp(/foo/, \"i\"); // invalid in ES5, valid in ES6\n    new RegExp(/foo/g, \"i\"); // invalid in ES5, valid in ES6\n  },\n\n  // called as a function (equivalent to the constructor per ES6 21.2.3)\n  function () {\n    RegExp(\"foo\");\n    RegExp(/foo/);\n    RegExp(\"foo\", \"i\");\n    RegExp(\"foo\", \"ig\");\n    RegExp(/foo/, \"i\"); // invalid in ES5, valid in ES6\n    RegExp(/foo/g, \"i\"); // invalid in ES5, valid in ES6\n  },\n\n  // invalid flags\n  function () {\n    RegExp(\"foo\", \"z\"); // error\n    new RegExp(\"foo\", \"z\"); // error\n  },\n];");
}
#[test]
fn test_weakset_js_format_1_3e2d7f82() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nlet ws = new WeakSet();\nlet obj: Object = {};\nlet dict: {foo: string} = {foo: 'bar'};\n\nws.add(window);\nws.add(obj);\nws.add(dict);\nws.has(window);\nws.has(obj);\nws.has(dict);\nws.delete(window);\nws.delete(obj);\nws.delete(dict);\n\nlet ws2 = new WeakSet([obj, dict]);\n\nlet ws3 = new WeakSet([1, 2, 3]); // error, must be objects\n\nfunction* generator(): Iterable<{foo: string}> {\n  while (true) {\n    yield {foo: 'bar'};\n  }\n}\n\nlet ws4 = new WeakSet(generator());\n\nfunction* numbers(): Iterable<number> {\n  let i = 0;\n  while (true) {\n    yield i++;\n  }\n}\n\nlet ws5 = new WeakSet(numbers()); // error, must be objects") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nlet ws = new WeakSet();\nlet obj: Object = {};\nlet dict: { foo: string } = { foo: \"bar\" };\n\nws.add(window);\nws.add(obj);\nws.add(dict);\nws.has(window);\nws.has(obj);\nws.has(dict);\nws.delete(window);\nws.delete(obj);\nws.delete(dict);\n\nlet ws2 = new WeakSet([obj, dict]);\n\nlet ws3 = new WeakSet([1, 2, 3]); // error, must be objects\n\nfunction* generator(): Iterable<{ foo: string }> {\n  while (true) {\n    yield { foo: \"bar\" };\n  }\n}\n\nlet ws4 = new WeakSet(generator());\n\nfunction* numbers(): Iterable<number> {\n  let i = 0;\n  while (true) {\n    yield i++;\n  }\n}\n\nlet ws5 = new WeakSet(numbers()); // error, must be objects");
}
