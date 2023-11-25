#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_complex_md_prose_wrapalways_format_1_01facc8f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("---\n- hello:    world\n-         123\n---\n\n# something")?;
    assert_eq!(formatted, "---\n- hello: world\n- 123\n---\n\n# something");
    Ok(())
}
#[test]
fn test_empty_md_prose_wrapalways_format_1_4b12a98d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\n---")?;
    assert_eq!(formatted, "---\n---");
    Ok(())
}
#[test]
fn test_empty_2_md_prose_wrapalways_format_1_00327d87() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\n---\n\nContent")?;
    assert_eq!(formatted, "---\n---\n\nContent");
    Ok(())
}
#[test]
fn test_simple_md_prose_wrapalways_format_1_d199ddb9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\nhello: world\n---")?;
    assert_eq!(formatted, "---\nhello: world\n---");
    Ok(())
}
#[test]
fn test_simple_2_md_prose_wrapalways_format_1_b7c4c886() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\nhello: world\n---\n\nContent")?;
    assert_eq!(formatted, "---\nhello: world\n---\n\nContent");
    Ok(())
}
#[test]
fn test_trailing_spaces_md_prose_wrapalways_format_1_cb3662ac() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("---\n           v spaces\n---         \n\nThis paragraph should be considered part of the _markdown_ instead of *yaml*.\n\n---") ? ;
    assert_eq ! (formatted , "---\nv spaces\n---\n\nThis paragraph should be considered part of the _markdown_ instead of _yaml_.\n\n---");
    Ok(())
}
