use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_url_less_format_1_63793fd5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("less")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("@import ur\n  l(//fonts.googleapis.com/css?family=Open+Sans:400,400italic);");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "@import ur\n  l(//fonts.googleapis.com/css?family=Open+Sans:400,400italic);"
    );
}
