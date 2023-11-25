#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_block_comment_json_trailing_commaall_format_1_5836cb39() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{/*comment*/\"K\":\"V\"}")?;
    assert_eq!(formatted, "{ /*comment*/ \"K\": \"V\" }");
    Ok(())
}
#[test]
fn test_block_comment_json_trailing_commaall_format_2_5836cb39() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{/*comment*/\"K\":\"V\"}")?;
    assert_eq!(formatted, "{ /*comment*/ K: \"V\" }");
    Ok(())
}
#[test]
fn test_block_comment_json_trailing_commaes_5_format_1_5836cb39() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{/*comment*/\"K\":\"V\"}")?;
    assert_eq!(formatted, "{ /*comment*/ \"K\": \"V\" }");
    Ok(())
}
#[test]
fn test_block_comment_json_trailing_commaes_5_format_2_5836cb39() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{/*comment*/\"K\":\"V\"}")?;
    assert_eq!(formatted, "{ /*comment*/ K: \"V\" }");
    Ok(())
}
#[test]
fn test_bottom_block_comment_json_trailing_commaall_format_1_4a397bb4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1 /* block-comment */")?;
    assert_eq!(formatted, "1 /* block-comment */");
    Ok(())
}
#[test]
fn test_bottom_block_comment_json_trailing_commaall_format_2_4a397bb4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1 /* block-comment */")?;
    assert_eq!(formatted, "1 /* block-comment */");
    Ok(())
}
#[test]
fn test_bottom_block_comment_json_trailing_commaes_5_format_1_4a397bb4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1 /* block-comment */")?;
    assert_eq!(formatted, "1 /* block-comment */");
    Ok(())
}
#[test]
fn test_bottom_block_comment_json_trailing_commaes_5_format_2_4a397bb4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1 /* block-comment */")?;
    assert_eq!(formatted, "1 /* block-comment */");
    Ok(())
}
#[test]
fn test_bottom_line_comment_json_trailing_commaall_format_1_d129fbd0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1 // line-comment")?;
    assert_eq!(formatted, "1 // line-comment");
    Ok(())
}
#[test]
fn test_bottom_line_comment_json_trailing_commaall_format_2_d129fbd0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1 // line-comment")?;
    assert_eq!(formatted, "1 // line-comment");
    Ok(())
}
#[test]
fn test_bottom_line_comment_json_trailing_commaes_5_format_1_d129fbd0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1 // line-comment")?;
    assert_eq!(formatted, "1 // line-comment");
    Ok(())
}
#[test]
fn test_bottom_line_comment_json_trailing_commaes_5_format_2_d129fbd0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1 // line-comment")?;
    assert_eq!(formatted, "1 // line-comment");
    Ok(())
}
#[test]
fn test_line_comment_json_trailing_commaall_format_1_f0c11fcf() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{\n  //comment\n  \"K\":\"V\"\n}")?;
    assert_eq!(formatted, "{\n  //comment\n  \"K\": \"V\"\n}");
    Ok(())
}
#[test]
fn test_line_comment_json_trailing_commaall_format_2_f0c11fcf() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{\n  //comment\n  \"K\":\"V\"\n}")?;
    assert_eq!(formatted, "{\n  //comment\n  K: \"V\",\n}");
    Ok(())
}
#[test]
fn test_line_comment_json_trailing_commaes_5_format_1_f0c11fcf() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{\n  //comment\n  \"K\":\"V\"\n}")?;
    assert_eq!(formatted, "{\n  //comment\n  \"K\": \"V\"\n}");
    Ok(())
}
#[test]
fn test_line_comment_json_trailing_commaes_5_format_2_f0c11fcf() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{\n  //comment\n  \"K\":\"V\"\n}")?;
    assert_eq!(formatted, "{\n  //comment\n  K: \"V\",\n}");
    Ok(())
}
#[test]
fn test_top_block_comment_json_trailing_commaall_format_1_60a0a710() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* comment */{\n  \"foo\": \"bar\"\n}")?;
    assert_eq!(formatted, "/* comment */ {\n  \"foo\": \"bar\"\n}");
    Ok(())
}
#[test]
fn test_top_block_comment_json_trailing_commaall_format_2_60a0a710() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* comment */{\n  \"foo\": \"bar\"\n}")?;
    assert_eq!(formatted, "/* comment */ {\n  foo: \"bar\",\n}");
    Ok(())
}
#[test]
fn test_top_block_comment_json_trailing_commaes_5_format_1_60a0a710() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* comment */{\n  \"foo\": \"bar\"\n}")?;
    assert_eq!(formatted, "/* comment */ {\n  \"foo\": \"bar\"\n}");
    Ok(())
}
#[test]
fn test_top_block_comment_json_trailing_commaes_5_format_2_60a0a710() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* comment */{\n  \"foo\": \"bar\"\n}")?;
    assert_eq!(formatted, "/* comment */ {\n  foo: \"bar\",\n}");
    Ok(())
}
#[test]
fn test_top_line_comment_json_trailing_commaall_format_1_1ce16c8b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("// comment 1\n// comment 2\n{\n  \"foo\": \"bar\"\n}")?;
    assert_eq!(
        formatted,
        "// comment 1\n// comment 2\n{\n  \"foo\": \"bar\"\n}"
    );
    Ok(())
}
#[test]
fn test_top_line_comment_json_trailing_commaall_format_2_1ce16c8b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("// comment 1\n// comment 2\n{\n  \"foo\": \"bar\"\n}")?;
    assert_eq!(
        formatted,
        "// comment 1\n// comment 2\n{\n  foo: \"bar\",\n}"
    );
    Ok(())
}
#[test]
fn test_top_line_comment_json_trailing_commaes_5_format_1_1ce16c8b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("// comment 1\n// comment 2\n{\n  \"foo\": \"bar\"\n}")?;
    assert_eq!(
        formatted,
        "// comment 1\n// comment 2\n{\n  \"foo\": \"bar\"\n}"
    );
    Ok(())
}
#[test]
fn test_top_line_comment_json_trailing_commaes_5_format_2_1ce16c8b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("// comment 1\n// comment 2\n{\n  \"foo\": \"bar\"\n}")?;
    assert_eq!(
        formatted,
        "// comment 1\n// comment 2\n{\n  foo: \"bar\",\n}"
    );
    Ok(())
}
