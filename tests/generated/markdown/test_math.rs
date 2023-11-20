#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_dollar_sign_md_format_1_ea419ff3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("$\n\n\\\\$\n\n\\\\\\\\$\n\n\\\\\\\\\\\\$");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "$\n\n\\\\$\n\n\\\\\\\\$\n\n\\\\\\\\\\\\$");
}
#[test]
fn test_empty_block_md_format_1_6e71d909() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("$$\n$$");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "$$\n$$");
}
#[test]
fn test_math_like_md_format_1_b5cf4150() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "$10 - $20\n\nParagraph with $14 million.    But if more $dollars on the same line...",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "$10 - $20\n\nParagraph with $14 million. But if more $dollars on the same line..."
    );
}
#[test]
fn test_remark_math_md_format_1_a430d167() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!-- tests from https://github.com/Rokt33r/remark-math/blob/9e13e49/specs/remark-math.spec.js -->\n\n$$\n\\\\beta+\\\\gamma\n$$\n\n---\n\n$\\\\alpha\\\\$\n\n---\n\n\\\\$\\\\alpha\\\\$\n\n---\n\n\\\\\\\\$\\\\alpha$\n\n---\n\n\\`$\\`\\\\alpha$\n\n---\n\n$\\\\alpha\\`$\\` foo\n\n---\n\n$\\`\\\\alpha\\`$\n\n---\n\n$\\\\alpha\\\\$$\n\n---\n\n$$\n\\\\alpha\\\\$\n$$\n\n---\n\ntango\n$$\n\\\\alpha\n$$\n\n---\n\n$$\\\\\\\\alpha$$\n\n---\n\n$$\\\\alpha$$\n$$\n\\\\alpha\\\\beta\n$$\n\n---\n\n> $$\n> \\\\alpha\\\\beta\n> $$\n\n---\n\n  $$$\n    \\\\alpha\n  $$$\n\n---\n\n$$  must\n\\\\alpha\n$$\n\n---\n\n$$\n\\\\alpha\n$$\n\\`\\`\\`\ncode fence\n\\`\\`\\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!-- tests from https://github.com/Rokt33r/remark-math/blob/9e13e49/specs/remark-math.spec.js -->\n\n$$\n\\\\beta+\\\\gamma\n$$\n\n---\n\n$\\\\alpha\\\\$\n\n---\n\n\\\\$\\\\alpha\\\\$\n\n---\n\n\\\\\\\\$\\\\alpha$\n\n---\n\n\\`$\\`\\\\alpha$\n\n---\n\n$\\\\alpha\\`$\\` foo\n\n---\n\n$\\`\\\\alpha\\`$\n\n---\n\n$\\\\alpha\\\\$$\n\n---\n\n$$\n\\\\alpha\\\\$\n$$\n\n---\n\ntango\n\n$$\n\\\\alpha\n$$\n\n---\n\n$$\\\\\\\\alpha$$\n\n---\n\n$$\\\\alpha$$\n\n$$\n\\\\alpha\\\\beta\n$$\n\n---\n\n> $$\n> \\\\alpha\\\\beta\n> $$\n\n---\n\n$$\n  \\\\alpha\n$$\n\n---\n\n$$\nmust\n\\\\alpha\n$$\n\n---\n\n$$\n\\\\alpha\n$$\n\n\\`\\`\\`\ncode fence\n\\`\\`\\`");
}
