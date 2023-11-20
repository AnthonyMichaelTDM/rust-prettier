#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_import_css_format_1_8fc2400c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("        @import         \"./settings\"    ;\n\n\n@import      \"./components\" ;@import \"./themes\";\n.class{float: left;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@import \"./settings\";\n\n@import \"./components\";\n@import \"./themes\";\n.class {\n  float: left;\n}");
}
