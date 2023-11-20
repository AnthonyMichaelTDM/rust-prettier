#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_empty_with_no_trailing_newline_hbs_format_1_68b329da() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["glimmer"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_empty_with_trailing_newline_hbs_format_1_e1c06d85() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["glimmer"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_end_of_line_last_hbs_format_1_5c4dd62a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["glimmer"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a bare string to hightlight eol last handling");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "a bare string to hightlight eol last handlin");
}
