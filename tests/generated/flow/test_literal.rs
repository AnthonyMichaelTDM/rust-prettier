#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_multiline_js_format_1_1bf1dc7c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type loremIpsumFooBazBar1 = 'Multiline string\\\n         Multiline string'\n\ntype loremIpsumFooBazBar2 = 'Multiline string\\\n         Multiline string\\\n         Multiline string'") ? ;
    assert_eq ! (formatted , "type loremIpsumFooBazBar1 =\n  \"Multiline string\\\n         Multiline string\";\n\ntype loremIpsumFooBazBar2 =\n  \"Multiline string\\\n         Multiline string\\\n         Multiline string\";");
    Ok(())
}
