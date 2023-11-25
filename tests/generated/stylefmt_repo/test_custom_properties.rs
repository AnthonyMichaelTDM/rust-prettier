#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_custom_properties_css_format_1_27447fbc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format(":root{--mainColor:#123456}\na{color:var(        --mainColor     )     ; }")?;
    assert_eq!(
        formatted,
        ":root {\n  --mainColor: #123456;\n}\na {\n  color: var(--mainColor);\n}"
    );
    Ok(())
}
