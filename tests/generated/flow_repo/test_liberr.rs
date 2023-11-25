#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_67b490a6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n// one error here, to verify lib errors sort to top.\nvar x: string = 0;") ? ;
    assert_eq ! (formatted , "/**\n * @flow\n */\n// one error here, to verify lib errors sort to top.\nvar x: string = 0;");
    Ok(())
}
