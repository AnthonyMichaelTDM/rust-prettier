#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_literal_js_format_1_a8687761() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "typescript", "flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("100n\n9223372036854775807n\n0o16432n\n0O16432n\n0xFFF123n\n0XFFF123n\n0b101011101n\n0B101011101n\n200_000n\n0x0000_000An\n0b0111_1111n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "100n;\n9223372036854775807n;\n0o16432n;\n0o16432n;\n0xfff123n;\n0xfff123n;\n0b101011101n;\n0b101011101n;\n200_000n;\n0x0000_000an;\n0b0111_1111n;");
}
