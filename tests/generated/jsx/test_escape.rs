#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_escape_js_format_1_fc9c4bca() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<div>&lt;</div>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<div>&lt;</div>;");
}
#[test]
fn test_nbsp_js_format_1_7d402304() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("many_nbsp = <div>&nbsp; &nbsp; </div>\nsingle_nbsp = <div>&nbsp;</div>\nnbsp_with_newline =\n  <div>\n    &nbsp;\n  </div>\n\nmany_raw_nbsp = <div>\u{a0}\u{a0}\u{a0}</div>\nmany_raw_spaces = <div>   </div>\n\n\namp = <span>foo &amp; bar</span>\nraw_amp = <span>foo & bar</span>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "many_nbsp = <div>&nbsp; &nbsp; </div>;\nsingle_nbsp = <div>&nbsp;</div>;\nnbsp_with_newline = <div>&nbsp;</div>;\n\nmany_raw_nbsp = <div>\u{a0}\u{a0}\u{a0}</div>;\nmany_raw_spaces = <div> </div>;\n\namp = <span>foo &amp; bar</span>;\nraw_amp = <span>foo & bar</span>;");
}
