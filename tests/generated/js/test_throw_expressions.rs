#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_throw_expression_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_throw_expression_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_throw_expression_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_throw_expression_js_format_1_086c96b5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function save(filename = throw new TypeError(\"Argument required\")) {}\n\nlint(ast, {\n  with: () => throw new Error(\"avoid using 'with' statements.\")\n});\n\nfunction getEncoder(encoding) {\n  const encoder = encoding === \"utf8\" ? new UTF8Encoder()\n                : encoding === \"utf16le\" ? new UTF16Encoder(false)\n                : encoding === \"utf16be\" ? new UTF16Encoder(true)\n                : throw new Error(\"Unsupported encoding\");\n}\n\nclass Product {\n  get id() { return this._id; }\n  set id(value) { this._id = value || throw new Error(\"Invalid value\"); }\n}") ? ;
    assert_eq ! (formatted , "function save(filename = throw new TypeError(\"Argument required\")) {}\n\nlint(ast, {\n  with: () => throw new Error(\"avoid using 'with' statements.\"),\n});\n\nfunction getEncoder(encoding) {\n  const encoder =\n    encoding === \"utf8\"\n      ? new UTF8Encoder()\n      : encoding === \"utf16le\"\n        ? new UTF16Encoder(false)\n        : encoding === \"utf16be\"\n          ? new UTF16Encoder(true)\n          : throw new Error(\"Unsupported encoding\");\n}\n\nclass Product {\n  get id() {\n    return this._id;\n  }\n  set id(value) {\n    this._id = value || throw new Error(\"Invalid value\");\n  }\n}");
    Ok(())
}
