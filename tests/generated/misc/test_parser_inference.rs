#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_snippet_prettierrc_in_json_format_1_61494137() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("prettierrc")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{\"printWidth\": 100,\n\"overrides\": [\n  {\"files\": \".prettierrc\",\n    \"options\": {\"parser\": \"json\"\n  }},\n  {\"files\": \"*.js\",\n    \"options\": {\"parser\": \"babel\",\n  \"singleQuote\": true,\"printWidth\": 80,\"semi\":\nfalse,\n\"quoteProps\": \"as-needed\"\n  }}\n]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{\n  \"printWidth\": 100,\n  \"overrides\": [\n    { \"files\": \".prettierrc\", \"options\": { \"parser\": \"json\" } },\n    {\n      \"files\": \"*.js\",\n      \"options\": {\n        \"parser\": \"babel\",\n        \"singleQuote\": true,\n        \"printWidth\": 80,\n        \"semi\": false,\n        \"quoteProps\": \"as-needed\"\n      }\n    }\n  ]\n}");
}
#[test]
fn test_snippet_prettierrc_in_yaml_format_1_928a7668() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("prettierrc")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("# comment\nprintWidth: 100\noverrides:\n  - files: '.prettierrc'\n    options:\n      parser: \"json") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "# comment\nprintWidth: 100\noverrides:\n  - files: \".prettierrc\"\n    options:\n      parser: \"json\"");
}
#[test]
fn test_test_html_format_1_04464cec() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<foo>\n                 <bar/>\n</foo>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<foo>\n  <bar />\n</foo>");
}
#[test]
fn test_test_importmap_format_1_e2fefa4a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("importmap")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{\"imports\": {\n    \"prettier\": \"https://unpkg.com/prettier@2.6.2/esm/standalone.mjs\",\n    \"prettier/\": \"https://unpkg.com/prettier@2.6.2/\"\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{\n  \"imports\": {\n    \"prettier\": \"https://unpkg.com/prettier@2.6.2/esm/standalone.mjs\",\n    \"prettier/\": \"https://unpkg.com/prettier@2.6.2/\"\n  }\n}");
}
#[test]
fn test_test_js_format_1_a9979391() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo(\n                 'bar')");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo(\"bar\");");
}
#[test]
fn test_test_json_format_1_65ee3d14() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{foo:\n                 'bar'}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "{ \"foo\": \"bar\" }");
}
#[test]
fn test_test_ts_format_1_db41e511() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("type foo =\n                 'bar'");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "type foo = \"bar\";");
}
#[test]
fn test_test_wxs_format_1_4c841e11() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("wxs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("var msg =\n\"hello world\";\n\nmodule.exports.message = msg;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "var msg = \"hello world\";\n\nmodule.exports.message = msg;"
    );
}
#[test]
fn test_test_wxss_format_1_d061219c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("wxss")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("view.foo {font-size: 14rpx;\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "view.foo {\n  font-size: 14rpx;\n}");
}
#[test]
fn test_test_yml_format_1_c5ce43e6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo:\n                 - 'bar'");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo:\n  - \"bar\"");
}
