#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_items_yml_tab_width_4_format_1_54f3cb90() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("- 123\n- 456\n- 789");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- 123\n- 456\n- 789");
}
#[test]
fn test_items_yml_format_1_54f3cb90() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("- 123\n- 456\n- 789");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- 123\n- 456\n- 789");
}
#[test]
fn test_middle_comment_yml_tab_width_4_format_1_8054a673() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("!!set # comment\n- 123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!!set # comment\n- 123");
}
#[test]
fn test_middle_comment_yml_format_1_8054a673() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("!!set # comment\n- 123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!!set # comment\n- 123");
}
#[test]
fn test_middle_comments_yml_tab_width_4_format_1_d0c2d195() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("!!set # comment 1\n# comment 2\n- 123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!!set\n# comment 1\n# comment 2\n- 123");
}
#[test]
fn test_middle_comments_yml_format_1_d0c2d195() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("!!set # comment 1\n# comment 2\n- 123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!!set\n# comment 1\n# comment 2\n- 123");
}
#[test]
fn test_nested_yml_tab_width_4_format_1_2f1dfd53() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("- 1\n- - 2-1\n  - 2-2\n- - - 3-1-1\n    - 3-2-1");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- 1\n- - 2-1\n  - 2-2\n- - - 3-1-1\n    - 3-2-1");
}
#[test]
fn test_nested_yml_format_1_2f1dfd53() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("- 1\n- - 2-1\n  - 2-2\n- - - 3-1-1\n    - 3-2-1");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- 1\n- - 2-1\n  - 2-2\n- - - 3-1-1\n    - 3-2-1");
}
#[test]
fn test_null_item_yml_tab_width_4_format_1_ac859d67() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("-");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "-");
}
#[test]
fn test_null_item_yml_format_1_ac859d67() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("-");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "-");
}
#[test]
fn test_props_yml_tab_width_4_format_1_37406e03() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("--- !!set &anchor\n- 123\n- 456");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "---\n!!set &anchor\n- 123\n- 456");
}
#[test]
fn test_props_yml_format_1_37406e03() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("--- !!set &anchor\n- 123\n- 456");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "---\n!!set &anchor\n- 123\n- 456");
}
#[test]
fn test_props_in_map_yml_tab_width_4_format_1_4c9feb5d() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("a: !!set &anchor\n- 123\n- 456");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "a: !!set &anchor\n    - 123\n    - 456");
}
#[test]
fn test_props_in_map_yml_format_1_4c9feb5d() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("a: !!set &anchor\n- 123\n- 456");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "a: !!set &anchor\n  - 123\n  - 456");
}
