#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_custom_parser_html_format_1_a651ce11() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "---mycustomparser\n  \ntitle: Hello\nslug: home\n\n---\n\n<h1>\n  Hello world!</h1>",
    )?;
    assert_eq!(
        formatted,
        "---mycustomparser\n  \ntitle: Hello\nslug: home\n\n---\n\n<h1>Hello world!</h1>"
    );
    Ok(())
}
#[test]
fn test_empty_html_format_1_97355988() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\n---\n\n<h1>\n  Hello world!</h1>")?;
    assert_eq!(formatted, "---\n---\n\n<h1>Hello world!</h1>");
    Ok(())
}
#[test]
fn test_empty_2_html_format_1_b92cbb73() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\n---\n\n<div>\n---\n</div>")?;
    assert_eq!(formatted, "---\n---\n\n<div>---</div>");
    Ok(())
}
#[test]
fn test_issue_9042_html_format_1_5f38371d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("---\nlayout: foo\n---\n\nTest <a\nhref=\"https://prettier.io\">abc</a>.")?;
    assert_eq!(
        formatted,
        "---\nlayout: foo\n---\n\nTest <a href=\"https://prettier.io\">abc</a>."
    );
    Ok(())
}
#[test]
fn test_issue_9042_no_empty_line_html_format_1_a46b9f91() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("---\nlayout: foo\n---\nTest <a\nhref=\"https://prettier.io\">abc</a>.")?;
    assert_eq!(
        formatted,
        "---\nlayout: foo\n---\n\nTest <a href=\"https://prettier.io\">abc</a>."
    );
    Ok(())
}
