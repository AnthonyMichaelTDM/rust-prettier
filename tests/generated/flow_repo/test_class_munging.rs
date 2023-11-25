use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_with_munging_js_format_1_56b06937() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\nclass Foo {\n  _method(): string {\n    return 'this is private';\n  }\n}\n\nclass Bar extends Foo {\n  test() {\n    (this._method(): string); // error\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @flow\n */\n\nclass Foo {\n  _method(): string {\n    return \"this is private\";\n  }\n}\n\nclass Bar extends Foo {\n  test() {\n    (this._method(): string); // error\n  }\n}");
}
#[test]
fn test_without_munging_js_format_1_f41f7a2f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n * @preventMunge\n */\n\nclass Foo {\n  _method(): string {\n    return 'this is not private';\n  }\n}\n\nclass Bar extends Foo {\n  test() {\n    (this._method(): string); // ok\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @flow\n * @preventMunge\n */\n\nclass Foo {\n  _method(): string {\n    return \"this is not private\";\n  }\n}\n\nclass Bar extends Foo {\n  test() {\n    (this._method(): string); // ok\n  }\n}");
}
