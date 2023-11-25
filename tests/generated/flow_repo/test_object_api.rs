#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_1797b2f9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* @flow */\n\nmodule.exports = { a() {} }")?;
    assert_eq!(formatted, "/* @flow */\n\nmodule.exports = { a() {} };");
    Ok(())
}
#[test]
fn test_b_js_format_1_16b4c86c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nvar a = require('./a');\nvar b = Object.assign({ bar() {}, ...{} }, a);\nb.a(); // works here\nmodule.exports = b;") ? ;
    assert_eq ! (formatted , "/* @flow */\n\nvar a = require(\"./a\");\nvar b = Object.assign({ bar() {}, ...{} }, a);\nb.a(); // works here\nmodule.exports = b;");
    Ok(())
}
#[test]
fn test_c_js_format_1_974bbcb1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("/* @flow */\n\nvar c = require('./b');\nc.a();\nc.foo()")?;
    assert_eq!(
        formatted,
        "/* @flow */\n\nvar c = require(\"./b\");\nc.a();\nc.foo();"
    );
    Ok(())
}
#[test]
fn test_object_assign_js_format_1_5687dd0f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nvar export_ = Object.assign({}, {\n    foo: function(param) { return param; }\n});\n\nvar decl_export_: { foo: any; bar: any } = Object.assign({}, export_);\n\nlet anyObj: Object = {};\nObject.assign(anyObj, anyObj); // makes sure this terminates\n\nmodule.exports = export_;") ? ;
    assert_eq ! (formatted , "/* @flow */\n\nvar export_ = Object.assign(\n  {},\n  {\n    foo: function (param) {\n      return param;\n    },\n  },\n);\n\nvar decl_export_: { foo: any, bar: any } = Object.assign({}, export_);\n\nlet anyObj: Object = {};\nObject.assign(anyObj, anyObj); // makes sure this terminates\n\nmodule.exports = export_;");
    Ok(())
}
#[test]
fn test_object_create_js_format_1_059ed901() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nclass C { foo: string; }\n\n// OK, `instanceof C` would be true\n(Object.create(C.prototype): C);\n\n// OK, `instanceof C` would be true\n(Object.create(new C): C);\n\n// error, object literals don't structurally match instances\n({ foo: \"foo\" }: C);\n\n// error, object types don't structurally match instances\ntype O = { foo: string; }\ndeclare var o: O;\n(o: C);") ? ;
    assert_eq ! (formatted , "/* @flow */\n\nclass C {\n  foo: string;\n}\n\n// OK, `instanceof C` would be true\n(Object.create(C.prototype): C);\n\n// OK, `instanceof C` would be true\n(Object.create(new C()): C);\n\n// error, object literals don't structurally match instances\n({ foo: \"foo\" }: C);\n\n// error, object types don't structurally match instances\ntype O = { foo: string };\ndeclare var o: O;\n(o: C);");
    Ok(())
}
#[test]
fn test_object_getprototypeof_js_format_1_47672445() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nclass Foo {}\nclass Bar extends Foo {}\n\nlet tests = [\n  function() {\n    const x = new Bar();\n    (Object.getPrototypeOf(x): Foo);\n  },\n];") ? ;
    assert_eq ! (formatted , "// @flow\n\nclass Foo {}\nclass Bar extends Foo {}\n\nlet tests = [\n  function () {\n    const x = new Bar();\n    (Object.getPrototypeOf(x): Foo);\n  },\n];");
    Ok(())
}
#[test]
fn test_object_keys_js_format_1_83e10af2() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nvar sealed = {one: 'one', two: 'two'};\n(Object.keys(sealed): Array<'one'|'two'>);\n(Object.keys(sealed): void); // error, Array<string>\n\nvar unsealed = {};\nObject.keys(unsealed).forEach(k => {\n  (k : number) // error: string ~> number\n});\n\nvar dict: { [k: number]: string } = {};\nObject.keys(dict).forEach(k => {\n  (k : number) // error: string ~> number\n});\n\nvar any: Object = {};\n(Object.keys(any): Array<number>); // error, Array<string>\n\nclass Foo {\n  prop: string;\n  foo() {}\n}\n// constructor and foo not enumerable\n(Object.keys(new Foo()): Array<'error'>); // error: prop ~> error\n\nclass Bar extends Foo {\n  bar_prop: string;\n  bar() {}\n}\n// only own enumerable props\n(Object.keys(new Bar()): Array<'error'>); // error: bar_prop ~> error") ? ;
    assert_eq ! (formatted , "/* @flow */\n\nvar sealed = { one: \"one\", two: \"two\" };\n(Object.keys(sealed): Array<\"one\" | \"two\">);\n(Object.keys(sealed): void); // error, Array<string>\n\nvar unsealed = {};\nObject.keys(unsealed).forEach((k) => {\n  (k: number); // error: string ~> number\n});\n\nvar dict: { [k: number]: string } = {};\nObject.keys(dict).forEach((k) => {\n  (k: number); // error: string ~> number\n});\n\nvar any: Object = {};\n(Object.keys(any): Array<number>); // error, Array<string>\n\nclass Foo {\n  prop: string;\n  foo() {}\n}\n// constructor and foo not enumerable\n(Object.keys(new Foo()): Array<\"error\">); // error: prop ~> error\n\nclass Bar extends Foo {\n  bar_prop: string;\n  bar() {}\n}\n// only own enumerable props\n(Object.keys(new Bar()): Array<\"error\">); // error: bar_prop ~> error");
    Ok(())
}
#[test]
fn test_object_missing_js_format_1_a4fb5c9a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "// @flow\n\nlet tests = [\n  function() {\n    Object.doesNotExist();\n  },\n];",
    )?;
    assert_eq!(
        formatted,
        "// @flow\n\nlet tests = [\n  function () {\n    Object.doesNotExist();\n  },\n];"
    );
    Ok(())
}
#[test]
fn test_object_prototype_js_format_1_60c95de3() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nfunction takesABool(x: boolean) {}\nfunction takesAString(x: string) {}\nfunction takesANumber(x: number) {}\nfunction takesAnObject(x: Object) {}\n\nclass Foo {}\n\nvar a = { foo: 'bar' };\nvar b = { foo: 'bar', ...{}};\nvar c = { foo: 'bar', toString: function(): number { return 123; }};\nvar d : { [key: string]: string } = { foo: 'bar' };\nvar x = new Date();\nvar y = new Foo();\n\n//\n// toString\n//\n\n// call\ntakesAString(a.toString());\nd.toString(); // ok, even though dict specifies strings, this is a function\n\n// get\nvar aToString : () => string = a.toString;\nvar aToString2 = a.toString;\ntakesAString(aToString2());\n\n// set\nb.toString = function(): string { return 'foo'; };\nc.toString = function(): number { return 123; };\n\n// override\nvar cToString : () => number = c.toString;\n\n// ... on a built-in instance\nvar xToString : number = x.toString; // error\nvar xToString2 : () => number = x.toString; // error\ntakesAString(x.toString());\n\n// ... on an instance\nvar yToString : number = y.toString; // error\ntakesAString(y.toString());\n\n// ... on a primitive\n(123).toString();\n(123).toString;\n(123).toString = function() {}; // error\n(123).toString(2);\n(123).toString('foo'); // error\n(123).toString(null); // error\n\n\n//\n// hasOwnProperty\n//\n\n// call\ntakesABool(a.hasOwnProperty('foo'));\n\n// get\nvar aHasOwnProperty : (prop: string) => boolean = a.hasOwnProperty;\nvar aHasOwnProperty2 = a.hasOwnProperty;\ntakesABool(aHasOwnProperty2('bar'));\n\n// set\nb.hasOwnProperty = function() { return false; };\n\n// ... on a built-in instance\nvar xHasOwnProperty : number = x.hasOwnProperty; // error\nvar xHasOwnProperty2 : (prop: string) => number = x.hasOwnProperty; // error\ntakesABool(x.hasOwnProperty('foo'));\n\n// ... on an instance\nvar yHasOwnProperty : number = y.hasOwnProperty; // error\ntakesABool(y.hasOwnProperty('foo'));\n\n\n//\n// propertyIsEnumerable\n//\n\n// call\ntakesABool(a.propertyIsEnumerable('foo'));\n\n// get\nvar aPropertyIsEnumerable : (prop: string) => boolean = a.propertyIsEnumerable;\nvar aPropertyIsEnumerable2 = a.propertyIsEnumerable;\ntakesABool(aPropertyIsEnumerable2('bar'));\n\n// set\nb.propertyIsEnumerable = function() { return false; };\n\n// ... on a built-in instance\nvar xPropertyIsEnumerable : number = x.propertyIsEnumerable; // error\nvar xPropertyIsEnumerable2 : (prop: string) => number =\n  x.propertyIsEnumerable; // error\ntakesABool(x.propertyIsEnumerable('foo'));\n\n// ... on an instance\nvar yPropertyIsEnumerable : number = y.propertyIsEnumerable; // error\ntakesABool(y.propertyIsEnumerable('foo'));\n\n\n//\n// valueOf\n//\n\n// call\ntakesAnObject(a.valueOf());\n\n// get\nvar aValueOf : () => Object = a.valueOf;\nvar aValueOf2 = a.valueOf;\ntakesAnObject(aValueOf2());\n\n// set\nb.valueOf = function() { return {}; };\n\n// ... on a built-in instance\nvar xValueOf : number = x.valueOf; // error\ntakesANumber(x.valueOf());\n\n// ... on an instance\nvar yValueOf : number = y.valueOf; // error\ntakesAnObject(y.valueOf());\n\n// ... on a literal\nvar strValueOf : string = (\"foo\").valueOf();\nvar numValueOf : number = (123).valueOf();\nvar boolValueOf : boolean = (true).valueOf();\n\n//\n// toLocaleString\n//\n\n// call\ntakesAString(a.toLocaleString());\n\n// get\nvar aToLocaleString : () => string = a.toLocaleString;\nvar aToLocaleString2 = a.toLocaleString;\ntakesAString(aToLocaleString2());\n\n// set\nb.toLocaleString = function() { return 'derp'; };\n\n// ... on a built-in instance\nvar xToLocaleString : number = x.toLocaleString; // error\nvar xToLocaleString2 : () => number = x.toLocaleString; // error\ntakesAString(x.toLocaleString());\n\n// ... on an instance\nvar yToLocaleString : number = y.toLocaleString; // error\ntakesAString(y.toLocaleString());\n\n\n//\n// constructor\n//\n\nvar k : Object = a.constructor;\n(123).constructor;") ? ;
    assert_eq ! (formatted , "/* @flow */\n\nfunction takesABool(x: boolean) {}\nfunction takesAString(x: string) {}\nfunction takesANumber(x: number) {}\nfunction takesAnObject(x: Object) {}\n\nclass Foo {}\n\nvar a = { foo: \"bar\" };\nvar b = { foo: \"bar\", ...{} };\nvar c = {\n  foo: \"bar\",\n  toString: function (): number {\n    return 123;\n  },\n};\nvar d: { [key: string]: string } = { foo: \"bar\" };\nvar x = new Date();\nvar y = new Foo();\n\n//\n// toString\n//\n\n// call\ntakesAString(a.toString());\nd.toString(); // ok, even though dict specifies strings, this is a function\n\n// get\nvar aToString: () => string = a.toString;\nvar aToString2 = a.toString;\ntakesAString(aToString2());\n\n// set\nb.toString = function (): string {\n  return \"foo\";\n};\nc.toString = function (): number {\n  return 123;\n};\n\n// override\nvar cToString: () => number = c.toString;\n\n// ... on a built-in instance\nvar xToString: number = x.toString; // error\nvar xToString2: () => number = x.toString; // error\ntakesAString(x.toString());\n\n// ... on an instance\nvar yToString: number = y.toString; // error\ntakesAString(y.toString());\n\n// ... on a primitive\n(123).toString();\n(123).toString;\n(123).toString = function () {}; // error\n(123).toString(2);\n(123).toString(\"foo\"); // error\n(123).toString(null); // error\n\n//\n// hasOwnProperty\n//\n\n// call\ntakesABool(a.hasOwnProperty(\"foo\"));\n\n// get\nvar aHasOwnProperty: (prop: string) => boolean = a.hasOwnProperty;\nvar aHasOwnProperty2 = a.hasOwnProperty;\ntakesABool(aHasOwnProperty2(\"bar\"));\n\n// set\nb.hasOwnProperty = function () {\n  return false;\n};\n\n// ... on a built-in instance\nvar xHasOwnProperty: number = x.hasOwnProperty; // error\nvar xHasOwnProperty2: (prop: string) => number = x.hasOwnProperty; // error\ntakesABool(x.hasOwnProperty(\"foo\"));\n\n// ... on an instance\nvar yHasOwnProperty: number = y.hasOwnProperty; // error\ntakesABool(y.hasOwnProperty(\"foo\"));\n\n//\n// propertyIsEnumerable\n//\n\n// call\ntakesABool(a.propertyIsEnumerable(\"foo\"));\n\n// get\nvar aPropertyIsEnumerable: (prop: string) => boolean = a.propertyIsEnumerable;\nvar aPropertyIsEnumerable2 = a.propertyIsEnumerable;\ntakesABool(aPropertyIsEnumerable2(\"bar\"));\n\n// set\nb.propertyIsEnumerable = function () {\n  return false;\n};\n\n// ... on a built-in instance\nvar xPropertyIsEnumerable: number = x.propertyIsEnumerable; // error\nvar xPropertyIsEnumerable2: (prop: string) => number = x.propertyIsEnumerable; // error\ntakesABool(x.propertyIsEnumerable(\"foo\"));\n\n// ... on an instance\nvar yPropertyIsEnumerable: number = y.propertyIsEnumerable; // error\ntakesABool(y.propertyIsEnumerable(\"foo\"));\n\n//\n// valueOf\n//\n\n// call\ntakesAnObject(a.valueOf());\n\n// get\nvar aValueOf: () => Object = a.valueOf;\nvar aValueOf2 = a.valueOf;\ntakesAnObject(aValueOf2());\n\n// set\nb.valueOf = function () {\n  return {};\n};\n\n// ... on a built-in instance\nvar xValueOf: number = x.valueOf; // error\ntakesANumber(x.valueOf());\n\n// ... on an instance\nvar yValueOf: number = y.valueOf; // error\ntakesAnObject(y.valueOf());\n\n// ... on a literal\nvar strValueOf: string = \"foo\".valueOf();\nvar numValueOf: number = (123).valueOf();\nvar boolValueOf: boolean = true.valueOf();\n\n//\n// toLocaleString\n//\n\n// call\ntakesAString(a.toLocaleString());\n\n// get\nvar aToLocaleString: () => string = a.toLocaleString;\nvar aToLocaleString2 = a.toLocaleString;\ntakesAString(aToLocaleString2());\n\n// set\nb.toLocaleString = function () {\n  return \"derp\";\n};\n\n// ... on a built-in instance\nvar xToLocaleString: number = x.toLocaleString; // error\nvar xToLocaleString2: () => number = x.toLocaleString; // error\ntakesAString(x.toLocaleString());\n\n// ... on an instance\nvar yToLocaleString: number = y.toLocaleString; // error\ntakesAString(y.toLocaleString());\n\n//\n// constructor\n//\n\nvar k: Object = a.constructor;\n(123).constructor;");
    Ok(())
}
