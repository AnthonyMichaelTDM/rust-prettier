#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_long_rule_css_format_1_ce7be55e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format (".nuclide-resize-sensitive-container-sensor,\n.nuclide-resize-sensitive-container-expand,\n.nuclide-resize-sensitive-container-shrink {\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ".nuclide-resize-sensitive-container-sensor,\n.nuclide-resize-sensitive-container-expand,\n.nuclide-resize-sensitive-container-shrink {\n}");
}
