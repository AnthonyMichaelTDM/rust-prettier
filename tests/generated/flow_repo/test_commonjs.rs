#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_abs_js_format_1_4c2ba324() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\nfunction f(x:string) { }\n\nmodule.exports = f;")?;
    assert_eq!(formatted, "function f(x: string) {}\n\nmodule.exports = f;");
    Ok(())
}
#[test]
fn test_rel_js_format_1_779c9fa7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\nvar f = require('./Abs');\n\nf(0);")?;
    assert_eq!(formatted, "var f = require(\"./Abs\");\n\nf(0);");
    Ok(())
}
