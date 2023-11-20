#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_common_yml_format_1_a4425fb7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- &abc a\n- *abc");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- &abc a\n- *abc");
}
