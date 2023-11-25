#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_249b374f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("var o = Object.freeze({ foo: 0 });\n(o.foo: string);\nmodule.exports = o;")?;
    assert_eq!(
        formatted,
        "var o = Object.freeze({ foo: 0 });\n(o.foo: string);\nmodule.exports = o;"
    );
    Ok(())
}
#[test]
fn test_test_2_js_format_1_41bfbc0a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("var o = require('./test');\n(o.foo: string);")?;
    assert_eq!(formatted, "var o = require(\"./test\");\n(o.foo: string);");
    Ok(())
}
