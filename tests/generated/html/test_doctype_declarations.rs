#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_html_4_01_frameset_html_format_1_325c4c13() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 4.01 Frameset//EN\"\n  \"http://www.w3.org/TR/html4/frameset.dtd\">\n<html>\n  <head>\n    <title>An HTML standard template</title>\n    <meta charset=\"utf-8\"  />\n  </head>\n  <body>\n    <p>… Your HTML content here …</p>\n  </body>\n</html>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!DOCTYPE html PUBLIC \"-//W3C//DTD HTML 4.01 Frameset//EN\" \"http://www.w3.org/TR/html4/frameset.dtd\">\n<html>\n  <head>\n    <title>An HTML standard template</title>\n    <meta charset=\"utf-8\" />\n  </head>\n  <body>\n    <p>… Your HTML content here …</p>\n  </body>\n</html>");
}
#[test]
fn test_html_4_01_strict_html_format_1_ca090183() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 4.01//EN\"\n  \"http://www.w3.org/TR/html4/strict.dtd\">\n<html>\n  <head>\n    <title>An HTML standard template</title>\n    <meta charset=\"utf-8\" />\n  </head>\n  <body>\n    <p>… Your HTML content here …</p>\n  </body>\n</html>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!DOCTYPE html PUBLIC \"-//W3C//DTD HTML 4.01//EN\" \"http://www.w3.org/TR/html4/strict.dtd\">\n<html>\n  <head>\n    <title>An HTML standard template</title>\n    <meta charset=\"utf-8\" />\n  </head>\n  <body>\n    <p>… Your HTML content here …</p>\n  </body>\n</html>");
}
#[test]
fn test_html_4_01_transitional_html_format_1_116b9d69() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 4.01 Transitional//EN\"\n  \"http://www.w3.org/TR/html4/loose.dtd\">\n<html>\n  <head>\n    <title>An HTML standard template</title>\n    <meta charset=\"utf-8\" />\n  </head>\n  <body>\n    <p>… Your HTML content here …</p>\n  </body>\n</html>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!DOCTYPE html PUBLIC \"-//W3C//DTD HTML 4.01 Transitional//EN\" \"http://www.w3.org/TR/html4/loose.dtd\">\n<html>\n  <head>\n    <title>An HTML standard template</title>\n    <meta charset=\"utf-8\" />\n  </head>\n  <body>\n    <p>… Your HTML content here …</p>\n  </body>\n</html>");
}
#[test]
fn test_html_5_html_format_1_7c7baa53() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html>\n<html>\n  <head>\n    <title>An HTML standard template</title>\n    <meta charset=\"utf-8\" />\n  </head>\n  <body>\n    <p>… Your HTML content here …</p>\n  </body>\n</html>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!doctype html>\n<html>\n  <head>\n    <title>An HTML standard template</title>\n    <meta charset=\"utf-8\" />\n  </head>\n  <body>\n    <p>… Your HTML content here …</p>\n  </body>\n</html>");
}
#[test]
fn test_ibm_system_html_format_1_d028312f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.ibm.com/data/dtd/v11/ibmxhtml1-transitional.dtd\">") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.ibm.com/data/dtd/v11/ibmxhtml1-transitional.dtd\">");
}
#[test]
fn test_legacy_string_html_format_1_1c3a86b1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<!DOCTYPE html SYSTEM \"about:legacy-compat\">");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<!DOCTYPE html SYSTEM \"about:legacy-compat\">");
}
#[test]
fn test_xhtml_1_0_frameset_html_format_1_1671808b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\n<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Frameset//EN\"\n \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-frameset.dtd\">") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Frameset//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-frameset.dtd\">");
}
#[test]
fn test_xhtml_1_0_strict_html_format_1_45865f11() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Strict//EN\"\n \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd\">") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Strict//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd\">");
}
#[test]
fn test_xhtml_1_0_transitional_html_format_1_267ed835() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\n<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\"\n \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">");
}
#[test]
fn test_xhtml_1_1_html_format_1_f81ccebb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.1//EN\" \"http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd\">\n<html xmlns=\"http://www.w3.org/1999/xhtml\">\n  <head>\n    <meta http-equiv=\"Content-Type\" content=\"text/html; charset=windows-1251\" />\n    <title>XHTML markup</title>\n  </head>\n  <body style=\"background-color:#ffffcc; color:#008800\">\n    <br />\n    <h2 align=\"center\">Sample XHTML page</h2>\n    <br />\n    <div align=\"center\">\n      <img src=\"../images/bee3.jpg\" width=\"400\" height=\"250\" alt=\"Beep\" vspace=\"20\" />\n    </div>\n    <p align=\"center\" style=\"font-size:17px\">Bar Foo,<br />\n      Foo,<br />\n      Bar<br />\n      Foo</p>\n    <p align=\"center\"><em>String</em></p>\n    <br />\n    <hr />\n  </body>\n</html>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.1//EN\" \"http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd\">\n<html xmlns=\"http://www.w3.org/1999/xhtml\">\n  <head>\n    <meta http-equiv=\"Content-Type\" content=\"text/html; charset=windows-1251\" />\n    <title>XHTML markup</title>\n  </head>\n  <body style=\"background-color: #ffffcc; color: #008800\">\n    <br />\n    <h2 align=\"center\">Sample XHTML page</h2>\n    <br />\n    <div align=\"center\">\n      <img\n        src=\"../images/bee3.jpg\"\n        width=\"400\"\n        height=\"250\"\n        alt=\"Beep\"\n        vspace=\"20\"\n      />\n    </div>\n    <p align=\"center\" style=\"font-size: 17px\">\n      Bar Foo,<br />\n      Foo,<br />\n      Bar<br />\n      Foo\n    </p>\n    <p align=\"center\"><em>String</em></p>\n    <br />\n    <hr />\n  </body>\n</html>");
}
