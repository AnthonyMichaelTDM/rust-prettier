#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_keyof_ts_format_1_4c2576bf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type A = keyof (T | U);\ntype B = keyof (X & Y);\ntype C = keyof T | U;\ntype D = keyof X & Y;\ntype E = (keyof T)[];\ntype F = ((keyof T))[];\ntype G = (keyof T1)[\"foo\"];\ntype H = ((keyof T1))[\"foo\"];\ntype I = (((keyof T1)))[\"foo\"];\ntype J = ((((keyof T1))))[\"foo\"];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type A = keyof (T | U);\ntype B = keyof (X & Y);\ntype C = keyof T | U;\ntype D = keyof X & Y;\ntype E = (keyof T)[];\ntype F = (keyof T)[];\ntype G = (keyof T1)[\"foo\"];\ntype H = (keyof T1)[\"foo\"];\ntype I = (keyof T1)[\"foo\"];\ntype J = (keyof T1)[\"foo\"];");
}
