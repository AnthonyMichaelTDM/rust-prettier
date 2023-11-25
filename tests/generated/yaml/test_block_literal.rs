#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_clip_yml_prose_wrapalways_format_1_b88694f9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("|\n    123\n    456\n    789\n\n")?;
    assert_eq!(formatted, "|\n  123\n  456\n  789");
    Ok(())
}
#[test]
fn test_clip_yml_format_1_b88694f9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("|\n    123\n    456\n    789\n\n")?;
    assert_eq!(formatted, "|\n  123\n  456\n  789");
    Ok(())
}
#[test]
fn test_indent_yml_prose_wrapalways_format_1_9329114a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("|2-\n    123\n   456\n  789\n\n")?;
    assert_eq!(formatted, "|2-\n    123\n   456\n  789");
    Ok(())
}
#[test]
fn test_indent_yml_format_1_9329114a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("|2-\n    123\n   456\n  789\n\n")?;
    assert_eq!(formatted, "|2-\n    123\n   456\n  789");
    Ok(())
}
#[test]
fn test_keep_yml_prose_wrapalways_format_1_115b98fe() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("|+\n    123\n    456\n    789\n\n")?;
    assert_eq!(formatted, "|+\n  123\n  456\n  789\n\n");
    Ok(())
}
#[test]
fn test_keep_yml_format_1_115b98fe() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("|+\n    123\n    456\n    789\n\n")?;
    assert_eq!(formatted, "|+\n  123\n  456\n  789\n\n");
    Ok(())
}
#[test]
fn test_map_yml_prose_wrapalways_format_1_99cac222() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("a: |\n  123\n  456\n  789\nb: |1\n    123\n   456\n  789\nd: |\n  123\n  456\n  789\n\nc: 0") ? ;
    assert_eq ! (formatted , "a: |\n  123\n  456\n  789\nb: |1\n    123\n   456\n  789\nd: |\n  123\n  456\n  789\n\nc: 0");
    Ok(())
}
#[test]
fn test_map_yml_format_1_99cac222() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("a: |\n  123\n  456\n  789\nb: |1\n    123\n   456\n  789\nd: |\n  123\n  456\n  789\n\nc: 0") ? ;
    assert_eq ! (formatted , "a: |\n  123\n  456\n  789\nb: |1\n    123\n   456\n  789\nd: |\n  123\n  456\n  789\n\nc: 0");
    Ok(())
}
#[test]
fn test_middle_comment_yml_prose_wrapalways_format_1_8110894a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!str #comment\n|\n  123")?;
    assert_eq!(formatted, "!!str #comment\n|\n  123");
    Ok(())
}
#[test]
fn test_middle_comment_yml_format_1_8110894a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!str #comment\n|\n  123")?;
    assert_eq!(formatted, "!!str #comment\n|\n  123");
    Ok(())
}
#[test]
fn test_middle_comments_yml_prose_wrapalways_format_1_81bc163e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!str # comment 1\n# comment 2\n|\n  123")?;
    assert_eq!(formatted, "!!str\n# comment 1\n# comment 2\n|\n  123");
    Ok(())
}
#[test]
fn test_middle_comments_yml_format_1_81bc163e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!str # comment 1\n# comment 2\n|\n  123")?;
    assert_eq!(formatted, "!!str\n# comment 1\n# comment 2\n|\n  123");
    Ok(())
}
#[test]
fn test_newline_yml_prose_wrapalways_format_1_45b7dcfa() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- |+\n  123\n  456\n  789\n\n\n\n- 0")?;
    assert_eq!(formatted, "- |+\n  123\n  456\n  789\n\n\n\n- 0");
    Ok(())
}
#[test]
fn test_newline_yml_format_1_45b7dcfa() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- |+\n  123\n  456\n  789\n\n\n\n- 0")?;
    assert_eq!(formatted, "- |+\n  123\n  456\n  789\n\n\n\n- 0");
    Ok(())
}
#[test]
fn test_newline_unaligned_yml_prose_wrapalways_format_1_46b2aca3() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("|\n  1\n  2\n    3\n    4\n  5\n  6\n\n  1\n  2\n    3\n    4\n  5\n  6\n\n\n  1\n  2\n    3\n    4\n  5\n  6\n\n  1\n  2\n\n    3\n    4\n\n  5\n  6") ? ;
    assert_eq ! (formatted , "|\n  1\n  2\n    3\n    4\n  5\n  6\n\n  1\n  2\n    3\n    4\n  5\n  6\n\n\n  1\n  2\n    3\n    4\n  5\n  6\n\n  1\n  2\n\n    3\n    4\n\n  5\n  6");
    Ok(())
}
#[test]
fn test_newline_unaligned_yml_format_1_46b2aca3() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("|\n  1\n  2\n    3\n    4\n  5\n  6\n\n  1\n  2\n    3\n    4\n  5\n  6\n\n\n  1\n  2\n    3\n    4\n  5\n  6\n\n  1\n  2\n\n    3\n    4\n\n  5\n  6") ? ;
    assert_eq ! (formatted , "|\n  1\n  2\n    3\n    4\n  5\n  6\n\n  1\n  2\n    3\n    4\n  5\n  6\n\n\n  1\n  2\n    3\n    4\n  5\n  6\n\n  1\n  2\n\n    3\n    4\n\n  5\n  6");
    Ok(())
}
#[test]
fn test_props_yml_prose_wrapalways_format_1_f87c13de() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!str &anchor |\n  123")?;
    assert_eq!(formatted, "!!str &anchor |\n  123");
    Ok(())
}
#[test]
fn test_props_yml_format_1_f87c13de() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!str &anchor |\n  123")?;
    assert_eq!(formatted, "!!str &anchor |\n  123");
    Ok(())
}
#[test]
fn test_props_in_map_yml_prose_wrapalways_format_1_9a1c9836() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: !!str &anchor |\n  123")?;
    assert_eq!(formatted, "a: !!str &anchor |\n  123");
    Ok(())
}
#[test]
fn test_props_in_map_yml_format_1_9a1c9836() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: !!str &anchor |\n  123")?;
    assert_eq!(formatted, "a: !!str &anchor |\n  123");
    Ok(())
}
#[test]
fn test_seq_yml_prose_wrapalways_format_1_cc2d326e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("- |\n  123\n  456\n  789\n- |1\n    123\n   456\n  789\n- 0")?;
    assert_eq!(
        formatted,
        "- |\n  123\n  456\n  789\n- |1\n    123\n   456\n  789\n- 0"
    );
    Ok(())
}
#[test]
fn test_seq_yml_format_1_cc2d326e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("- |\n  123\n  456\n  789\n- |1\n    123\n   456\n  789\n- 0")?;
    assert_eq!(
        formatted,
        "- |\n  123\n  456\n  789\n- |1\n    123\n   456\n  789\n- 0"
    );
    Ok(())
}
#[test]
fn test_strip_yml_prose_wrapalways_format_1_0b1ba0e3() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("|-\n    123\n    456\n    789\n\n")?;
    assert_eq!(formatted, "|-\n  123\n  456\n  789");
    Ok(())
}
#[test]
fn test_strip_yml_format_1_0b1ba0e3() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("|-\n    123\n    456\n    789\n\n")?;
    assert_eq!(formatted, "|-\n  123\n  456\n  789");
    Ok(())
}
#[test]
fn test_trailing_comment_yml_prose_wrapalways_format_1_e8a453b0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: !!str | # hello\n  hello")?;
    assert_eq!(formatted, "a: !!str | # hello\n  hello");
    Ok(())
}
#[test]
fn test_trailing_comment_yml_format_1_e8a453b0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: !!str | # hello\n  hello")?;
    assert_eq!(formatted, "a: !!str | # hello\n  hello");
    Ok(())
}
