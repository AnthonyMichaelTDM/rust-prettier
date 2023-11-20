#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_369283d3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("1_1\n1_1.1_1\n0o1_1\n0o0_11\n1.1_0_1e1\n1.1_0_1E1\n.1_1\n0x1_1\n0xa_1\n0xA_1\n0b01_1\n0b0_1_1") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "1_1;\n1_1.1_1;\n0o1_1;\n0o0_11;\n1.1_0_1e1;\n1.1_0_1e1;\n0.1_1;\n0x1_1;\n0xa_1;\n0xa_1;\n0b01_1;\n0b0_1_1;");
}
