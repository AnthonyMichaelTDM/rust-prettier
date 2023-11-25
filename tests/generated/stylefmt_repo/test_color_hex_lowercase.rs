use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_color_hex_lowercase_css_format_1_f457f0d2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(".foo{color:#ABCDEF}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, ".foo {\n  color: #abcdef;\n}");
}
