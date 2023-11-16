#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_missing_comments_md_format_1_dd29e688() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("\\`\\`\\`\\`\\`\\`missing-comments\n        This should not be replaced.\n\\`\\`\\`\\`\\`\\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "\\`\\`\\`missing-comments\n        This should not be replaced.\n\\`\\`\\`"
    );
}
