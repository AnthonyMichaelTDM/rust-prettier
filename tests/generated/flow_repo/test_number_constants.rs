#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_number_constants_js_format_1_77b2274d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var a: number = Number.MAX_SAFE_INTEGER;\nvar b: string = Number.MAX_SAFE_INTEGER;\nvar c: number = Number.MIN_SAFE_INTEGER;\nvar d: string = Number.MIN_SAFE_INTEGER;\nvar e: number = Number.MAX_VALUE;\nvar f: string = Number.MAX_VALUE;\nvar g: number = Number.MIN_VALUE;\nvar h: string = Number.MIN_VALUE;\nvar i: number = Number.NaN;\nvar j: string = Number.NaN;\nvar k: number = Number.EPSILON;\nvar l: string = Number.EPSILON;") ? ;
    assert_eq ! (formatted , "var a: number = Number.MAX_SAFE_INTEGER;\nvar b: string = Number.MAX_SAFE_INTEGER;\nvar c: number = Number.MIN_SAFE_INTEGER;\nvar d: string = Number.MIN_SAFE_INTEGER;\nvar e: number = Number.MAX_VALUE;\nvar f: string = Number.MAX_VALUE;\nvar g: number = Number.MIN_VALUE;\nvar h: string = Number.MIN_VALUE;\nvar i: number = Number.NaN;\nvar j: string = Number.NaN;\nvar k: number = Number.EPSILON;\nvar l: string = Number.EPSILON;");
    Ok(())
}
