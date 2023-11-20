#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_standalone_end_marker_html_format_1_d4a58338() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["html"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div></div\n>\n<span></span>\n<div></div\n    \n    \n           >\n<span></span>\n<div></div\n>\n\n<span></span>\n\n<div>\n  <a href=\"#123123123123123131231312321312312312312312312312312313123123123123123\"\n    >123123123123</a\n  >\n\n  123123\n</div>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<div></div>\n<span></span>\n<div></div>\n<span></span>\n<div></div>\n\n<span></span>\n\n<div>\n  <a\n    href=\"#123123123123123131231312321312312312312312312312312313123123123123123\"\n    >123123123123</a\n  >\n\n  123123\n</div>");
}
