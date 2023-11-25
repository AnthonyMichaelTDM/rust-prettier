#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_directives_and_comments_yml_format_1_8a6dde22() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("# 123\n%YAML 1.2\n# 456\n---\n# 789\ntest\n# 000")?;
    assert_eq!(
        formatted,
        "# 123\n%YAML 1.2\n# 456\n---\n# 789\ntest\n# 000"
    );
    Ok(())
}
#[test]
fn test_separator_yml_format_1_8a0f565b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("a\n---\nb\n...\nc\n... #\nd\n...\n---\ne\n...\n#\n---\nf\n--- #\ng")?;
    assert_eq!(
        formatted,
        "a\n---\nb\n---\nc\n... #\nd\n---\ne\n...\n#\n---\nf\n--- #\ng"
    );
    Ok(())
}
#[test]
fn test_with_document_head_yml_format_1_cd892de7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\n123")?;
    assert_eq!(formatted, "---\n123");
    Ok(())
}
#[test]
fn test_with_document_head_like_yml_format_1_b940c45a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---666\n123")?;
    assert_eq!(formatted, "---666\n123");
    Ok(())
}
#[test]
fn test_without_document_head_yml_format_1_b481bb58() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("123")?;
    assert_eq!(formatted, "123");
    Ok(())
}
