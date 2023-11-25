use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_implements_js_format_1_0a0df1d4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A implements B {}\nclass A implements B, C {}\ndeclare class A implements B {}\ndeclare class A mixins B implements C {}\ndeclare class A implements B, C {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class A implements B {}\nclass A implements B, C {}\ndeclare class A implements B {}\ndeclare class A mixins B implements C {}\ndeclare class A implements B, C {}");
}
