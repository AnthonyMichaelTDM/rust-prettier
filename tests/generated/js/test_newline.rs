#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_backslash_2028_js_format_1_f328a571() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1;/*a*///b\u{2028}/*c*/2");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "1; /*a*/ //b\n/*c*/ 2;");
}
#[test]
fn test_backslash_2029_js_format_1_a3a3bbba() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1;/*a*///b\u{2029}/*c*/2");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "1; /*a*/ //b\n/*c*/ 2;");
}
