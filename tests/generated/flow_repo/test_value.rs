#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_value_js_format_1_ab5855fd() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var o = {};\no[\"x\"] = 4;\nvar y:string = o[\"x\"];\n\nvar table: { [_: string]: number } = {};\ntable[\"x\"] = \"hello\";") ? ;
    assert_eq ! (formatted , "var o = {};\no[\"x\"] = 4;\nvar y: string = o[\"x\"];\n\nvar table: { [_: string]: number } = {};\ntable[\"x\"] = \"hello\";");
    Ok(())
}
