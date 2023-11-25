#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_items_yml_tab_width_4_format_1_54f3cb90() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- 123\n- 456\n- 789")?;
    assert_eq!(formatted, "- 123\n- 456\n- 789");
    Ok(())
}
#[test]
fn test_items_yml_format_1_54f3cb90() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- 123\n- 456\n- 789")?;
    assert_eq!(formatted, "- 123\n- 456\n- 789");
    Ok(())
}
#[test]
fn test_middle_comment_yml_tab_width_4_format_1_8054a673() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!set # comment\n- 123")?;
    assert_eq!(formatted, "!!set # comment\n- 123");
    Ok(())
}
#[test]
fn test_middle_comment_yml_format_1_8054a673() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!set # comment\n- 123")?;
    assert_eq!(formatted, "!!set # comment\n- 123");
    Ok(())
}
#[test]
fn test_middle_comments_yml_tab_width_4_format_1_d0c2d195() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!set # comment 1\n# comment 2\n- 123")?;
    assert_eq!(formatted, "!!set\n# comment 1\n# comment 2\n- 123");
    Ok(())
}
#[test]
fn test_middle_comments_yml_format_1_d0c2d195() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!set # comment 1\n# comment 2\n- 123")?;
    assert_eq!(formatted, "!!set\n# comment 1\n# comment 2\n- 123");
    Ok(())
}
#[test]
fn test_nested_yml_tab_width_4_format_1_2f1dfd53() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- 1\n- - 2-1\n  - 2-2\n- - - 3-1-1\n    - 3-2-1")?;
    assert_eq!(formatted, "- 1\n- - 2-1\n  - 2-2\n- - - 3-1-1\n    - 3-2-1");
    Ok(())
}
#[test]
fn test_nested_yml_format_1_2f1dfd53() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- 1\n- - 2-1\n  - 2-2\n- - - 3-1-1\n    - 3-2-1")?;
    assert_eq!(formatted, "- 1\n- - 2-1\n  - 2-2\n- - - 3-1-1\n    - 3-2-1");
    Ok(())
}
#[test]
fn test_null_item_yml_tab_width_4_format_1_ac859d67() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("-")?;
    assert_eq!(formatted, "-");
    Ok(())
}
#[test]
fn test_null_item_yml_format_1_ac859d67() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("-")?;
    assert_eq!(formatted, "-");
    Ok(())
}
#[test]
fn test_props_yml_tab_width_4_format_1_37406e03() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("--- !!set &anchor\n- 123\n- 456")?;
    assert_eq!(formatted, "---\n!!set &anchor\n- 123\n- 456");
    Ok(())
}
#[test]
fn test_props_yml_format_1_37406e03() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("--- !!set &anchor\n- 123\n- 456")?;
    assert_eq!(formatted, "---\n!!set &anchor\n- 123\n- 456");
    Ok(())
}
#[test]
fn test_props_in_map_yml_tab_width_4_format_1_4c9feb5d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: !!set &anchor\n- 123\n- 456")?;
    assert_eq!(formatted, "a: !!set &anchor\n    - 123\n    - 456");
    Ok(())
}
#[test]
fn test_props_in_map_yml_format_1_4c9feb5d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: !!set &anchor\n- 123\n- 456")?;
    assert_eq!(formatted, "a: !!set &anchor\n  - 123\n  - 456");
    Ok(())
}
