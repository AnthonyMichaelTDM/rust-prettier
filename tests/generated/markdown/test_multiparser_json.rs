#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_invalid_json_md_trailing_commaall_format_1_422b1e96() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!-- #10333 -->\n\\`\\`\\`json\npackages\\\\the-hub\\\\cypress\\\\fixtures\\\\gridConfiguration.json\n\\`\\`\\") ? ;
    assert_eq ! (formatted , "<!-- #10333 -->\n\n\\`\\`\\`json\npackages\\\\the-hub\\\\cypress\\\\fixtures\\\\gridConfiguration.json\n\\`\\`\\`");
    Ok(())
}
#[test]
fn test_jsonc_md_trailing_commaall_format_1_e0a09a39() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("\\`\\`\\`jsonc\n   // comment\n{\n     \"foo\"   :    \"bar\"\n}\n\\`\\`\\`")?;
    assert_eq!(
        formatted,
        "\\`\\`\\`jsonc\n// comment\n{\n  \"foo\": \"bar\"\n}\n\\`\\`\\`"
    );
    Ok(())
}
#[test]
fn test_trailing_comma_md_trailing_commaall_format_1_6911bac1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`\\`\\`json\n{\n  \"foo\": \"bar\"\n}\n\\`\\`\\`")?;
    assert_eq!(
        formatted,
        "\\`\\`\\`json\n{\n  \"foo\": \"bar\"\n}\n\\`\\`\\`"
    );
    Ok(())
}
