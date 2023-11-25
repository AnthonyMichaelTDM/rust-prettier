#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_keyof_ts_format_1_4c2576bf() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type A = keyof (T | U);\ntype B = keyof (X & Y);\ntype C = keyof T | U;\ntype D = keyof X & Y;\ntype E = (keyof T)[];\ntype F = ((keyof T))[];\ntype G = (keyof T1)[\"foo\"];\ntype H = ((keyof T1))[\"foo\"];\ntype I = (((keyof T1)))[\"foo\"];\ntype J = ((((keyof T1))))[\"foo\"];") ? ;
    assert_eq ! (formatted , "type A = keyof (T | U);\ntype B = keyof (X & Y);\ntype C = keyof T | U;\ntype D = keyof X & Y;\ntype E = (keyof T)[];\ntype F = (keyof T)[];\ntype G = (keyof T1)[\"foo\"];\ntype H = (keyof T1)[\"foo\"];\ntype I = (keyof T1)[\"foo\"];\ntype J = (keyof T1)[\"foo\"];");
    Ok(())
}
