#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_block_html_bracket_same_linefalse_format_1_548aa6c5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_same_line(false)
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">\ntext\n</div>\n<div long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"></div>\n<div class=\"a\">\ntext\n</div>\n<div class=\"a\">text</div>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<div\n  long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"\n>\n  text\n</div>\n<div\n  long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"\n></div>\n<div class=\"a\">text</div>\n<div class=\"a\">text</div>");
}
#[test]
fn test_block_html_bracket_same_linetrue_format_1_548aa6c5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_same_line(true)
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">\ntext\n</div>\n<div long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"></div>\n<div class=\"a\">\ntext\n</div>\n<div class=\"a\">text</div>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<div\n  long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">\n  text\n</div>\n<div\n  long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"></div>\n<div class=\"a\">text</div>\n<div class=\"a\">text</div>");
}
#[test]
fn test_embed_html_bracket_same_linefalse_format_1_c1da19d6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_same_line(false)
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">\nalert(1)</script>\n<style long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">\n.a{color: #f00}</style>\n<script>\nalert(1)</script>\n<style>\n.a{color: #f00}</style>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<script\n  long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"\n>\n  alert(1);\n</script>\n<style\n  long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"\n>\n  .a {\n    color: #f00;\n  }\n</style>\n<script>\n  alert(1);\n</script>\n<style>\n  .a {\n    color: #f00;\n  }\n</style>");
}
#[test]
fn test_embed_html_bracket_same_linetrue_format_1_c1da19d6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_same_line(true)
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">\nalert(1)</script>\n<style long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">\n.a{color: #f00}</style>\n<script>\nalert(1)</script>\n<style>\n.a{color: #f00}</style>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<script\n  long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">\n  alert(1);\n</script>\n<style\n  long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">\n  .a {\n    color: #f00;\n  }\n</style>\n<script>\n  alert(1);\n</script>\n<style>\n  .a {\n    color: #f00;\n  }\n</style>");
}
#[test]
fn test_inline_html_bracket_same_linefalse_format_1_acf1a3d6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_same_line(false)
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\n<span long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">\ntext\n</span>\n<span long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"></span>\n<span  class=\"a\">text</span>\n<span long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">\ntext\n</span>\n<span  class=\"a\">text</span><span long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">\ntext\n</span>\n<span  class=\"a\">text</span><span  class=\"a\">text</span><span  class=\"a\">text</span><span  class=\"a\">text</span><span  class=\"a\">text</span>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<span\n  long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"\n>\n  text\n</span>\n<span\n  long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"\n></span>\n<span class=\"a\">text</span>\n<span\n  long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"\n>\n  text\n</span>\n<span class=\"a\">text</span\n><span\n  long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"\n>\n  text\n</span>\n<span class=\"a\">text</span><span class=\"a\">text</span><span class=\"a\">text</span\n><span class=\"a\">text</span><span class=\"a\">text</span>");
}
#[test]
fn test_inline_html_bracket_same_linetrue_format_1_acf1a3d6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_same_line(true)
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\n<span long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">\ntext\n</span>\n<span long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"></span>\n<span  class=\"a\">text</span>\n<span long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">\ntext\n</span>\n<span  class=\"a\">text</span><span long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">\ntext\n</span>\n<span  class=\"a\">text</span><span  class=\"a\">text</span><span  class=\"a\">text</span><span  class=\"a\">text</span><span  class=\"a\">text</span>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<span\n  long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">\n  text\n</span>\n<span\n  long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"></span>\n<span class=\"a\">text</span>\n<span\n  long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">\n  text\n</span>\n<span class=\"a\">text</span\n><span\n  long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">\n  text\n</span>\n<span class=\"a\">text</span><span class=\"a\">text</span><span class=\"a\">text</span\n><span class=\"a\">text</span><span class=\"a\">text</span>");
}
#[test]
fn test_void_elements_html_bracket_same_linefalse_format_1_2fc07cc5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_same_line(false)
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<img long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\" src=\"./1.jpg\"/>\n<img src=\"./1.jpg\"/><img src=\"./1.jpg\"/><img src=\"./1.jpg\"/><img src=\"./1.jpg\"/><img src=\"./1.jpg\"/>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<img\n  long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"\n  src=\"./1.jpg\"\n/>\n<img src=\"./1.jpg\" /><img src=\"./1.jpg\" /><img src=\"./1.jpg\" /><img\n  src=\"./1.jpg\"\n/><img src=\"./1.jpg\" />");
}
#[test]
fn test_void_elements_html_bracket_same_linetrue_format_1_2fc07cc5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_same_line(true)
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<img long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\" src=\"./1.jpg\"/>\n<img src=\"./1.jpg\"/><img src=\"./1.jpg\"/><img src=\"./1.jpg\"/><img src=\"./1.jpg\"/><img src=\"./1.jpg\"/>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<img\n  long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"\n  src=\"./1.jpg\" />\n<img src=\"./1.jpg\" /><img src=\"./1.jpg\" /><img src=\"./1.jpg\" /><img\n  src=\"./1.jpg\" /><img src=\"./1.jpg\" />");
}
