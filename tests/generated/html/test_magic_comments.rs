use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_display_html_format_1_c58aa812() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div>\n    <!-- display: inline -->\n    <p>Long Long Long Long Long Long Long Long Long Long Long Long Long Long Long Long Long Long</p>\n</div>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<div>\n  <!-- display: inline -->\n  <p\n    >Long Long Long Long Long Long Long Long Long Long Long Long Long Long Long\n    Long Long Long</p\n  >\n</div>");
}
