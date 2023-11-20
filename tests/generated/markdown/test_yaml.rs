#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_complex_md_prose_wrapalways_format_1_01facc8f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .prose_wrap("always")
        .parsers(vec!["markdown"])
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("---\n- hello:    world\n-         123\n---\n\n# something");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "---\n- hello: world\n- 123\n---\n\n# something");
}
#[test]
fn test_empty_md_prose_wrapalways_format_1_4b12a98d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\n---");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "---\n---");
}
#[test]
fn test_empty_2_md_prose_wrapalways_format_1_00327d87() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .prose_wrap("always")
        .parsers(vec!["markdown"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\n---\n\nContent");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "---\n---\n\nContent");
}
#[test]
fn test_simple_md_prose_wrapalways_format_1_d199ddb9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .prose_wrap("always")
        .parsers(vec!["markdown"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\nhello: world\n---");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "---\nhello: world\n---");
}
#[test]
fn test_simple_2_md_prose_wrapalways_format_1_b7c4c886() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\nhello: world\n---\n\nContent");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "---\nhello: world\n---\n\nContent");
}
#[test]
fn test_trailing_spaces_md_prose_wrapalways_format_1_cb3662ac() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("---\n           v spaces\n---         \n\nThis paragraph should be considered part of the _markdown_ instead of *yaml*.\n\n---") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "---\nv spaces\n---\n\nThis paragraph should be considered part of the _markdown_ instead of _yaml_.\n\n---");
}
