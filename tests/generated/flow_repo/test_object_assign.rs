#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_c1f35594() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\nvar EventEmitter = require('events').EventEmitter;\n\n// This pattern seems to cause the trouble.\nvar Bad = Object.assign({}, EventEmitter.prototype, {\n  foo: function(): string { return 'hi'; }\n});\n\n// Calling Bad.foo() in the same file doesn't error\nvar bad: number = Bad.foo();\n\n// Doesn't repro if I extend the class myself\nclass MyEventEmitter extends events$EventEmitter {}\nvar Good = Object.assign({}, MyEventEmitter.prototype, {\n  foo: function(): string { return 'hi'; }\n});\n// Calling Good.foo() in the same file doesn't error\nvar good: number = Good.foo();\n\nmodule.exports = {\n  Bad: Bad,\n  Good: Good,\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @flow\n */\n\nvar EventEmitter = require(\"events\").EventEmitter;\n\n// This pattern seems to cause the trouble.\nvar Bad = Object.assign({}, EventEmitter.prototype, {\n  foo: function (): string {\n    return \"hi\";\n  },\n});\n\n// Calling Bad.foo() in the same file doesn't error\nvar bad: number = Bad.foo();\n\n// Doesn't repro if I extend the class myself\nclass MyEventEmitter extends events$EventEmitter {}\nvar Good = Object.assign({}, MyEventEmitter.prototype, {\n  foo: function (): string {\n    return \"hi\";\n  },\n});\n// Calling Good.foo() in the same file doesn't error\nvar good: number = Good.foo();\n\nmodule.exports = {\n  Bad: Bad,\n  Good: Good,\n};");
}
#[test]
fn test_b_js_format_1_ab9e1654() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\nvar A = require('./A.js');\n\nvar good: number = A.Good.foo();\n\nvar f = A.Bad.foo; // Property access is fine\nvar bad_: number = f(); // Calling the function is fine\n\nvar bad: number = A.Bad.foo(); // Method call is not fine\n/*\nB.js|12 col 1 error|  call of method foo\n|| Property not found in\nA.js|8 col 23 error|  object literal\n*/") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @flow\n */\n\nvar A = require(\"./A.js\");\n\nvar good: number = A.Good.foo();\n\nvar f = A.Bad.foo; // Property access is fine\nvar bad_: number = f(); // Calling the function is fine\n\nvar bad: number = A.Bad.foo(); // Method call is not fine\n/*\nB.js|12 col 1 error|  call of method foo\n|| Property not found in\nA.js|8 col 23 error|  object literal\n*/");
}
#[test]
fn test_apply_js_format_1_13079b6a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n(Object.assign.apply(null, [{}, {a: 1}, {b: 'foo'}]): {a: number, b: string});") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n(Object.assign.apply(null, [{}, { a: 1 }, { b: \"foo\" }]): {\n  a: number,\n  b: string,\n});");
}
#[test]
fn test_non_objects_js_format_1_04af8e2f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nObject.assign(\"123\", {a: \"foo\"});\nObject.assign(123, {a: \"foo\"});\nObject.assign({a: \"foo\"}, 123);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nObject.assign(\"123\", { a: \"foo\" });\nObject.assign(123, { a: \"foo\" });\nObject.assign({ a: \"foo\" }, 123);");
}
#[test]
fn test_spread_js_format_1_3a2402d4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\ndeclare var arrOfObjs: Array<{ foo: string }>;\ndeclare var roArrOfObjs: $ReadOnlyArray<{foo: string}>;\ndeclare var tup: [{foo: string}, {bar: number}];\n\n(Object.assign({}, ...arrOfObjs): { foo: number}); // Error: string ~> number\n(Object.assign({}, ...roArrOfObjs): { foo: number}); // Error: string ~> number\n(Object.assign({}, ...tup): { foo: string, bar: boolean}); // Error: number ~> boolean\n\n(Object.assign(\n  {},\n  ...[{a: 1}, {b: 'foo'}],\n  ...[{c: true}],\n): {a: number, b: true, c: boolean}); // Error: 'foo' => true") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\ndeclare var arrOfObjs: Array<{ foo: string }>;\ndeclare var roArrOfObjs: $ReadOnlyArray<{ foo: string }>;\ndeclare var tup: [{ foo: string }, { bar: number }];\n\n(Object.assign({}, ...arrOfObjs): { foo: number }); // Error: string ~> number\n(Object.assign({}, ...roArrOfObjs): { foo: number }); // Error: string ~> number\n(Object.assign({}, ...tup): { foo: string, bar: boolean }); // Error: number ~> boolean\n\n(Object.assign({}, ...[{ a: 1 }, { b: \"foo\" }], ...[{ c: true }]): {\n  a: number,\n  b: true,\n  c: boolean,\n}); // Error: 'foo' => true");
}
#[test]
fn test_undefined_js_format_1_638638cb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nvar React = require('react');\n\ntype DefaultProps = {\n  foo: number,\n}\n\ntype Props = {\n  foo: number,\n}\n\nclass MyReactThing extends React.Component {\n  props: Props;\n  static defaultProps: DefaultProps;\n  getFoo(): number { return this.props.foo; }\n}\n\n<MyReactThing />; // works\n<MyReactThing foo={undefined} />; // also works") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nvar React = require(\"react\");\n\ntype DefaultProps = {\n  foo: number,\n};\n\ntype Props = {\n  foo: number,\n};\n\nclass MyReactThing extends React.Component {\n  props: Props;\n  static defaultProps: DefaultProps;\n  getFoo(): number {\n    return this.props.foo;\n  }\n}\n\n<MyReactThing />; // works\n<MyReactThing foo={undefined} />; // also works");
}
