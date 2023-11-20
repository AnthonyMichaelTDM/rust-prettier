#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_main_js_format_1_ea9780ae() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nvar React = require('react');\n\nclass CustomComponent extends React.Component {\n  props: {\n    prop: string\n  };\n}\n\nvar a: React.Element<{prop: string}> = <CustomComponent prop=\"asdf\" />;\nvar b: React.Element<{prop1: string}> = <CustomComponent prop=\"asdf\" />; // Error: Props<{prop}> ~> Props<{prop1}>\n\n// Since intrinsics are typed as \\`any\\` out of the box, we can pass any\n// attributes to intrinsics!\nvar c: React.Element<any> = <div not_a_real_attr=\"asdf\" />;\n// However, we don't allow such elements to be viewed as React elements with\n// different attributes.\nvar d: React.Element<{doesntmatch: string}> = <div not_a_real_attr=\"asdf\" />;\n// No error as long as expectations are consistent, though.\nvar e: React.Element<{not_a_real_attr: string}> = <div not_a_real_attr=\"asdf\" />;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nvar React = require(\"react\");\n\nclass CustomComponent extends React.Component {\n  props: {\n    prop: string,\n  };\n}\n\nvar a: React.Element<{ prop: string }> = <CustomComponent prop=\"asdf\" />;\nvar b: React.Element<{ prop1: string }> = <CustomComponent prop=\"asdf\" />; // Error: Props<{prop}> ~> Props<{prop1}>\n\n// Since intrinsics are typed as \\`any\\` out of the box, we can pass any\n// attributes to intrinsics!\nvar c: React.Element<any> = <div not_a_real_attr=\"asdf\" />;\n// However, we don't allow such elements to be viewed as React elements with\n// different attributes.\nvar d: React.Element<{ doesntmatch: string }> = <div not_a_real_attr=\"asdf\" />;\n// No error as long as expectations are consistent, though.\nvar e: React.Element<{ not_a_real_attr: string }> = (\n  <div not_a_real_attr=\"asdf\" />\n);");
}
#[test]
fn test_strings_js_format_1_b832dbaa() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nvar React = require('react');\n\n// The builtin $JSXIntrinsics should allow any string\n\nvar Div = 'div';\nvar Bad = 'bad';\nvar Str: string = 'str';\n\n<Div />; // This is fine\n<Bad />; // This is fine\n<Str />; // This is fine\n\nReact.createElement('div', {}); // This is fine\nReact.createElement('bad', {}); // This is fine\n\n<Div id={42} />; // This is fine") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nvar React = require(\"react\");\n\n// The builtin $JSXIntrinsics should allow any string\n\nvar Div = \"div\";\nvar Bad = \"bad\";\nvar Str: string = \"str\";\n\n<Div />; // This is fine\n<Bad />; // This is fine\n<Str />; // This is fine\n\nReact.createElement(\"div\", {}); // This is fine\nReact.createElement(\"bad\", {}); // This is fine\n\n<Div id={42} />; // This is fine");
}
