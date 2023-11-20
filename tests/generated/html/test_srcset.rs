#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_invalid_html_format_1_9ae5c7b1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<img src=\"a\"\nsrcset=\"\n should-not-format  400w 100h,\n       should-not-format  500w 200h\n\"\n alt=\"\"/>\n\n<img src=\"a\"\nsrcset=\"\n should-not-format ,, should-not-format 0q,,,\n\"\n alt=\"\"/>\n\n<img src=\"a\"\nsrcset=\",,,\"/>\n\n<img src=\"a\"\nsrcset=\"   \"/>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<img\n  src=\"a\"\n  srcset=\"\n should-not-format  400w 100h,\n       should-not-format  500w 200h\n\"\n  alt=\"\"\n/>\n\n<img\n  src=\"a\"\n  srcset=\"\n should-not-format ,, should-not-format 0q,,,\n\"\n  alt=\"\"\n/>\n\n<img src=\"a\" srcset=\",,,\" />\n\n<img src=\"a\" srcset=\"   \" />");
}
