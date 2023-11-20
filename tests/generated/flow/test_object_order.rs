#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_order_js_trailing_commaall_format_1_62f98dce() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Foo = {\n  // a\n  alpha: 'hello',\n\n  // b\n  [key: string]: void,\n  // c\n\n  beta: 10,\n\n  // d\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type Foo = {\n  // a\n  alpha: \"hello\",\n\n  // b\n  [key: string]: void,\n  // c\n\n  beta: 10,\n\n  // d\n};");
}
#[test]
fn test_order_js_trailing_commaes_5_format_1_62f98dce() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .trailing_comma("es5")
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Foo = {\n  // a\n  alpha: 'hello',\n\n  // b\n  [key: string]: void,\n  // c\n\n  beta: 10,\n\n  // d\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type Foo = {\n  // a\n  alpha: \"hello\",\n\n  // b\n  [key: string]: void,\n  // c\n\n  beta: 10,\n\n  // d\n};");
}
