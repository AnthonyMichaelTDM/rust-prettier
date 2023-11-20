#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_custom_parser_html_format_1_a651ce11() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "---mycustomparser\n  \ntitle: Hello\nslug: home\n\n---\n\n<h1>\n  Hello world!</h1>",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "---mycustomparser\n  \ntitle: Hello\nslug: home\n\n---\n\n<h1>Hello world!</h1>"
    );
}
#[test]
fn test_empty_html_format_1_97355988() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\n---\n\n<h1>\n  Hello world!</h1>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "---\n---\n\n<h1>Hello world!</h1>");
}
#[test]
fn test_empty_2_html_format_1_b92cbb73() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\n---\n\n<div>\n---\n</div>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "---\n---\n\n<div>---</div>");
}
#[test]
fn test_issue_9042_html_format_1_5f38371d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("---\nlayout: foo\n---\n\nTest <a\nhref=\"https://prettier.io\">abc</a>.");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "---\nlayout: foo\n---\n\nTest <a href=\"https://prettier.io\">abc</a>."
    );
}
#[test]
fn test_issue_9042_no_empty_line_html_format_1_a46b9f91() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("---\nlayout: foo\n---\nTest <a\nhref=\"https://prettier.io\">abc</a>.");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "---\nlayout: foo\n---\n\nTest <a href=\"https://prettier.io\">abc</a>."
    );
}
