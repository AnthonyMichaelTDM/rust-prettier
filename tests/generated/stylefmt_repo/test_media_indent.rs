use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_media_indent_css_format_1_3eeccb15() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format (".foo {\n    @media(max-width:1000px)\n    {\n        display: none;\n          }\n        }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        ".foo {\n  @media (max-width: 1000px) {\n    display: none;\n  }\n}"
    );
}
