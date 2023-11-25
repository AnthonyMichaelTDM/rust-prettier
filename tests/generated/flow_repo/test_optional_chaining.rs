#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_private_properties_js_format_1_f9f07536() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nclass X {\n  #foo: number\n  constructor() {\n    (this?.#foo: empty);\n  }\n};\n\nclass Y {\n  #bar: X\n  #baz: ?X\n  constructor() {\n    (this?.#bar: empty);\n    (this?.#baz: empty);\n  }\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nclass X {\n  #foo: number;\n  constructor() {\n    (this?.#foo: empty);\n  }\n}\n\nclass Y {\n  #bar: X;\n  #baz: ?X;\n  constructor() {\n    (this?.#bar: empty);\n    (this?.#baz: empty);\n  }\n}");
}
