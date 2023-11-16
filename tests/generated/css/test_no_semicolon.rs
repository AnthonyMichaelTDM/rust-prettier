#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_url_css_format_1_63793fd5() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer
        .format("@import ur\n  l(//fonts.googleapis.com/css?family=Open+Sans:400,400italic);");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "@import ur\n  l(//fonts.googleapis.com/css?family=Open+Sans:400,400italic);"
    );
}
