use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_directives_and_comments_yml_format_1_8a6dde22() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("# 123\n%YAML 1.2\n# 456\n---\n# 789\ntest\n# 000");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "# 123\n%YAML 1.2\n# 456\n---\n# 789\ntest\n# 000"
    );
}
#[test]
fn test_separator_yml_format_1_8a0f565b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("a\n---\nb\n...\nc\n... #\nd\n...\n---\ne\n...\n#\n---\nf\n--- #\ng");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "a\n---\nb\n---\nc\n... #\nd\n---\ne\n...\n#\n---\nf\n--- #\ng"
    );
}
#[test]
fn test_with_document_head_yml_format_1_cd892de7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\n123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "---\n123");
}
#[test]
fn test_with_document_head_like_yml_format_1_b940c45a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---666\n123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "---666\n123");
}
#[test]
fn test_without_document_head_yml_format_1_b481bb58() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "123");
}
