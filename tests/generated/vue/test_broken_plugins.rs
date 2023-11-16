#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_missing_comments_vue_format_1_cc4b3068() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format(
        "<template lang=\"missing-comments\">\n      This should not be replaced.</template>",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<template lang=\"missing-comments\">\n      This should not be replaced.</template>"
    );
}
