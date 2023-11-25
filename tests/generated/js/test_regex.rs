#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_d_flag_js_format_1_ddeb0f21() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/./d;")?;
    assert_eq!(formatted, "/./d;");
    Ok(())
}
#[test]
fn test_multiple_flags_js_format_1_671448e6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/.*/ms;\n/.*/my;")?;
    assert_eq!(formatted, "/.*/ms;\n/.*/my;");
    Ok(())
}
#[test]
fn test_regexp_modifiers_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_regexp_modifiers_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_regexp_modifiers_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_regexp_modifiers_js_format_1_5daeaa9a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("/(?ims:^[a-z])/u;\n/(?-ims:^[a-z].)(^[a-z].)/uims;\n/(?ims:^[a-z].1$)/;")?;
    assert_eq!(
        formatted,
        "/(?ims:^[a-z])/u;\n/(?-ims:^[a-z].)(^[a-z].)/imsu;\n/(?ims:^[a-z].1$)/;"
    );
    Ok(())
}
#[test]
fn test_test_js_format_1_ce890b42() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/[/]\\\\/\\\\u0aBc/mgi;")?;
    assert_eq!(formatted, "/[/]\\\\/\\\\u0aBc/gim;");
    Ok(())
}
#[test]
fn test_v_flag_js_flow_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_v_flag_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_v_flag_js_format_1_b3692630() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/a/v;")?;
    assert_eq!(formatted, "/a/v;");
    Ok(())
}
