#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_selector_less_format_1_2d4aead5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("less")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("@supports selector(\n:focus-visible // *\"a\"\n) {\na{color:#f00\n}\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "@supports selector(\n  :focus-visible // *\"a\"\n) {\n  a {\n    color: #f00;\n  }\n}"
    );
}
