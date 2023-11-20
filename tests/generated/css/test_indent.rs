#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_indent_css_format_1_7bedeec0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("div {\n  background: var(fig-light-02) url(/images/inset-shadow-east-ltr.png) 100% 0 repeat-y;\n  box-shadow: 0 0 1px 2px rgba(88, 144, 255, 0.75), 0 1px 1px rgba(0, 0, 0, 0.15);\n  padding-bottom: calc(var(ads-help-tray-footer-with-support-link-height) + var(ads-help-tray-header-height-new));\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "div {\n  background: var(fig-light-02) url(/images/inset-shadow-east-ltr.png) 100% 0\n    repeat-y;\n  box-shadow:\n    0 0 1px 2px rgba(88, 144, 255, 0.75),\n    0 1px 1px rgba(0, 0, 0, 0.15);\n  padding-bottom: calc(\n    var(ads-help-tray-footer-with-support-link-height) +\n      var(ads-help-tray-header-height-new)\n  );\n}");
}
#[test]
fn test_selectors_css_format_1_00587573() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("a {\n  ~ .Pagination-itemWrapper:not(.is-separator):not([data-priority^='#{$priority}'])\n    ~ .Pagination-itemWrapper.is-separator[data-priority^='#{$priority}'] {\n    display: flex;\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "a {\n  ~ .Pagination-itemWrapper:not(.is-separator):not(\n      [data-priority^=\"#{$priority}\"]\n    )\n    ~ .Pagination-itemWrapper.is-separator[data-priority^=\"#{$priority}\"] {\n    display: flex;\n  }\n}");
}
