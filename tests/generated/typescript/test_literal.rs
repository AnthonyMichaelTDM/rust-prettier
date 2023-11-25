use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_multiline_ts_format_1_1bf1dc7c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type loremIpsumFooBazBar1 = 'Multiline string\\\\\n         Multiline string'\n\ntype loremIpsumFooBazBar2 = 'Multiline string\\\\\n         Multiline string\\\\\n         Multiline string'") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type loremIpsumFooBazBar1 = \"Multiline string\\\\\n         Multiline string\";\n\ntype loremIpsumFooBazBar2 = \"Multiline string\\\\\n         Multiline string\\\\\n         Multiline string\";");
}
