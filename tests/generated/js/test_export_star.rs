#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_export_star_js_format_1_e1d889ee() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("export * from \"mod\";\n")?;
    assert_eq!(formatted, "export * from \"mod\";");
    Ok(())
}
#[test]
fn test_export_star_as_js_format_1_409a990f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("export * as ns from \"mod\";\n")?;
    assert_eq!(formatted, "export * as ns from \"mod\";");
    Ok(())
}
#[test]
fn test_export_star_as_default_js_format_1_9380d522() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("export * as default from 'foo'")?;
    assert_eq!(formatted, "export * as default from \"foo\";");
    Ok(())
}
#[test]
fn test_export_star_as_reserved_word_js_format_1_bf8dc141() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export * as function from 'foo'\nexport * as const from 'foo'\nexport * as as from 'foo'\nexport * as from from 'foo'") ? ;
    assert_eq ! (formatted , "export * as function from \"foo\";\nexport * as const from \"foo\";\nexport * as as from \"foo\";\nexport * as from from \"foo\";");
    Ok(())
}
#[test]
fn test_export_star_as_string_js_flow_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_export_star_as_string_js_typescript_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_export_star_as_string_js_format_1_e3ecf295() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("export * as 'foo' from 'foo'")?;
    assert_eq!(formatted, "export * as \"foo\" from \"foo\";");
    Ok(())
}
#[test]
fn test_export_star_as_string_2_js_flow_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_export_star_as_string_2_js_typescript_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_export_star_as_string_2_js_format_1_2b16fe8b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("export * as \"foo\" from 'foo'")?;
    assert_eq!(formatted, "export * as \"foo\" from \"foo\";");
    Ok(())
}
