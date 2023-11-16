#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_test_js_format_1_5738e003() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nclass Foo {\n  annotationOnly: string;\n  initOnly = 'asdf';\n  initWithAnnotation: string = 'asdf';\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nclass Foo {\n  annotationOnly: string;\n  initOnly = \"asdf\";\n  initWithAnnotation: string = \"asdf\";\n}");
}
