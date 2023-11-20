#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_loose_css_format_1_ff3b5107() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("div {height: calc(-0.5 * var(ei-table-cell-padding));\n  width: -webkit-calc\n(100% + 20px);\n  margin: -moz-calc(\n100%\n-\n320px\n);\n  background: url(\n    var( audience-network-checkbox-image)\n    ) center no-repeat;\n  background-image: url(\n    )\n    center center\n    no-repeat\n    black;}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "div {\n  height: calc(-0.5 * var(ei-table-cell-padding));\n  width: -webkit-calc (100% + 20px);\n  margin: -moz-calc(100% - 320px);\n  background: url(var(audience-network-checkbox-image)) center no-repeat;\n  background-image: url() center center no-repeat black;\n}");
}
