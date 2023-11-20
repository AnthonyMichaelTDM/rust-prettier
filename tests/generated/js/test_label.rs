#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_block_statement_and_regexp_js_format_1_f4ecd9e8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a : { b : 1 }/2/;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "a: {\n  b: 1;\n}\n/2/;");
}
#[test]
fn test_comment_js_format_1_6697c7ea() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{\n  inf_leave: // goto emulation\n  for (;;) {}\n}\n{\n  inf_leave:\n  // goto emulation\n  for (; ;) { }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{\n  // goto emulation\n  inf_leave: for (;;) {}\n}\n{\n  // goto emulation\n  inf_leave: for (;;) {}\n}");
}
#[test]
fn test_empty_label_js_format_1_ac9375a4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a:;\nb");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "a:;\nb;");
}
