#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_rest_ts_format_1_23dbfde6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["typescript"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("function test([first, ...rest]) {}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "function test([first, ...rest]) {}");
}
