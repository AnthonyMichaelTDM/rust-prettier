#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_req_js_format_1_fe7a0162() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("module.exports = 0;")?;
    assert_eq!(formatted, "module.exports = 0;");
    Ok(())
}
#[test]
fn test_test_js_format_1_40f006e7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("var x = require('./req');\n(x: string);\n\nimport x from './req';")?;
    assert_eq!(
        formatted,
        "var x = require(\"./req\");\n(x: string);\n\nimport x from \"./req\";"
    );
    Ok(())
}
