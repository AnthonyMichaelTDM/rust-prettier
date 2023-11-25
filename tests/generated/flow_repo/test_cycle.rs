#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_27793955() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("var B = require('./B');\n\nclass A extends B { }\n\nmodule.exports = A;")?;
    assert_eq!(
        formatted,
        "var B = require(\"./B\");\n\nclass A extends B {}\n\nmodule.exports = A;"
    );
    Ok(())
}
#[test]
fn test_b_js_format_1_2d31e9fb() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("var A = require('./A');\n\n//class B extends A { }\n\nmodule.exports = B;")?;
    assert_eq!(
        formatted,
        "var A = require(\"./A\");\n\n//class B extends A { }\n\nmodule.exports = B;"
    );
    Ok(())
}
