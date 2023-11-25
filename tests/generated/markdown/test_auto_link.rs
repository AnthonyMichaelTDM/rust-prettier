#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_7b6ac6e8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("example.com:80/_a")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_ http://www.example.com:80/_a ")?;
    assert_eq!(formatted, "_ http://www.example.com:80/_a _");
    Ok(())
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_8d9b49e4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("example.com:80/_a_")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_ http://www.example.com:80/_a_ ")?;
    assert_eq!(formatted, "_ http://www.example.com:80/_a_ \\_");
    Ok(())
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_bd846ff9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("example.com:80/_a_")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_ http://www.example.com:80/_a")?;
    assert_eq!(formatted, "_ http://www.example.com:80/_a_");
    Ok(())
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_0d0803e2() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("example.com:80/_a_/")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_ http://www.example.com:80/_a_/ ")?;
    assert_eq!(formatted, "_ http://www.example.com:80/_a_/ \\_");
    Ok(())
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_157040bf() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("example.com:80/_a_/_")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_ http://www.example.com:80/_a_/_ ")?;
    assert_eq!(formatted, "_ http://www.example.com:80/_a_/\\_ \\_");
    Ok(())
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_b48088c9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("example.com:80/_a_/_")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_ http://www.example.com:80/_a_/")?;
    assert_eq!(formatted, "_ http://www.example.com:80/_a_/\\_");
    Ok(())
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_8bd96f50() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("example.com:80/_a_/__")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_ http://www.example.com:80/_a_/_")?;
    assert_eq!(formatted, "_ http://www.example.com:80/_a_/\\_\\_");
    Ok(())
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_8923996e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("example.com:80/_a__")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_ http://www.example.com:80/_a__ ")?;
    assert_eq!(formatted, "_ http://www.example.com:80/_a__ _");
    Ok(())
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_2e4043dd() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("example.com:80/_a__")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_ http://www.example.com:80/_a_")?;
    assert_eq!(formatted, "\\_ http://www.example.com:80/_a__");
    Ok(())
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_90ba1d99() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("example.com:80/_a___")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_ http://www.example.com:80/_a__")?;
    assert_eq!(formatted, "\\_ http://www.example.com:80/_a___");
    Ok(())
}
#[test]
fn test_snippet_httpwww_example_com_80_a_b_format_1_dc86dfcf() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("example.com:80/_a_b")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_ http://www.example.com:80/_a_b ")?;
    assert_eq!(formatted, "_ http://www.example.com:80/_a_b _");
    Ok(())
}
#[test]
fn test_snippet_httpwww_example_com_80_a_b_format_1_85c0ee84() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("example.com:80/_a_b_")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_ http://www.example.com:80/_a_b")?;
    assert_eq!(formatted, "_ http://www.example.com:80/_a_b_");
    Ok(())
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_55ebe5ee() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("example.com:80/_a")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_http://www.example.com:80/_a ")?;
    assert_eq!(formatted, "_http://www.example.com:80/_a _");
    Ok(())
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_d7f96584() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("example.com:80/_a_")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_http://www.example.com:80/_a_ ")?;
    assert_eq!(formatted, "_http://www.example.com:80/_a_ \\_");
    Ok(())
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_dad5a8fc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("example.com:80/_a_")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_http://www.example.com:80/_a")?;
    assert_eq!(formatted, "_http://www.example.com:80/_a_");
    Ok(())
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_d2bca30f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("example.com:80/_a_/")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_http://www.example.com:80/_a_/ ")?;
    assert_eq!(formatted, "_http://www.example.com:80/_a_/ \\_");
    Ok(())
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_ea89d285() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("example.com:80/_a_/_")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_http://www.example.com:80/_a_/_ ")?;
    assert_eq!(formatted, "_http://www.example.com:80/_a_/\\_ \\_");
    Ok(())
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_a4670c40() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("example.com:80/_a_/_")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_http://www.example.com:80/_a_/")?;
    assert_eq!(formatted, "_http://www.example.com:80/_a_/\\_");
    Ok(())
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_3a2341ff() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("example.com:80/_a_/__")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_http://www.example.com:80/_a_/_")?;
    assert_eq!(formatted, "_http://www.example.com:80/_a_/\\_\\_");
    Ok(())
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_9d1c75ea() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("example.com:80/_a__")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_http://www.example.com:80/_a__ ")?;
    assert_eq!(formatted, "_http://www.example.com:80/_a__ _");
    Ok(())
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_54678ec1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("example.com:80/_a__")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_http://www.example.com:80/_a_")?;
    assert_eq!(formatted, "\\_http://www.example.com:80/_a__");
    Ok(())
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_b7422d5c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("example.com:80/_a___")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_http://www.example.com:80/_a__")?;
    assert_eq!(formatted, "\\_http://www.example.com:80/_a___");
    Ok(())
}
#[test]
fn test_snippet_httpwww_example_com_80_a_b_format_1_c9c09612() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("example.com:80/_a_b")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_http://www.example.com:80/_a_b ")?;
    assert_eq!(formatted, "_http://www.example.com:80/_a_b _");
    Ok(())
}
#[test]
fn test_snippet_httpwww_example_com_80_a_b_format_1_4ff6fbbe() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("example.com:80/_a_b_")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_http://www.example.com:80/_a_b")?;
    assert_eq!(formatted, "_http://www.example.com:80/_a_b_");
    Ok(())
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_266da82c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("example.com:80/_a")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("http://www.example.com:80/_")?;
    assert_eq!(formatted, "http://www.example.com:80/_a");
    Ok(())
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_d59dc3a3() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("example.com:80/_a_")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("http://www.example.com:80/_a")?;
    assert_eq!(formatted, "http://www.example.com:80/_a_");
    Ok(())
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_dc5b174e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("example.com:80/_a_/")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("http://www.example.com:80/_a_")?;
    assert_eq!(formatted, "http://www.example.com:80/_a_/");
    Ok(())
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_39280ae3() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("example.com:80/_a_/_")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("http://www.example.com:80/_a_/")?;
    assert_eq!(formatted, "http://www.example.com:80/_a_/_");
    Ok(())
}
#[test]
fn test_snippet_httpwww_example_com_80_a_format_1_54137733() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("example.com:80/_a__")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("http://www.example.com:80/_a_")?;
    assert_eq!(formatted, "http://www.example.com:80/_a__");
    Ok(())
}
#[test]
fn test_snippet_httpwww_example_com_80_a_b_format_1_9013d69a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("example.com:80/_a_b")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("http://www.example.com:80/_a_")?;
    assert_eq!(formatted, "http://www.example.com:80/_a_b");
    Ok(())
}
