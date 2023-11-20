#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_before_text_html_html_whitespace_sensitivityignore_format_1_6e0a6ccf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .html_whitespace_sensitivity("ignore")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<!-- hello -->\n\n123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<!-- hello -->\n\n123");
}
#[test]
fn test_before_text_html_html_whitespace_sensitivitystrict_format_1_6e0a6ccf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .html_whitespace_sensitivity("strict")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<!-- hello -->\n\n123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<!-- hello -->\n\n123");
}
#[test]
fn test_before_text_html_print_width_infinity_format_1_6e0a6ccf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(INFINITY)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<!-- hello -->\n\n123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<!-- hello -->\n\n123");
}
#[test]
fn test_before_text_html_print_width_1_format_1_6e0a6ccf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(1)
        .parsers(vec!["html"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<!-- hello -->\n\n123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<!-- hello -->\n\n123");
}
#[test]
fn test_before_text_html_format_1_6e0a6ccf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["html"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<!-- hello -->\n\n123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<!-- hello -->\n\n123");
}
#[test]
fn test_bogus_html_html_whitespace_sensitivityignore_format_1_1a1a88c9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("ignore")
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<? hello ?>\n<!- world ->");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<? hello ?>\n<!- world ->");
}
#[test]
fn test_bogus_html_html_whitespace_sensitivitystrict_format_1_1a1a88c9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .html_whitespace_sensitivity("strict")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<? hello ?>\n<!- world ->");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<? hello ?>\n<!- world ->");
}
#[test]
fn test_bogus_html_print_width_infinity_format_1_1a1a88c9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(INFINITY)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<? hello ?>\n<!- world ->");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<? hello ?>\n<!- world ->");
}
#[test]
fn test_bogus_html_print_width_1_format_1_1a1a88c9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(1)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<? hello ?>\n<!- world ->");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<? hello ?>\n<!- world ->");
}
#[test]
fn test_bogus_html_format_1_1a1a88c9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<? hello ?>\n<!- world ->");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<? hello ?>\n<!- world ->");
}
#[test]
fn test_conditional_html_html_whitespace_sensitivityignore_format_1_52d360b9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["html"])
        .html_whitespace_sensitivity("ignore")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html>\n<html>\n  <body>\n\n    <!--[if IE 5]>This is IE 5<br><![endif]-->\n    <!--[if IE 6]>This is IE 6<br><![endif]-->\n    <!--[if IE 7]>This is IE 7<br><![endif]-->\n    <!--[if IE 8]>This is IE 8<br><![endif]-->\n    <!--[if IE 9]>This is IE 9<br><![endif]-->\n\n  </body>\n</html>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html lang=\"zh-CN\"><![endif]-->\n<html lang=\"zh-CN\">\n  <head></head>\n  <body></body>\n</html>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html lang=\"zh-CN\"><div><![endif]-->\n<html lang=\"zh-CN\">\n  <head></head>\n  <body></body>\n</html>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html lang=\"zh-CN\"><div></div><![endif]-->\n<html lang=\"zh-CN\">\n  <head></head>\n  <body></body>\n</html>\n\n<body width=\"100%\" align=\"center\">\n  <center  >                                        \n    <!--[if (gte mso 9)|(IE)]><table cellpadding=\"0\" cellspacing=\"0\" border=\"0\" width=\"600\" align=\"center\"><tr><td><![endif]-->\n    <div>  </div>\n    <!--[if (gte mso 9)|(IE)]></td></tr></table><![endif]-->\n  </center  >\n</body>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><![endif]-->\n<!--[if gte IE 9]><!--><html><!--<![endif]-->\n  <head></head>\n  <body></body>\n</html>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><![endif]-->\n<!--[if gte IE 9]><!--><html hello><!--<![endif]-->\n  <head></head>\n  <body></body>\n</html>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><head><![endif]-->\n<!--[if gte IE 9]><!--><html><head><!--<![endif]-->\n  </head>\n  <body></body>\n</html>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><![endif]-->\n<!--[if gte IE 9\n]><!--><html><!--<![endif]-->\n  <head></head>\n  <body></body>\n</html>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!doctype html>\n<html>\n  <body>\n    <!--[if IE 5]>\n      This is IE 5\n      <br />\n    <![endif]-->\n    <!--[if IE 6]>\n      This is IE 6\n      <br />\n    <![endif]-->\n    <!--[if IE 7]>\n      This is IE 7\n      <br />\n    <![endif]-->\n    <!--[if IE 8]>\n      This is IE 8\n      <br />\n    <![endif]-->\n    <!--[if IE 9]>\n      This is IE 9\n      <br />\n    <![endif]-->\n  </body>\n</html>\n\n<!doctype html>\n<!--[if lt IE 9]><html lang=\"zh-CN\"><![endif]-->\n<html lang=\"zh-CN\">\n  <head></head>\n  <body></body>\n</html>\n\n<!doctype html>\n<!--[if lt IE 9]><html lang=\"zh-CN\"><div><![endif]-->\n<html lang=\"zh-CN\">\n  <head></head>\n  <body></body>\n</html>\n\n<!doctype html>\n<!--[if lt IE 9]><html lang=\"zh-CN\"><div></div><![endif]-->\n<html lang=\"zh-CN\">\n  <head></head>\n  <body></body>\n</html>\n\n<body width=\"100%\" align=\"center\">\n  <center>\n    <!--[if (gte mso 9)|(IE)]><table cellpadding=\"0\" cellspacing=\"0\" border=\"0\" width=\"600\" align=\"center\"><tr><td><![endif]-->\n    <div></div>\n    <!--[if (gte mso 9)|(IE)]></td></tr></table><![endif]-->\n  </center>\n</body>\n\n<!doctype html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><![endif]-->\n<!--[if gte IE 9]><!--><html><!--<![endif]-->\n  <head></head>\n  <body></body>\n</html>\n\n<!doctype html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><![endif]-->\n<!--[if gte IE 9]><!--><html hello><!--<![endif]-->\n  <head></head>\n  <body></body>\n</html>\n\n<!doctype html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><head><![endif]-->\n<!--[if gte IE 9]><!-->\n<html>\n  <head>\n    <!--<![endif]-->\n  </head>\n  <body></body>\n</html>\n\n<!doctype html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><![endif]-->\n<!--[if gte IE 9]><!--><html><!--<![endif]-->\n  <head></head>\n  <body></body>\n</html>");
}
#[test]
fn test_conditional_html_html_whitespace_sensitivitystrict_format_1_52d360b9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("strict")
        .print_width(80)
        .parsers(vec!["html"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html>\n<html>\n  <body>\n\n    <!--[if IE 5]>This is IE 5<br><![endif]-->\n    <!--[if IE 6]>This is IE 6<br><![endif]-->\n    <!--[if IE 7]>This is IE 7<br><![endif]-->\n    <!--[if IE 8]>This is IE 8<br><![endif]-->\n    <!--[if IE 9]>This is IE 9<br><![endif]-->\n\n  </body>\n</html>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html lang=\"zh-CN\"><![endif]-->\n<html lang=\"zh-CN\">\n  <head></head>\n  <body></body>\n</html>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html lang=\"zh-CN\"><div><![endif]-->\n<html lang=\"zh-CN\">\n  <head></head>\n  <body></body>\n</html>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html lang=\"zh-CN\"><div></div><![endif]-->\n<html lang=\"zh-CN\">\n  <head></head>\n  <body></body>\n</html>\n\n<body width=\"100%\" align=\"center\">\n  <center  >                                        \n    <!--[if (gte mso 9)|(IE)]><table cellpadding=\"0\" cellspacing=\"0\" border=\"0\" width=\"600\" align=\"center\"><tr><td><![endif]-->\n    <div>  </div>\n    <!--[if (gte mso 9)|(IE)]></td></tr></table><![endif]-->\n  </center  >\n</body>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><![endif]-->\n<!--[if gte IE 9]><!--><html><!--<![endif]-->\n  <head></head>\n  <body></body>\n</html>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><![endif]-->\n<!--[if gte IE 9]><!--><html hello><!--<![endif]-->\n  <head></head>\n  <body></body>\n</html>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><head><![endif]-->\n<!--[if gte IE 9]><!--><html><head><!--<![endif]-->\n  </head>\n  <body></body>\n</html>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><![endif]-->\n<!--[if gte IE 9\n]><!--><html><!--<![endif]-->\n  <head></head>\n  <body></body>\n</html>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!doctype html>\n<html>\n  <body>\n    <!--[if IE 5]>This is IE 5<br /><![endif]-->\n    <!--[if IE 6]>This is IE 6<br /><![endif]-->\n    <!--[if IE 7]>This is IE 7<br /><![endif]-->\n    <!--[if IE 8]>This is IE 8<br /><![endif]-->\n    <!--[if IE 9]>This is IE 9<br /><![endif]-->\n  </body>\n</html>\n\n<!doctype html>\n<!--[if lt IE 9]><html lang=\"zh-CN\"><![endif]-->\n<html lang=\"zh-CN\">\n  <head></head>\n  <body></body>\n</html>\n\n<!doctype html>\n<!--[if lt IE 9]><html lang=\"zh-CN\"><div><![endif]-->\n<html lang=\"zh-CN\">\n  <head></head>\n  <body></body>\n</html>\n\n<!doctype html>\n<!--[if lt IE 9]><html lang=\"zh-CN\"><div></div><![endif]-->\n<html lang=\"zh-CN\">\n  <head></head>\n  <body></body>\n</html>\n\n<body width=\"100%\" align=\"center\">\n  <center>\n    <!--[if (gte mso 9)|(IE)]><table cellpadding=\"0\" cellspacing=\"0\" border=\"0\" width=\"600\" align=\"center\"><tr><td><![endif]-->\n    <div> </div>\n    <!--[if (gte mso 9)|(IE)]></td></tr></table><![endif]-->\n  </center>\n</body>\n\n<!doctype html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><![endif]-->\n<!--[if gte IE 9]><!--><html><!--<![endif]-->\n  <head></head>\n  <body></body>\n</html>\n\n<!doctype html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><![endif]-->\n<!--[if gte IE 9]><!--><html hello><!--<![endif]-->\n  <head></head>\n  <body></body>\n</html>\n\n<!doctype html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><head><![endif]-->\n<!--[if gte IE 9\n]><!--><html\n  ><head\n    ><!--<![endif]-->\n  </head>\n  <body></body>\n</html>\n\n<!doctype html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><![endif]-->\n<!--[if gte IE 9]><!--><html><!--<![endif]-->\n  <head></head>\n  <body></body>\n</html>");
}
#[test]
fn test_conditional_html_print_width_infinity_format_1_52d360b9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(INFINITY)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html>\n<html>\n  <body>\n\n    <!--[if IE 5]>This is IE 5<br><![endif]-->\n    <!--[if IE 6]>This is IE 6<br><![endif]-->\n    <!--[if IE 7]>This is IE 7<br><![endif]-->\n    <!--[if IE 8]>This is IE 8<br><![endif]-->\n    <!--[if IE 9]>This is IE 9<br><![endif]-->\n\n  </body>\n</html>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html lang=\"zh-CN\"><![endif]-->\n<html lang=\"zh-CN\">\n  <head></head>\n  <body></body>\n</html>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html lang=\"zh-CN\"><div><![endif]-->\n<html lang=\"zh-CN\">\n  <head></head>\n  <body></body>\n</html>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html lang=\"zh-CN\"><div></div><![endif]-->\n<html lang=\"zh-CN\">\n  <head></head>\n  <body></body>\n</html>\n\n<body width=\"100%\" align=\"center\">\n  <center  >                                        \n    <!--[if (gte mso 9)|(IE)]><table cellpadding=\"0\" cellspacing=\"0\" border=\"0\" width=\"600\" align=\"center\"><tr><td><![endif]-->\n    <div>  </div>\n    <!--[if (gte mso 9)|(IE)]></td></tr></table><![endif]-->\n  </center  >\n</body>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><![endif]-->\n<!--[if gte IE 9]><!--><html><!--<![endif]-->\n  <head></head>\n  <body></body>\n</html>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><![endif]-->\n<!--[if gte IE 9]><!--><html hello><!--<![endif]-->\n  <head></head>\n  <body></body>\n</html>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><head><![endif]-->\n<!--[if gte IE 9]><!--><html><head><!--<![endif]-->\n  </head>\n  <body></body>\n</html>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><![endif]-->\n<!--[if gte IE 9\n]><!--><html><!--<![endif]-->\n  <head></head>\n  <body></body>\n</html>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!doctype html>\n<html>\n  <body>\n    <!--[if IE 5]>This is IE 5<br /><![endif]-->\n    <!--[if IE 6]>This is IE 6<br /><![endif]-->\n    <!--[if IE 7]>This is IE 7<br /><![endif]-->\n    <!--[if IE 8]>This is IE 8<br /><![endif]-->\n    <!--[if IE 9]>This is IE 9<br /><![endif]-->\n  </body>\n</html>\n\n<!doctype html>\n<!--[if lt IE 9]><html lang=\"zh-CN\"><![endif]-->\n<html lang=\"zh-CN\">\n  <head></head>\n  <body></body>\n</html>\n\n<!doctype html>\n<!--[if lt IE 9]><html lang=\"zh-CN\"><div><![endif]-->\n<html lang=\"zh-CN\">\n  <head></head>\n  <body></body>\n</html>\n\n<!doctype html>\n<!--[if lt IE 9]><html lang=\"zh-CN\"><div></div><![endif]-->\n<html lang=\"zh-CN\">\n  <head></head>\n  <body></body>\n</html>\n\n<body width=\"100%\" align=\"center\">\n  <center>\n    <!--[if (gte mso 9)|(IE)]><table cellpadding=\"0\" cellspacing=\"0\" border=\"0\" width=\"600\" align=\"center\"><tr><td><![endif]-->\n    <div></div>\n    <!--[if (gte mso 9)|(IE)]></td></tr></table><![endif]-->\n  </center>\n</body>\n\n<!doctype html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><![endif]-->\n<!--[if gte IE 9]><!--><html><!--<![endif]-->\n  <head></head>\n  <body></body>\n</html>\n\n<!doctype html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><![endif]-->\n<!--[if gte IE 9]><!--><html hello><!--<![endif]-->\n  <head></head>\n  <body></body>\n</html>\n\n<!doctype html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><head><![endif]-->\n<!--[if gte IE 9]><!-->\n<html>\n  <head>\n    <!--<![endif]-->\n  </head>\n  <body></body>\n</html>\n\n<!doctype html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><![endif]-->\n<!--[if gte IE 9]><!--><html><!--<![endif]-->\n  <head></head>\n  <body></body>\n</html>");
}
#[test]
fn test_conditional_html_print_width_1_format_1_52d360b9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(1)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html>\n<html>\n  <body>\n\n    <!--[if IE 5]>This is IE 5<br><![endif]-->\n    <!--[if IE 6]>This is IE 6<br><![endif]-->\n    <!--[if IE 7]>This is IE 7<br><![endif]-->\n    <!--[if IE 8]>This is IE 8<br><![endif]-->\n    <!--[if IE 9]>This is IE 9<br><![endif]-->\n\n  </body>\n</html>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html lang=\"zh-CN\"><![endif]-->\n<html lang=\"zh-CN\">\n  <head></head>\n  <body></body>\n</html>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html lang=\"zh-CN\"><div><![endif]-->\n<html lang=\"zh-CN\">\n  <head></head>\n  <body></body>\n</html>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html lang=\"zh-CN\"><div></div><![endif]-->\n<html lang=\"zh-CN\">\n  <head></head>\n  <body></body>\n</html>\n\n<body width=\"100%\" align=\"center\">\n  <center  >                                        \n    <!--[if (gte mso 9)|(IE)]><table cellpadding=\"0\" cellspacing=\"0\" border=\"0\" width=\"600\" align=\"center\"><tr><td><![endif]-->\n    <div>  </div>\n    <!--[if (gte mso 9)|(IE)]></td></tr></table><![endif]-->\n  </center  >\n</body>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><![endif]-->\n<!--[if gte IE 9]><!--><html><!--<![endif]-->\n  <head></head>\n  <body></body>\n</html>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><![endif]-->\n<!--[if gte IE 9]><!--><html hello><!--<![endif]-->\n  <head></head>\n  <body></body>\n</html>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><head><![endif]-->\n<!--[if gte IE 9]><!--><html><head><!--<![endif]-->\n  </head>\n  <body></body>\n</html>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><![endif]-->\n<!--[if gte IE 9\n]><!--><html><!--<![endif]-->\n  <head></head>\n  <body></body>\n</html>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!doctype html>\n<html>\n  <body>\n    <!--[if IE 5\n      ]>This\n      is\n      IE\n      5<br\n    /><![endif]-->\n    <!--[if IE 6\n      ]>This\n      is\n      IE\n      6<br\n    /><![endif]-->\n    <!--[if IE 7\n      ]>This\n      is\n      IE\n      7<br\n    /><![endif]-->\n    <!--[if IE 8\n      ]>This\n      is\n      IE\n      8<br\n    /><![endif]-->\n    <!--[if IE 9\n      ]>This\n      is\n      IE\n      9<br\n    /><![endif]-->\n  </body>\n</html>\n\n<!doctype html>\n<!--[if lt IE 9]><html lang=\"zh-CN\"><![endif]-->\n<html\n  lang=\"zh-CN\"\n>\n  <head></head>\n  <body></body>\n</html>\n\n<!doctype html>\n<!--[if lt IE 9]><html lang=\"zh-CN\"><div><![endif]-->\n<html\n  lang=\"zh-CN\"\n>\n  <head></head>\n  <body></body>\n</html>\n\n<!doctype html>\n<!--[if lt IE 9]><html lang=\"zh-CN\"><div></div><![endif]-->\n<html\n  lang=\"zh-CN\"\n>\n  <head></head>\n  <body></body>\n</html>\n\n<body\n  width=\"100%\"\n  align=\"center\"\n>\n  <center>\n    <!--[if (gte mso 9)|(IE)]><table cellpadding=\"0\" cellspacing=\"0\" border=\"0\" width=\"600\" align=\"center\"><tr><td><![endif]-->\n    <div></div>\n    <!--[if (gte mso 9)|(IE)]></td></tr></table><![endif]-->\n  </center>\n</body>\n\n<!doctype html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><![endif]-->\n<!--[if gte IE 9]><!--><html><!--<![endif]-->\n  <head></head>\n  <body></body>\n</html>\n\n<!doctype html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><![endif]-->\n<!--[if gte IE 9]><!--><html\n  hello\n><!--<![endif]-->\n  <head></head>\n  <body></body>\n</html>\n\n<!doctype html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><head><![endif]-->\n<!--[if gte IE 9]><!-->\n<html>\n  <head>\n    <!--<![endif]-->\n  </head>\n  <body></body>\n</html>\n\n<!doctype html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><![endif]-->\n<!--[if gte IE 9]><!--><html><!--<![endif]-->\n  <head></head>\n  <body></body>\n</html>");
}
#[test]
fn test_conditional_html_format_1_52d360b9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["html"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html>\n<html>\n  <body>\n\n    <!--[if IE 5]>This is IE 5<br><![endif]-->\n    <!--[if IE 6]>This is IE 6<br><![endif]-->\n    <!--[if IE 7]>This is IE 7<br><![endif]-->\n    <!--[if IE 8]>This is IE 8<br><![endif]-->\n    <!--[if IE 9]>This is IE 9<br><![endif]-->\n\n  </body>\n</html>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html lang=\"zh-CN\"><![endif]-->\n<html lang=\"zh-CN\">\n  <head></head>\n  <body></body>\n</html>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html lang=\"zh-CN\"><div><![endif]-->\n<html lang=\"zh-CN\">\n  <head></head>\n  <body></body>\n</html>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html lang=\"zh-CN\"><div></div><![endif]-->\n<html lang=\"zh-CN\">\n  <head></head>\n  <body></body>\n</html>\n\n<body width=\"100%\" align=\"center\">\n  <center  >                                        \n    <!--[if (gte mso 9)|(IE)]><table cellpadding=\"0\" cellspacing=\"0\" border=\"0\" width=\"600\" align=\"center\"><tr><td><![endif]-->\n    <div>  </div>\n    <!--[if (gte mso 9)|(IE)]></td></tr></table><![endif]-->\n  </center  >\n</body>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><![endif]-->\n<!--[if gte IE 9]><!--><html><!--<![endif]-->\n  <head></head>\n  <body></body>\n</html>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><![endif]-->\n<!--[if gte IE 9]><!--><html hello><!--<![endif]-->\n  <head></head>\n  <body></body>\n</html>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><head><![endif]-->\n<!--[if gte IE 9]><!--><html><head><!--<![endif]-->\n  </head>\n  <body></body>\n</html>\n\n<!DOCTYPE html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><![endif]-->\n<!--[if gte IE 9\n]><!--><html><!--<![endif]-->\n  <head></head>\n  <body></body>\n</html>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!doctype html>\n<html>\n  <body>\n    <!--[if IE 5]>This is IE 5<br /><![endif]-->\n    <!--[if IE 6]>This is IE 6<br /><![endif]-->\n    <!--[if IE 7]>This is IE 7<br /><![endif]-->\n    <!--[if IE 8]>This is IE 8<br /><![endif]-->\n    <!--[if IE 9]>This is IE 9<br /><![endif]-->\n  </body>\n</html>\n\n<!doctype html>\n<!--[if lt IE 9]><html lang=\"zh-CN\"><![endif]-->\n<html lang=\"zh-CN\">\n  <head></head>\n  <body></body>\n</html>\n\n<!doctype html>\n<!--[if lt IE 9]><html lang=\"zh-CN\"><div><![endif]-->\n<html lang=\"zh-CN\">\n  <head></head>\n  <body></body>\n</html>\n\n<!doctype html>\n<!--[if lt IE 9]><html lang=\"zh-CN\"><div></div><![endif]-->\n<html lang=\"zh-CN\">\n  <head></head>\n  <body></body>\n</html>\n\n<body width=\"100%\" align=\"center\">\n  <center>\n    <!--[if (gte mso 9)|(IE)]><table cellpadding=\"0\" cellspacing=\"0\" border=\"0\" width=\"600\" align=\"center\"><tr><td><![endif]-->\n    <div></div>\n    <!--[if (gte mso 9)|(IE)]></td></tr></table><![endif]-->\n  </center>\n</body>\n\n<!doctype html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><![endif]-->\n<!--[if gte IE 9]><!--><html><!--<![endif]-->\n  <head></head>\n  <body></body>\n</html>\n\n<!doctype html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><![endif]-->\n<!--[if gte IE 9]><!--><html hello><!--<![endif]-->\n  <head></head>\n  <body></body>\n</html>\n\n<!doctype html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><head><![endif]-->\n<!--[if gte IE 9]><!-->\n<html>\n  <head>\n    <!--<![endif]-->\n  </head>\n  <body></body>\n</html>\n\n<!doctype html>\n<!--[if lt IE 9]><html class=\"legacy-ie\"><![endif]-->\n<!--[if gte IE 9]><!--><html><!--<![endif]-->\n  <head></head>\n  <body></body>\n</html>");
}
#[test]
fn test_for_debugging_html_html_whitespace_sensitivityignore_format_1_41d72a6f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .html_whitespace_sensitivity("ignore")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html>\n<html>\n  <body>\n\n<!-- Do not display this at the moment\n<img border=\"0\" src=\"pic_trulli.jpg\" alt=\"Trulli\">\n-->\n\n  <!-- Do not display this at the moment\n  <img border=\"0\" src=\"pic_trulli.jpg\" alt=\"Trulli\">\n  -->\n\n    <!-- Do not display this at the moment\n    <img border=\"0\" src=\"pic_trulli.jpg\" alt=\"Trulli\">\n    -->\n\n  </body>\n</html>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!doctype html>\n<html>\n  <body>\n    <!-- Do not display this at the moment\n<img border=\"0\" src=\"pic_trulli.jpg\" alt=\"Trulli\">\n-->\n\n    <!-- Do not display this at the moment\n  <img border=\"0\" src=\"pic_trulli.jpg\" alt=\"Trulli\">\n  -->\n\n    <!-- Do not display this at the moment\n    <img border=\"0\" src=\"pic_trulli.jpg\" alt=\"Trulli\">\n    -->\n  </body>\n</html>");
}
#[test]
fn test_for_debugging_html_html_whitespace_sensitivitystrict_format_1_41d72a6f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .html_whitespace_sensitivity("strict")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html>\n<html>\n  <body>\n\n<!-- Do not display this at the moment\n<img border=\"0\" src=\"pic_trulli.jpg\" alt=\"Trulli\">\n-->\n\n  <!-- Do not display this at the moment\n  <img border=\"0\" src=\"pic_trulli.jpg\" alt=\"Trulli\">\n  -->\n\n    <!-- Do not display this at the moment\n    <img border=\"0\" src=\"pic_trulli.jpg\" alt=\"Trulli\">\n    -->\n\n  </body>\n</html>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!doctype html>\n<html>\n  <body>\n    <!-- Do not display this at the moment\n<img border=\"0\" src=\"pic_trulli.jpg\" alt=\"Trulli\">\n-->\n\n    <!-- Do not display this at the moment\n  <img border=\"0\" src=\"pic_trulli.jpg\" alt=\"Trulli\">\n  -->\n\n    <!-- Do not display this at the moment\n    <img border=\"0\" src=\"pic_trulli.jpg\" alt=\"Trulli\">\n    -->\n  </body>\n</html>");
}
#[test]
fn test_for_debugging_html_print_width_infinity_format_1_41d72a6f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(INFINITY)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html>\n<html>\n  <body>\n\n<!-- Do not display this at the moment\n<img border=\"0\" src=\"pic_trulli.jpg\" alt=\"Trulli\">\n-->\n\n  <!-- Do not display this at the moment\n  <img border=\"0\" src=\"pic_trulli.jpg\" alt=\"Trulli\">\n  -->\n\n    <!-- Do not display this at the moment\n    <img border=\"0\" src=\"pic_trulli.jpg\" alt=\"Trulli\">\n    -->\n\n  </body>\n</html>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!doctype html>\n<html>\n  <body>\n    <!-- Do not display this at the moment\n<img border=\"0\" src=\"pic_trulli.jpg\" alt=\"Trulli\">\n-->\n\n    <!-- Do not display this at the moment\n  <img border=\"0\" src=\"pic_trulli.jpg\" alt=\"Trulli\">\n  -->\n\n    <!-- Do not display this at the moment\n    <img border=\"0\" src=\"pic_trulli.jpg\" alt=\"Trulli\">\n    -->\n  </body>\n</html>");
}
#[test]
fn test_for_debugging_html_print_width_1_format_1_41d72a6f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(1)
        .parsers(vec!["html"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html>\n<html>\n  <body>\n\n<!-- Do not display this at the moment\n<img border=\"0\" src=\"pic_trulli.jpg\" alt=\"Trulli\">\n-->\n\n  <!-- Do not display this at the moment\n  <img border=\"0\" src=\"pic_trulli.jpg\" alt=\"Trulli\">\n  -->\n\n    <!-- Do not display this at the moment\n    <img border=\"0\" src=\"pic_trulli.jpg\" alt=\"Trulli\">\n    -->\n\n  </body>\n</html>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!doctype html>\n<html>\n  <body>\n    <!-- Do not display this at the moment\n<img border=\"0\" src=\"pic_trulli.jpg\" alt=\"Trulli\">\n-->\n\n    <!-- Do not display this at the moment\n  <img border=\"0\" src=\"pic_trulli.jpg\" alt=\"Trulli\">\n  -->\n\n    <!-- Do not display this at the moment\n    <img border=\"0\" src=\"pic_trulli.jpg\" alt=\"Trulli\">\n    -->\n  </body>\n</html>");
}
#[test]
fn test_for_debugging_html_format_1_41d72a6f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html>\n<html>\n  <body>\n\n<!-- Do not display this at the moment\n<img border=\"0\" src=\"pic_trulli.jpg\" alt=\"Trulli\">\n-->\n\n  <!-- Do not display this at the moment\n  <img border=\"0\" src=\"pic_trulli.jpg\" alt=\"Trulli\">\n  -->\n\n    <!-- Do not display this at the moment\n    <img border=\"0\" src=\"pic_trulli.jpg\" alt=\"Trulli\">\n    -->\n\n  </body>\n</html>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!doctype html>\n<html>\n  <body>\n    <!-- Do not display this at the moment\n<img border=\"0\" src=\"pic_trulli.jpg\" alt=\"Trulli\">\n-->\n\n    <!-- Do not display this at the moment\n  <img border=\"0\" src=\"pic_trulli.jpg\" alt=\"Trulli\">\n  -->\n\n    <!-- Do not display this at the moment\n    <img border=\"0\" src=\"pic_trulli.jpg\" alt=\"Trulli\">\n    -->\n  </body>\n</html>");
}
#[test]
fn test_hidden_html_html_whitespace_sensitivityignore_format_1_a0cf8294() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("ignore")
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html>\n<html>\n    <body>\n\n        <!--This is a comment-->\n        <!-- This is a comment -->\n        <!--  This is a comment  -->\n        <!--   This   is   a   comment   -->\n        <p>This is a paragraph.</p>\n        <!-- Comments are not displayed in the browser -->\n\n    </body>\n</html>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!doctype html>\n<html>\n  <body>\n    <!--This is a comment-->\n    <!-- This is a comment -->\n    <!--  This is a comment  -->\n    <!--   This   is   a   comment   -->\n    <p>This is a paragraph.</p>\n    <!-- Comments are not displayed in the browser -->\n  </body>\n</html>");
}
#[test]
fn test_hidden_html_html_whitespace_sensitivitystrict_format_1_a0cf8294() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("strict")
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html>\n<html>\n    <body>\n\n        <!--This is a comment-->\n        <!-- This is a comment -->\n        <!--  This is a comment  -->\n        <!--   This   is   a   comment   -->\n        <p>This is a paragraph.</p>\n        <!-- Comments are not displayed in the browser -->\n\n    </body>\n</html>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!doctype html>\n<html>\n  <body>\n    <!--This is a comment-->\n    <!-- This is a comment -->\n    <!--  This is a comment  -->\n    <!--   This   is   a   comment   -->\n    <p>This is a paragraph.</p>\n    <!-- Comments are not displayed in the browser -->\n  </body>\n</html>");
}
#[test]
fn test_hidden_html_print_width_infinity_format_1_a0cf8294() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(INFINITY)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html>\n<html>\n    <body>\n\n        <!--This is a comment-->\n        <!-- This is a comment -->\n        <!--  This is a comment  -->\n        <!--   This   is   a   comment   -->\n        <p>This is a paragraph.</p>\n        <!-- Comments are not displayed in the browser -->\n\n    </body>\n</html>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!doctype html>\n<html>\n  <body>\n    <!--This is a comment-->\n    <!-- This is a comment -->\n    <!--  This is a comment  -->\n    <!--   This   is   a   comment   -->\n    <p>This is a paragraph.</p>\n    <!-- Comments are not displayed in the browser -->\n  </body>\n</html>");
}
#[test]
fn test_hidden_html_print_width_1_format_1_a0cf8294() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(1)
        .parsers(vec!["html"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html>\n<html>\n    <body>\n\n        <!--This is a comment-->\n        <!-- This is a comment -->\n        <!--  This is a comment  -->\n        <!--   This   is   a   comment   -->\n        <p>This is a paragraph.</p>\n        <!-- Comments are not displayed in the browser -->\n\n    </body>\n</html>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!doctype html>\n<html>\n  <body>\n    <!--This is a comment-->\n    <!-- This is a comment -->\n    <!--  This is a comment  -->\n    <!--   This   is   a   comment   -->\n    <p>\n      This\n      is\n      a\n      paragraph.\n    </p>\n    <!-- Comments are not displayed in the browser -->\n  </body>\n</html>");
}
#[test]
fn test_hidden_html_format_1_a0cf8294() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["html"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html>\n<html>\n    <body>\n\n        <!--This is a comment-->\n        <!-- This is a comment -->\n        <!--  This is a comment  -->\n        <!--   This   is   a   comment   -->\n        <p>This is a paragraph.</p>\n        <!-- Comments are not displayed in the browser -->\n\n    </body>\n</html>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!doctype html>\n<html>\n  <body>\n    <!--This is a comment-->\n    <!-- This is a comment -->\n    <!--  This is a comment  -->\n    <!--   This   is   a   comment   -->\n    <p>This is a paragraph.</p>\n    <!-- Comments are not displayed in the browser -->\n  </body>\n</html>");
}
#[test]
fn test_surrounding_empty_line_html_html_whitespace_sensitivityignore_format_1_c2fbc0bb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .html_whitespace_sensitivity("ignore")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<ul><!-- 123\n--><li>First</li><!-- 123\n456\n   789\n--><li>Second</li><!--\n\n\n    123\n       456\n          789\n\n\n--><li>Second</li><!--\n\n\n           123\n        456\n    789\n\n\n--></ul>\n<span><!--\n--><span>a</span><!--\n--><span>b</span><!--\n--></span>\n\n<span><!-- 1\n--><span>a</span><!-- 2\n--><span>b</span><!-- 3\n--></span>\n\n<span><!--\n1 --><span>a</span><!--\n2 --><span>b</span><!--\n3 --></span>\n\n123<!---->456\n\n123<!--x-->456\n\n<!-- A\n     B -->\n\n<!--\nThe null hero's name is {{nullHero.name}}\n\nSee console log:\n  TypeError: Cannot read property 'name' of null in [null]\n-->\n\n<!--\n    The null hero's name is {{nullHero.name}}\n\n    See console log:\n    TypeError: Cannot read property 'name' of null in [null]\n-->") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<ul>\n  <!-- 123\n-->\n  <li>First</li>\n  <!-- 123\n456\n   789\n-->\n  <li>Second</li>\n  <!--\n\n\n    123\n       456\n          789\n\n\n-->\n  <li>Second</li>\n  <!--\n\n\n           123\n        456\n    789\n\n\n--></ul>\n<span>\n  <!--\n-->\n  <span>a</span>\n  <!--\n-->\n  <span>b</span>\n  <!--\n--></span>\n\n<span>\n  <!-- 1\n-->\n  <span>a</span>\n  <!-- 2\n-->\n  <span>b</span>\n  <!-- 3\n--></span>\n\n<span>\n  <!--\n1 -->\n  <span>a</span>\n  <!--\n2 -->\n  <span>b</span>\n  <!--\n3 -->\n</span>\n\n123\n<!---->\n456 123\n<!--x-->\n456\n\n<!-- A\n     B -->\n\n<!--\nThe null hero's name is {{nullHero.name}}\n\nSee console log:\n  TypeError: Cannot read property 'name' of null in [null]\n-->\n\n<!--\n    The null hero's name is {{nullHero.name}}\n\n    See console log:\n    TypeError: Cannot read property 'name' of null in [null]\n-->");
}
#[test]
fn test_surrounding_empty_line_html_html_whitespace_sensitivitystrict_format_1_c2fbc0bb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("strict")
        .print_width(80)
        .parsers(vec!["html"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<ul><!-- 123\n--><li>First</li><!-- 123\n456\n   789\n--><li>Second</li><!--\n\n\n    123\n       456\n          789\n\n\n--><li>Second</li><!--\n\n\n           123\n        456\n    789\n\n\n--></ul>\n<span><!--\n--><span>a</span><!--\n--><span>b</span><!--\n--></span>\n\n<span><!-- 1\n--><span>a</span><!-- 2\n--><span>b</span><!-- 3\n--></span>\n\n<span><!--\n1 --><span>a</span><!--\n2 --><span>b</span><!--\n3 --></span>\n\n123<!---->456\n\n123<!--x-->456\n\n<!-- A\n     B -->\n\n<!--\nThe null hero's name is {{nullHero.name}}\n\nSee console log:\n  TypeError: Cannot read property 'name' of null in [null]\n-->\n\n<!--\n    The null hero's name is {{nullHero.name}}\n\n    See console log:\n    TypeError: Cannot read property 'name' of null in [null]\n-->") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<ul\n  ><!-- 123\n--><li>First</li\n  ><!-- 123\n456\n   789\n--><li>Second</li\n  ><!--\n\n\n    123\n       456\n          789\n\n\n--><li>Second</li\n  ><!--\n\n\n           123\n        456\n    789\n\n\n--></ul>\n<span\n  ><!--\n--><span>a</span\n  ><!--\n--><span>b</span\n  ><!--\n--></span>\n\n<span\n  ><!-- 1\n--><span>a</span\n  ><!-- 2\n--><span>b</span\n  ><!-- 3\n--></span>\n\n<span\n  ><!--\n1 --><span>a</span\n  ><!--\n2 --><span>b</span\n  ><!--\n3 --></span\n>\n\n123<!---->456 123<!--x-->456\n\n<!-- A\n     B -->\n\n<!--\nThe null hero's name is {{nullHero.name}}\n\nSee console log:\n  TypeError: Cannot read property 'name' of null in [null]\n-->\n\n<!--\n    The null hero's name is {{nullHero.name}}\n\n    See console log:\n    TypeError: Cannot read property 'name' of null in [null]\n-->");
}
#[test]
fn test_surrounding_empty_line_html_print_width_infinity_format_1_c2fbc0bb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(INFINITY)
        .parsers(vec!["html"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<ul><!-- 123\n--><li>First</li><!-- 123\n456\n   789\n--><li>Second</li><!--\n\n\n    123\n       456\n          789\n\n\n--><li>Second</li><!--\n\n\n           123\n        456\n    789\n\n\n--></ul>\n<span><!--\n--><span>a</span><!--\n--><span>b</span><!--\n--></span>\n\n<span><!-- 1\n--><span>a</span><!-- 2\n--><span>b</span><!-- 3\n--></span>\n\n<span><!--\n1 --><span>a</span><!--\n2 --><span>b</span><!--\n3 --></span>\n\n123<!---->456\n\n123<!--x-->456\n\n<!-- A\n     B -->\n\n<!--\nThe null hero's name is {{nullHero.name}}\n\nSee console log:\n  TypeError: Cannot read property 'name' of null in [null]\n-->\n\n<!--\n    The null hero's name is {{nullHero.name}}\n\n    See console log:\n    TypeError: Cannot read property 'name' of null in [null]\n-->") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<ul>\n  <!-- 123\n-->\n  <li>First</li>\n  <!-- 123\n456\n   789\n-->\n  <li>Second</li>\n  <!--\n\n\n    123\n       456\n          789\n\n\n-->\n  <li>Second</li>\n  <!--\n\n\n           123\n        456\n    789\n\n\n--></ul>\n<span\n  ><!--\n--><span>a</span\n  ><!--\n--><span>b</span\n  ><!--\n--></span>\n\n<span\n  ><!-- 1\n--><span>a</span\n  ><!-- 2\n--><span>b</span\n  ><!-- 3\n--></span>\n\n<span\n  ><!--\n1 --><span>a</span\n  ><!--\n2 --><span>b</span\n  ><!--\n3 --></span\n>\n\n123<!---->456 123<!--x-->456\n\n<!-- A\n     B -->\n\n<!--\nThe null hero's name is {{nullHero.name}}\n\nSee console log:\n  TypeError: Cannot read property 'name' of null in [null]\n-->\n\n<!--\n    The null hero's name is {{nullHero.name}}\n\n    See console log:\n    TypeError: Cannot read property 'name' of null in [null]\n-->");
}
#[test]
fn test_surrounding_empty_line_html_print_width_1_format_1_c2fbc0bb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(1)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<ul><!-- 123\n--><li>First</li><!-- 123\n456\n   789\n--><li>Second</li><!--\n\n\n    123\n       456\n          789\n\n\n--><li>Second</li><!--\n\n\n           123\n        456\n    789\n\n\n--></ul>\n<span><!--\n--><span>a</span><!--\n--><span>b</span><!--\n--></span>\n\n<span><!-- 1\n--><span>a</span><!-- 2\n--><span>b</span><!-- 3\n--></span>\n\n<span><!--\n1 --><span>a</span><!--\n2 --><span>b</span><!--\n3 --></span>\n\n123<!---->456\n\n123<!--x-->456\n\n<!-- A\n     B -->\n\n<!--\nThe null hero's name is {{nullHero.name}}\n\nSee console log:\n  TypeError: Cannot read property 'name' of null in [null]\n-->\n\n<!--\n    The null hero's name is {{nullHero.name}}\n\n    See console log:\n    TypeError: Cannot read property 'name' of null in [null]\n-->") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<ul>\n  <!-- 123\n-->\n  <li>\n    First\n  </li>\n  <!-- 123\n456\n   789\n-->\n  <li>\n    Second\n  </li>\n  <!--\n\n\n    123\n       456\n          789\n\n\n-->\n  <li>\n    Second\n  </li>\n  <!--\n\n\n           123\n        456\n    789\n\n\n--></ul>\n<span\n  ><!--\n--><span\n    >a</span\n  ><!--\n--><span\n    >b</span\n  ><!--\n--></span>\n\n<span\n  ><!-- 1\n--><span\n    >a</span\n  ><!-- 2\n--><span\n    >b</span\n  ><!-- 3\n--></span>\n\n<span\n  ><!--\n1 --><span\n    >a</span\n  ><!--\n2 --><span\n    >b</span\n  ><!--\n3 --></span\n>\n\n123<!---->456\n123<!--x-->456\n\n<!-- A\n     B -->\n\n<!--\nThe null hero's name is {{nullHero.name}}\n\nSee console log:\n  TypeError: Cannot read property 'name' of null in [null]\n-->\n\n<!--\n    The null hero's name is {{nullHero.name}}\n\n    See console log:\n    TypeError: Cannot read property 'name' of null in [null]\n-->");
}
#[test]
fn test_surrounding_empty_line_html_format_1_c2fbc0bb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["html"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<ul><!-- 123\n--><li>First</li><!-- 123\n456\n   789\n--><li>Second</li><!--\n\n\n    123\n       456\n          789\n\n\n--><li>Second</li><!--\n\n\n           123\n        456\n    789\n\n\n--></ul>\n<span><!--\n--><span>a</span><!--\n--><span>b</span><!--\n--></span>\n\n<span><!-- 1\n--><span>a</span><!-- 2\n--><span>b</span><!-- 3\n--></span>\n\n<span><!--\n1 --><span>a</span><!--\n2 --><span>b</span><!--\n3 --></span>\n\n123<!---->456\n\n123<!--x-->456\n\n<!-- A\n     B -->\n\n<!--\nThe null hero's name is {{nullHero.name}}\n\nSee console log:\n  TypeError: Cannot read property 'name' of null in [null]\n-->\n\n<!--\n    The null hero's name is {{nullHero.name}}\n\n    See console log:\n    TypeError: Cannot read property 'name' of null in [null]\n-->") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<ul>\n  <!-- 123\n-->\n  <li>First</li>\n  <!-- 123\n456\n   789\n-->\n  <li>Second</li>\n  <!--\n\n\n    123\n       456\n          789\n\n\n-->\n  <li>Second</li>\n  <!--\n\n\n           123\n        456\n    789\n\n\n--></ul>\n<span\n  ><!--\n--><span>a</span\n  ><!--\n--><span>b</span\n  ><!--\n--></span>\n\n<span\n  ><!-- 1\n--><span>a</span\n  ><!-- 2\n--><span>b</span\n  ><!-- 3\n--></span>\n\n<span\n  ><!--\n1 --><span>a</span\n  ><!--\n2 --><span>b</span\n  ><!--\n3 --></span\n>\n\n123<!---->456 123<!--x-->456\n\n<!-- A\n     B -->\n\n<!--\nThe null hero's name is {{nullHero.name}}\n\nSee console log:\n  TypeError: Cannot read property 'name' of null in [null]\n-->\n\n<!--\n    The null hero's name is {{nullHero.name}}\n\n    See console log:\n    TypeError: Cannot read property 'name' of null in [null]\n-->");
}
