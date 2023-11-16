#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_parent_js_format_1_bf1852ba() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer
        .format("// @flow\n\nvar ParentFoo = {foo: 'bar'};\nmodule.exports = {ParentFoo};");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// @flow\n\nvar ParentFoo = { foo: \"bar\" };\nmodule.exports = { ParentFoo };"
    );
}
#[test]
fn test_main_js_format_1_76df567a() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nvar Parent = require('./Parent');\n\n// Hops through destructuring\nlet ParentFoo;\n({ParentFoo} = Parent);\nParentFoo; // Points to lval in line above this\n\n// Follows assignment on simple/\"non-destructuring\" patterns\nlet ParentFoo2;\nParentFoo2 = Parent;\nParentFoo2; // Points to LHS of line above this\n\n// Follows assignment with declaration\nlet ParentFoo3 = Parent;\nParentFoo3; // Points to LHS of line above this\n\n// Follows non-destructured property access of \\`require('Parent')\\`\nlet foo = require('./Parent').ParentFoo.foo;\nfoo;\n\nimport type {Foo} from './types';\nfunction takesFoo(foo: Foo) { }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nvar Parent = require(\"./Parent\");\n\n// Hops through destructuring\nlet ParentFoo;\n({ ParentFoo } = Parent);\nParentFoo; // Points to lval in line above this\n\n// Follows assignment on simple/\"non-destructuring\" patterns\nlet ParentFoo2;\nParentFoo2 = Parent;\nParentFoo2; // Points to LHS of line above this\n\n// Follows assignment with declaration\nlet ParentFoo3 = Parent;\nParentFoo3; // Points to LHS of line above this\n\n// Follows non-destructured property access of \\`require('Parent')\\`\nlet foo = require(\"./Parent\").ParentFoo.foo;\nfoo;\n\nimport type { Foo } from \"./types\";\nfunction takesFoo(foo: Foo) {}");
}
#[test]
fn test_override_js_format_1_da660050() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nclass C {\n  override() { }\n}\n\nclass D extends C {\n  foo() { this.override() }\n  bar() { this.override }\n  override() { }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nclass C {\n  override() {}\n}\n\nclass D extends C {\n  foo() {\n    this.override();\n  }\n  bar() {\n    this.override;\n  }\n  override() {}\n}");
}
#[test]
fn test_react_js_format_1_943e6864() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("var React = require('react');\n\nclass C extends React.Component {\n  props: { x: string };\n}\n\nlet msg = \"hello\";\n\n(<C x={msg}/>);\n\n(<div id={msg}/>);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var React = require(\"react\");\n\nclass C extends React.Component {\n  props: { x: string };\n}\n\nlet msg = \"hello\";\n\n<C x={msg} />;\n\n<div id={msg} />;");
}
#[test]
fn test_types_js_format_1_0ccaeaf8() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("// @flow\n\nexport type Foo = {};");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "// @flow\n\nexport type Foo = {};");
}
