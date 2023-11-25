use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_case_html_format_1_5a2c9262() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html>\n<HTML CLASS=\"no-js mY-ClAsS\">\n  <HEAD>\n    <META CHARSET=\"utf-8\">\n    <TITLE>My tITlE</TITLE>\n    <META NAME=\"description\" content=\"My CoNtEnT\">\n  </HEAD>\n  <body>\n    <P>Hello world!<BR> This is HTML5 Boilerplate.</P>\n    <SCRIPT>\n      window.ga = function () { ga.q.push(arguments) }; ga.q = []; ga.l = +new Date;\n      ga('create', 'UA-XXXXX-Y', 'auto'); ga('send', 'pageview')\n    </SCRIPT>\n    <SCRIPT src=\"https://www.google-analytics.com/analytics.js\" ASYNC DEFER></SCRIPT>\n  </body>\n</HTML>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!doctype html>\n<html class=\"no-js mY-ClAsS\">\n  <head>\n    <meta charset=\"utf-8\" />\n    <title>My tITlE</title>\n    <meta name=\"description\" content=\"My CoNtEnT\" />\n  </head>\n  <body>\n    <p>\n      Hello world!<br />\n      This is HTML5 Boilerplate.\n    </p>\n    <script>\n      window.ga = function () {\n        ga.q.push(arguments);\n      };\n      ga.q = [];\n      ga.l = +new Date();\n      ga(\"create\", \"UA-XXXXX-Y\", \"auto\");\n      ga(\"send\", \"pageview\");\n    </script>\n    <script\n      src=\"https://www.google-analytics.com/analytics.js\"\n      async\n      defer\n    ></script>\n  </body>\n</html>");
}
