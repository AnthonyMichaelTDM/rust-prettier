use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_force_singleline_in_mapping_value_yml_prose_wrapalways_format_1_0fc63254() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("no-whitesapce: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\n\nwhitespace: longlonglonglonglonglonglonglonglonglonglong longlonglonglonglonglonglonglonglonglonglong\n\nliteral-newline: longlonglonglonglonglonglonglonglonglonglong\n  longlonglonglonglonglonglonglonglonglonglong\n\nnewline: longlonglonglonglonglonglonglonglonglonglong\n\n  longlonglonglonglonglonglonglonglonglonglong") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "no-whitesapce: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\n\nwhitespace:\n  longlonglonglonglonglonglonglonglonglonglong\n  longlonglonglonglonglonglonglonglonglonglong\n\nliteral-newline:\n  longlonglonglonglonglonglonglonglonglonglong\n  longlonglonglonglonglonglonglonglonglonglong\n\nnewline: longlonglonglonglonglonglonglonglonglonglong\n\n  longlonglonglonglonglonglonglonglonglonglong");
}
#[test]
fn test_force_singleline_in_mapping_value_yml_prose_wrapnever_format_1_0fc63254() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("no-whitesapce: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\n\nwhitespace: longlonglonglonglonglonglonglonglonglonglong longlonglonglonglonglonglonglonglonglonglong\n\nliteral-newline: longlonglonglonglonglonglonglonglonglonglong\n  longlonglonglonglonglonglonglonglonglonglong\n\nnewline: longlonglonglonglonglonglonglonglonglonglong\n\n  longlonglonglonglonglonglonglonglonglonglong") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "no-whitesapce: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\n\nwhitespace: longlonglonglonglonglonglonglonglonglonglong longlonglonglonglonglonglonglonglonglonglong\n\nliteral-newline: longlonglonglonglonglonglonglonglonglonglong longlonglonglonglonglonglonglonglonglonglong\n\nnewline: longlonglonglonglonglonglonglonglonglonglong\n\n  longlonglonglonglonglonglonglonglonglonglong");
}
#[test]
fn test_force_singleline_in_mapping_value_yml_format_1_0fc63254() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("no-whitesapce: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\n\nwhitespace: longlonglonglonglonglonglonglonglonglonglong longlonglonglonglonglonglonglonglonglonglong\n\nliteral-newline: longlonglonglonglonglonglonglonglonglonglong\n  longlonglonglonglonglonglonglonglonglonglong\n\nnewline: longlonglonglonglonglonglonglonglonglonglong\n\n  longlonglonglonglonglonglonglonglonglonglong") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "no-whitesapce: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\n\nwhitespace: longlonglonglonglonglonglonglonglonglonglong longlonglonglonglonglonglonglonglonglonglong\n\nliteral-newline: longlonglonglonglonglonglonglonglonglonglong\n  longlonglonglonglonglonglonglonglonglonglong\n\nnewline: longlonglonglonglonglonglonglonglonglonglong\n\n  longlonglonglonglonglonglonglonglonglonglong");
}
#[test]
fn test_middle_comment_yml_prose_wrapalways_format_1_657a1acc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!str # comment\nhello");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!!str # comment\nhello");
}
#[test]
fn test_middle_comment_yml_prose_wrapnever_format_1_657a1acc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!str # comment\nhello");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!!str # comment\nhello");
}
#[test]
fn test_middle_comment_yml_format_1_657a1acc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!str # comment\nhello");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!!str # comment\nhello");
}
#[test]
fn test_middle_comments_yml_prose_wrapalways_format_1_94999c99() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!str # comment 1\n# comment 2\nhello");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!!str\n# comment 1\n# comment 2\nhello");
}
#[test]
fn test_middle_comments_yml_prose_wrapnever_format_1_94999c99() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!str # comment 1\n# comment 2\nhello");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!!str\n# comment 1\n# comment 2\nhello");
}
#[test]
fn test_middle_comments_yml_format_1_94999c99() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!str # comment 1\n# comment 2\nhello");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!!str\n# comment 1\n# comment 2\nhello");
}
#[test]
fn test_multiline_yml_prose_wrapalways_format_1_dded7bd1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("b:    \n  123123123123123123123123123   \n       123123123123123123123123123   \n         123123123123123123123123123\t\t\t\t\n    123123123123123123123123123   \n      123123123123123123123123123   \n  123123123123123123123123123   \n            123123123123123123123123123\t\t\t\t\n         \n         123123123123123123123123123   \n         \n         \n         123123123123123123123123123      \n         \n         \n         \n         \n         123123123123123123123123123  \n    ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "b: 123123123123123123123123123 123123123123123123123123123\n  123123123123123123123123123 123123123123123123123123123\n  123123123123123123123123123 123123123123123123123123123\n  123123123123123123123123123\n\n  123123123123123123123123123\n\n\n  123123123123123123123123123\n\n\n\n\n  123123123123123123123123123");
}
#[test]
fn test_multiline_yml_prose_wrapnever_format_1_dded7bd1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("b:    \n  123123123123123123123123123   \n       123123123123123123123123123   \n         123123123123123123123123123\t\t\t\t\n    123123123123123123123123123   \n      123123123123123123123123123   \n  123123123123123123123123123   \n            123123123123123123123123123\t\t\t\t\n         \n         123123123123123123123123123   \n         \n         \n         123123123123123123123123123      \n         \n         \n         \n         \n         123123123123123123123123123  \n    ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "b:\n  123123123123123123123123123 123123123123123123123123123 123123123123123123123123123 123123123123123123123123123 123123123123123123123123123 123123123123123123123123123 123123123123123123123123123\n\n  123123123123123123123123123\n\n\n  123123123123123123123123123\n\n\n\n\n  123123123123123123123123123");
}
#[test]
fn test_multiline_yml_format_1_dded7bd1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("b:    \n  123123123123123123123123123   \n       123123123123123123123123123   \n         123123123123123123123123123\t\t\t\t\n    123123123123123123123123123   \n      123123123123123123123123123   \n  123123123123123123123123123   \n            123123123123123123123123123\t\t\t\t\n         \n         123123123123123123123123123   \n         \n         \n         123123123123123123123123123      \n         \n         \n         \n         \n         123123123123123123123123123  \n    ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "b: 123123123123123123123123123\n  123123123123123123123123123\n  123123123123123123123123123\n  123123123123123123123123123\n  123123123123123123123123123\n  123123123123123123123123123\n  123123123123123123123123123\n\n  123123123123123123123123123\n\n\n  123123123123123123123123123\n\n\n\n\n  123123123123123123123123123");
}
#[test]
fn test_true_yml_prose_wrapalways_format_1_14e273e6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("hello");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "hello");
}
#[test]
fn test_true_yml_prose_wrapnever_format_1_14e273e6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("hello");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "hello");
}
#[test]
fn test_true_yml_format_1_14e273e6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("hello");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "hello");
}
#[test]
fn test_verbatim_yml_prose_wrapalways_format_1_5d31d918() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!<hello> hello");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!<hello> hello");
}
#[test]
fn test_verbatim_yml_prose_wrapnever_format_1_5d31d918() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!<hello> hello");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!<hello> hello");
}
#[test]
fn test_verbatim_yml_format_1_5d31d918() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!<hello> hello");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!<hello> hello");
}
