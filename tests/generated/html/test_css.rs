#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_empty_html_format_1_36ccab9b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<style></style>")?;
    assert_eq!(formatted, "<style></style>");
    Ok(())
}
#[test]
fn test_less_html_format_1_7c8c7840() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<style type=\"text/less\">\n  @nice-blue: #5B83AD;\n  @light-blue: @nice-blue + #111;\n\n  #header {\n    color: @light-blue;\n  }\n</style>\n\n<style lang=\"less\">\n  @nice-blue: #5B83AD;\n  @light-blue: @nice-blue + #111;\n\n  #header {\n    color: @light-blue;\n  }\n</style>") ? ;
    assert_eq ! (formatted , "<style type=\"text/less\">\n  @nice-blue: #5B83AD;\n  @light-blue: @nice-blue + #111;\n\n  #header {\n    color: @light-blue;\n  }\n</style>\n\n<style lang=\"less\">\n  @nice-blue: #5b83ad;\n  @light-blue: @nice-blue + #111;\n\n  #header {\n    color: @light-blue;\n  }\n</style>");
    Ok(())
}
#[test]
fn test_postcss_html_format_1_a3ddc168() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<style type=\"text/css\">\n  body { background: navy; color: yellow; }\n</style>\n\n<style lang=\"postcss\">\n  body { background: navy; color: yellow; }\n</style>") ? ;
    assert_eq ! (formatted , "<style type=\"text/css\">\n  body {\n    background: navy;\n    color: yellow;\n  }\n</style>\n\n<style lang=\"postcss\">\n  body {\n    background: navy;\n    color: yellow;\n  }\n</style>");
    Ok(())
}
#[test]
fn test_scss_html_format_1_41b27012() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<style type=\"text/x-scss\">\n  $font-stack:    Helvetica, sans-serif;\n  $primary-color: #333;\n\n  body {\n    font: 100% $font-stack;\n    color: $primary-color;\n  }\n</style>\n\n<style lang=\"scss\">\n  $font-stack:    Helvetica, sans-serif;\n  $primary-color: #333;\n\n  body {\n    font: 100% $font-stack;\n    color: $primary-color;\n  }\n</style>\n\n<style lang=\"scss\">\n.someElement {\n    @include bp-medium {\n      display: flex;\n    }\n    \n    @include bp-large {\n      margin-top: 10px;\n      margin-bottom: 10px;\n    }\n}\n</style>") ? ;
    assert_eq ! (formatted , "<style type=\"text/x-scss\">\n  $font-stack: Helvetica, sans-serif;\n  $primary-color: #333;\n\n  body {\n    font: 100% $font-stack;\n    color: $primary-color;\n  }\n</style>\n\n<style lang=\"scss\">\n  $font-stack: Helvetica, sans-serif;\n  $primary-color: #333;\n\n  body {\n    font: 100% $font-stack;\n    color: $primary-color;\n  }\n</style>\n\n<style lang=\"scss\">\n  .someElement {\n    @include bp-medium {\n      display: flex;\n    }\n\n    @include bp-large {\n      margin-top: 10px;\n      margin-bottom: 10px;\n    }\n  }\n</style>");
    Ok(())
}
#[test]
fn test_simple_html_format_1_9cef6ba1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html>\n<html>\n  <head>\n    <title>Sample styled page</title>\n    <style>a { color: red; }</style>\n    <style>\n      body { background: navy; color: yellow; }\n    </style>\n  </head>\n  <body>\n    <h1>Sample styled page</h1>\n    <p>This page is just a demo.</p>\n  </body>\n</html>") ? ;
    assert_eq ! (formatted , "<!doctype html>\n<html>\n  <head>\n    <title>Sample styled page</title>\n    <style>\n      a {\n        color: red;\n      }\n    </style>\n    <style>\n      body {\n        background: navy;\n        color: yellow;\n      }\n    </style>\n  </head>\n  <body>\n    <h1>Sample styled page</h1>\n    <p>This page is just a demo.</p>\n  </body>\n</html>");
    Ok(())
}
#[test]
fn test_single_style_html_format_1_4824187e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<style>a { color: red; }</style>\n<style>\n  h1 {\n    font-size: 120%;\n    font-family: Verdana, Arial, Helvetica, sans-serif;\n    color: #333366;\n  }\n</style>") ? ;
    assert_eq ! (formatted , "<style>\n  a {\n    color: red;\n  }\n</style>\n<style>\n  h1 {\n    font-size: 120%;\n    font-family: Verdana, Arial, Helvetica, sans-serif;\n    color: #333366;\n  }\n</style>");
    Ok(())
}
