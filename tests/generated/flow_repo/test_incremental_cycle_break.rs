#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_e59a20bb() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("/**\n * @providesModule A\n * @flow\n */\n\nrequire('B');")?;
    assert_eq!(
        formatted,
        "/**\n * @providesModule A\n * @flow\n */\n\nrequire(\"B\");"
    );
    Ok(())
}
#[test]
fn test_b_js_format_1_664cdacc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("/**\n * @providesModule B\n * @flow\n */\nrequire('A');")?;
    assert_eq!(
        formatted,
        "/**\n * @providesModule B\n * @flow\n */\nrequire(\"A\");"
    );
    Ok(())
}
