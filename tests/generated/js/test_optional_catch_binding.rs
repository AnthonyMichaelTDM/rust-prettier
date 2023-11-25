#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_optional_catch_binding_js_format_1_95b5ba03() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("try {\n\n}\ncatch {\n\n}\nfinally {\n\n}")?;
    assert_eq!(formatted, "try {\n} catch {\n} finally {\n}");
    Ok(())
}
