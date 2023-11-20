#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_custom_properties_css_format_1_27447fbc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["css"])
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format(":root{--mainColor:#123456}\na{color:var(        --mainColor     )     ; }");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        ":root {\n  --mainColor: #123456;\n}\na {\n  color: var(--mainColor);\n}"
    );
}
