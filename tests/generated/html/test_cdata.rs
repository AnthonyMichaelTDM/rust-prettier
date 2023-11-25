#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_example_html_format_1_00315c40() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<span><![CDATA[<sender>John Smith</sender>]]></span>\n\n<span><![CDATA[1]]> a <![CDATA[2]]></span>\n<span><![CDATA[1]]> <br> <![CDATA[2]]></span>") ? ;
    assert_eq ! (formatted , "<span><![CDATA[<sender>John Smith</sender>]]></span>\n\n<span><![CDATA[1]]> a <![CDATA[2]]></span>\n<span\n  ><![CDATA[1]]> <br />\n  <![CDATA[2]]></span\n>");
    Ok(())
}
