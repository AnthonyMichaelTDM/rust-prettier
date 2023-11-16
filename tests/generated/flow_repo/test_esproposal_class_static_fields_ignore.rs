#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_test_js_format_1_7cbc68a8() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nclass Foo {\n  static annotationOnly: string;\n  static initOnly = 'asdf';\n  static initWithAnnotation: string = 'asdf';\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nclass Foo {\n  static annotationOnly: string;\n  static initOnly = \"asdf\";\n  static initWithAnnotation: string = \"asdf\";\n}");
}
