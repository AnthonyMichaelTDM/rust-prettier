#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_element_node_hbs_format_1_3d0fef9a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div class=\"attribute\" {{modifier}} {{! comment}}>\n  Hello\n</div>\n\n<div>\n  Hello\n</div>\n\n<div>\n  hi\n</div>\n\n<div>\n  A long enough string to trigger a line break that would prevent wrapping.\n</div>\n\n<div>\n  A long enough string to trigger a line break that would prevent wrapping more.\n</div>\n\n<div>\n  A long enough string to trigger a line break that would prevent wrapping more and more.\n</div>\n\n<div>\n  {{#block}}\n    {{hello}}\n  {{/block}}\n</div>\n\n<div>\n  {{hello}}\n</div>\n\n<div></div>\n<img />\n\n<MyComponent @prop={{true}} @prop2={{true}} @prop3={{true}} @prop4={{true}} as |thing|></MyComponent>\n\n<div />\n<div></div>\n<custom-component />\n<custom-component></custom-component>\n<i />\n<i></i>\n<Component />\n<Component></Component>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<div class=\"attribute\" {{modifier}} {{! comment}}>\n  Hello\n</div>\n\n<div>\n  Hello\n</div>\n\n<div>\n  hi\n</div>\n\n<div>\n  A long enough string to trigger a line break that would prevent wrapping.\n</div>\n\n<div>\n  A long enough string to trigger a line break that would prevent wrapping more.\n</div>\n\n<div>\n  A long enough string to trigger a line break that would prevent wrapping more\n  and more.\n</div>\n\n<div>\n  {{#block}}\n    {{hello}}\n  {{/block}}\n</div>\n\n<div>\n  {{hello}}\n</div>\n\n<div></div>\n<img />\n\n<MyComponent\n  @prop={{true}}\n  @prop2={{true}}\n  @prop3={{true}}\n  @prop4={{true}}\n  as |thing|\n/>\n\n<div />\n<div></div>\n<custom-component />\n<custom-component></custom-component>\n<i />\n<i></i>\n<Component />\n<Component /");
}
#[test]
fn test_snippet_basefont_format_1_341b909e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<basefont> text </basefont");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<basefont> text </basefont");
}
#[test]
fn test_snippet_bgsound_format_1_4b5da402() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<bgsound> text </bgsound");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<bgsound> text </bgsound");
}
#[test]
fn test_snippet_frame_format_1_359c31d7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<frame> text </frame");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<frame> text </frame");
}
#[test]
fn test_snippet_image_format_1_05640651() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<image> text </image");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<image> text </image");
}
#[test]
fn test_snippet_isindex_format_1_2e3fa7b5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<isindex> text </isindex");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<isindex> text </isindex");
}
#[test]
fn test_snippet_menuitem_format_1_3344aed6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<menuitem> text </menuitem");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<menuitem> text </menuitem");
}
#[test]
fn test_snippet_nextid_format_1_8bc95893() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<nextid> text </nextid");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<nextid> text </nextid");
}
#[test]
fn test_void_elements_hbs_format_1_b3fcd7b4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<img>\n<input>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<img />\n<input /");
}
