#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_param_js_trailing_commaall_format_1_cd1d258e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .trailing_comma("all")
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const f = ({}: MyVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongType) => {};\nfunction g({}: Foo) {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const f =\n  ({}: MyVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongType) => {};\nfunction g({}: Foo) {}");
}
#[test]
fn test_param_js_trailing_commaes_5_format_1_cd1d258e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .trailing_comma("es5")
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const f = ({}: MyVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongType) => {};\nfunction g({}: Foo) {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const f =\n  ({}: MyVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongType) => {};\nfunction g({}: Foo) {}");
}
