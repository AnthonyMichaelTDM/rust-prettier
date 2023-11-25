#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_comment_after_css_format_1_31c782cb() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "---\ntitle: Title\ndescription: Description\n---\n/* comment */\n.something {\n}",
    )?;
    assert_eq!(
        formatted,
        "---\ntitle: Title\ndescription: Description\n---\n\n/* comment */\n.something {\n}"
    );
    Ok(())
}
#[test]
fn test_dirty_css_format_1_5f9b2d9e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("---\nhello:     world\na:       \n            - 123\n            - 666\n---\n\n.class {\n\n\n\n}") ? ;
    assert_eq!(
        formatted,
        "---\nhello: world\na:\n  - 123\n  - 666\n---\n\n.class {\n}"
    );
    Ok(())
}
#[test]
fn test_empty_css_format_1_307e4f77() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\n---\n\na {\n    color: red;\n}")?;
    assert_eq!(formatted, "---\n---\n\na {\n  color: red;\n}");
    Ok(())
}
#[test]
fn test_empty_newlines_css_format_1_90a289a8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\n\n\n\n---\n\n\n\na {\n    color: red;\n}")?;
    assert_eq!(formatted, "---\n---\n\na {\n  color: red;\n}");
    Ok(())
}
#[test]
fn test_ignore_css_format_1_cb2176e2() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("---\nhello: world\n---\n\n/* prettier-ignore */\n.foo {}")?;
    assert_eq!(
        formatted,
        "---\nhello: world\n---\n\n/* prettier-ignore */\n.foo {}"
    );
    Ok(())
}
#[test]
fn test_malformed_css_format_1_f9a8e381() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\naaa\nb---\n\na {\n    color: red;\n")?;
    assert_eq!(formatted, "--- aaa b--- a {\n  color: red;\n}");
    Ok(())
}
#[test]
fn test_malformed_2_css_format_1_d1f261de() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("---\nfoo: bar\n---\n\na {\ncolor:blue\n}\n\n---\n\n.b {\ncolor:red\n")?;
    assert_eq!(
        formatted,
        "---\nfoo: bar\n---\n\na {\n  color: blue;\n}\n\n--- .b {\n  color: red;\n}"
    );
    Ok(())
}
#[test]
fn test_only_comments_css_format_1_7f79c2d7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("---\n# comment 1\n# comment 2\n# comment 3\n---\n\na {\n    color: red;\n}")?;
    assert_eq!(
        formatted,
        "---\n# comment 1\n# comment 2\n# comment 3\n---\n\na {\n  color: red;\n}"
    );
    Ok(())
}
#[test]
fn test_with_comments_css_format_1_d40a1828() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("---\ntitle: Title\ndescription: Description\n# This is a comment\n---\n\na {\n    color: red;\n") ? ;
    assert_eq ! (formatted , "---\ntitle: Title\ndescription: Description\n# This is a comment\n---\n\na {\n  color: red;\n}");
    Ok(())
}
#[test]
fn test_without_newline_after_css_format_1_0f0fa0cf() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("---\ntitle: Title\ndescription: Description\n---\na {\n    color: red;\n}")?;
    assert_eq!(
        formatted,
        "---\ntitle: Title\ndescription: Description\n---\n\na {\n  color: red;\n}"
    );
    Ok(())
}
#[test]
fn test_yaml_css_format_1_56103c4d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("---\ntitle: Title\ndescription: Description\n---\n\na {\n    color: red;\n}")?;
    assert_eq!(
        formatted,
        "---\ntitle: Title\ndescription: Description\n---\n\na {\n  color: red;\n}"
    );
    Ok(())
}
