#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_window_1_js_format_1_33b2abb1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("/*\n@providesModule Window1\n*/\nmodule.exports = window.history;")?;
    assert_eq!(
        formatted,
        "/*\n@providesModule Window1\n*/\nmodule.exports = window.history;"
    );
    Ok(())
}
#[test]
fn test_window_2_js_format_1_720e4193() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("/* @providesModule Window2 */\n\nmodule.exports = window.parent;")?;
    assert_eq!(
        formatted,
        "/* @providesModule Window2 */\n\nmodule.exports = window.parent;"
    );
    Ok(())
}
