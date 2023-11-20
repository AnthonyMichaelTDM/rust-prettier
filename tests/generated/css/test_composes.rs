#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_composes_css_format_1_bf133962() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format (".reference {\n  composes: selector from \"a/long/file/path/exceeding/the/maximum/length/forcing/a/line-wrap/file.css\";\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ".reference {\n  composes: selector from \"a/long/file/path/exceeding/the/maximum/length/forcing/a/line-wrap/file.css\";\n}");
}
