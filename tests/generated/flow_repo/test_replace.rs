#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_193ee44f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("var a = 0;\n\nfunction foo(x) { }\n\nfoo(\"\");")?;
    assert_eq!(formatted, "var a = 0;\n\nfunction foo(x) {}\n\nfoo(\"\");");
    Ok(())
}
