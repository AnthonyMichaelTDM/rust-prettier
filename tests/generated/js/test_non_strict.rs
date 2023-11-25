#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_argument_name_clash_js_format_1_b6f4c0da() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("function f(a,a){return a}")?;
    assert_eq!(formatted, "function f(a, a) {\n  return a;\n}");
    Ok(())
}
#[test]
fn test_keywords_js_format_1_ad0425aa() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var package = require('../package');\n\n/**\n * My amazing comment\n */\nfunction myFunction() {\n\treturn 'StringGainz';\n}") ? ;
    assert_eq ! (formatted , "var package = require(\"../package\");\n\n/**\n * My amazing comment\n */\nfunction myFunction() {\n  return \"StringGainz\";\n}");
    Ok(())
}
#[test]
fn test_octal_number_js_typescript_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_octal_number_js_format_1_030ed232() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("0777")?;
    assert_eq!(formatted, "0777;");
    Ok(())
}
