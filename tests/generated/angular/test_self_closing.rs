#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_self_closing_component_html_format_1_3bccd400() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("component.html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<app-test/>\n<app-test />\n<app-test\n/>\n<img>\n<img/>\n<img />\n<img\n/>\n<div/>\n<div />\n<div\n/>") ? ;
    assert_eq ! (formatted , "<app-test />\n<app-test />\n<app-test />\n<img />\n<img />\n<img />\n<img />\n<div />\n<div />\n<div />");
    Ok(())
}
