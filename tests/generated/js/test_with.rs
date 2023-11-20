#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_indent_js_format_1_6cded8a9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel", "flow", "typescript"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("with (0) {}\n\nwith (0) 1;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "with (0) {\n}\n\nwith (0) 1;");
}
