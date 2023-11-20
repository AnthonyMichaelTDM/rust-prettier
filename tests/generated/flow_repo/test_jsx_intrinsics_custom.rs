#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_main_js_format_1_5507bc87() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nvar React = require('react');\n\nclass CustomComponent extends React.Component {\n  props: {\n    prop: string\n  };\n}\n\nvar a: React.Element<{prop: string}> = <CustomComponent prop=\"asdf\" />;\nvar b: React.Element<{prop1: string}> = <CustomComponent prop=\"asdf\" />; // Error: Props<{prop}> ~> Props<{prop1}>\n\n<div id=\"asdf\" />;\n<div id={42} />; // Error: (\\`id\\` prop) number ~> string\nvar c: React.Element<{id: string}> = <div id=\"asdf\" />;\nvar d: React.Element<{id: number}> = <div id=\"asdf\" />; // Error: Props<{id:string}> ~> Props<{id:number}>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nvar React = require(\"react\");\n\nclass CustomComponent extends React.Component {\n  props: {\n    prop: string,\n  };\n}\n\nvar a: React.Element<{ prop: string }> = <CustomComponent prop=\"asdf\" />;\nvar b: React.Element<{ prop1: string }> = <CustomComponent prop=\"asdf\" />; // Error: Props<{prop}> ~> Props<{prop1}>\n\n<div id=\"asdf\" />;\n<div id={42} />; // Error: (\\`id\\` prop) number ~> string\nvar c: React.Element<{ id: string }> = <div id=\"asdf\" />;\nvar d: React.Element<{ id: number }> = <div id=\"asdf\" />; // Error: Props<{id:string}> ~> Props<{id:number}>");
}
#[test]
fn test_strings_js_format_1_da4e4a31() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nvar React = require('react');\n\nvar Div = 'div';\nvar Bad = 'bad';\nvar Str: string = 'str';\n\n<Div />; // This is fine\n<Bad />; // Error: 'bad' not in JSXIntrinsics\n<Str />; // Error: string ~> keys of JSXIntrinsics\n\nReact.createElement('div', {}); // This is fine\nReact.createElement('bad', {}); // Error: 'bad' not in JSXIntrinsics\nReact.createElement(Str, {}); // Error: string ~> keys of JSXIntrinsics\n\n// TODO: Make this an error\n<Div id={42} />; // Not an error but should be eventually") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nvar React = require(\"react\");\n\nvar Div = \"div\";\nvar Bad = \"bad\";\nvar Str: string = \"str\";\n\n<Div />; // This is fine\n<Bad />; // Error: 'bad' not in JSXIntrinsics\n<Str />; // Error: string ~> keys of JSXIntrinsics\n\nReact.createElement(\"div\", {}); // This is fine\nReact.createElement(\"bad\", {}); // Error: 'bad' not in JSXIntrinsics\nReact.createElement(Str, {}); // Error: string ~> keys of JSXIntrinsics\n\n// TODO: Make this an error\n<Div id={42} />; // Not an error but should be eventually");
}
