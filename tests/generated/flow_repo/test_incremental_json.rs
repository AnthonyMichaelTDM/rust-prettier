#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_7bf016cd() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("/**\n * @flow\n */\nvar data = require('./data');\nvar x: number = data.x;")?;
    assert_eq!(
        formatted,
        "/**\n * @flow\n */\nvar data = require(\"./data\");\nvar x: number = data.x;"
    );
    Ok(())
}
