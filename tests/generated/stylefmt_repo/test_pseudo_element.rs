#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_pseudo_element_css_format_1_034520f2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("a:after  { content:\"\" }\na::before  { content:\"\" }\n");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "a:after {\n  content: \"\";\n}\na::before {\n  content: \"\";\n}"
    );
}
