#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_class_with_generics_js_format_1_db4232ee() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import React from 'react';\n\n/*:: type Props = {\n  foo?: ?string,\n  bar: number,\n}; */\n\n/*:: type State = { baz: number }; */\n\nclass Component extends React.Component/*:: <Props, State> */ {\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import React from \"react\";\n\ntype Props = {\n  foo?: ?string,\n  bar: number,\n};\ntype State = { baz: number };\nclass Component extends React.Component<Props, State> {}");
}
#[test]
fn test_constructor_field_override_js_format_1_6b5b7fbe() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://github.com/prettier/prettier/issues/1481\nclass Foo {\n  constructor: () => this;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// https://github.com/prettier/prettier/issues/1481\nclass Foo {\n  constructor: () => this;\n}");
}
#[test]
fn test_functions_js_format_1_7aed1dad() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\nexport function updateStoreFromURL(\n  store /*: Store*/,\n  {search, hash} /*: {search: string, hash: string}*/\n) {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export function updateStoreFromURL(\n  store: Store,\n  { search, hash }: { search: string, hash: string },\n) {}");
}
