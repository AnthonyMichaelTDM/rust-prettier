use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_clip_yml_prose_wrapalways_format_1_0489f7b5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(">\n    123\n    456\n    789\n\n");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, ">\n  123 456 789");
}
#[test]
fn test_clip_yml_format_1_0489f7b5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(">\n    123\n    456\n    789\n\n");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, ">\n  123\n  456\n  789");
}
#[test]
fn test_indent_yml_prose_wrapalways_format_1_ea49748f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(">2-\n    123\n   456\n  789\n\n");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, ">2-\n    123\n   456\n  789");
}
#[test]
fn test_indent_yml_format_1_ea49748f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(">2-\n    123\n   456\n  789\n\n");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, ">2-\n    123\n   456\n  789");
}
#[test]
fn test_keep_yml_prose_wrapalways_format_1_a1376f9a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(">+\n    123\n    456\n    789\n\n");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, ">+\n  123 456 789\n\n");
}
#[test]
fn test_keep_yml_format_1_a1376f9a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(">+\n    123\n    456\n    789\n\n");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, ">+\n  123\n  456\n  789\n\n");
}
#[test]
fn test_map_yml_prose_wrapalways_format_1_07fa56cb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("a: >\n  123\n  456\n  789\nb: >1\n    123\n   456\n  789\nd: >\n  123\n  456\n  789\n\nc: 0") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "a: >\n  123 456 789\nb: >1\n    123\n   456\n  789\nd: >\n  123 456 789\n\nc: 0"
    );
}
#[test]
fn test_map_yml_format_1_07fa56cb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("a: >\n  123\n  456\n  789\nb: >1\n    123\n   456\n  789\nd: >\n  123\n  456\n  789\n\nc: 0") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "a: >\n  123\n  456\n  789\nb: >1\n    123\n   456\n  789\nd: >\n  123\n  456\n  789\n\nc: 0");
}
#[test]
fn test_middle_comment_yml_prose_wrapalways_format_1_ca54ffd1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!str #comment\n>\n  123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!!str #comment\n>\n  123");
}
#[test]
fn test_middle_comment_yml_format_1_ca54ffd1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!str #comment\n>\n  123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!!str #comment\n>\n  123");
}
#[test]
fn test_middle_comments_yml_prose_wrapalways_format_1_45fb8e6b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!str # comment 1\n# comment 2\n>\n  123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!!str\n# comment 1\n# comment 2\n>\n  123");
}
#[test]
fn test_middle_comments_yml_format_1_45fb8e6b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!str # comment 1\n# comment 2\n>\n  123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!!str\n# comment 1\n# comment 2\n>\n  123");
}
#[test]
fn test_newline_yml_prose_wrapalways_format_1_39fe717c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("- >+\n  123\n  456\n  789\n\n  123\n  456\n  789\n\n\n  123\n  456\n  789\n\n\n- >2+\n  123\n  456\n  789\n\n  123\n  456\n  789\n\n\n  123\n  456\n  789\n\n- 0") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "- >+\n  123 456 789\n\n  123 456 789\n\n\n  123 456 789\n\n\n- >2+\n  123 456 789\n\n  123 456 789\n\n\n  123 456 789\n\n- 0");
}
#[test]
fn test_newline_yml_format_1_39fe717c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("- >+\n  123\n  456\n  789\n\n  123\n  456\n  789\n\n\n  123\n  456\n  789\n\n\n- >2+\n  123\n  456\n  789\n\n  123\n  456\n  789\n\n\n  123\n  456\n  789\n\n- 0") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "- >+\n  123\n  456\n  789\n\n  123\n  456\n  789\n\n\n  123\n  456\n  789\n\n\n- >2+\n  123\n  456\n  789\n\n  123\n  456\n  789\n\n\n  123\n  456\n  789\n\n- 0");
}
#[test]
fn test_newline_unaligned_yml_prose_wrapalways_format_1_1c1c4b3b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format (">\n  1\n  2\n    3\n    4\n  5\n  6\n\n  1\n  2\n    3\n    4\n  5\n  6\n\n\n  1\n  2\n    3\n    4\n  5\n  6\n\n  1\n  2\n\n    3\n    4\n\n  5\n  6") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ">\n  1 2\n    3\n    4\n  5 6\n\n  1 2\n    3\n    4\n  5 6\n\n\n  1 2\n    3\n    4\n  5 6\n\n  1 2\n\n    3\n    4\n\n  5 6");
}
#[test]
fn test_newline_unaligned_yml_format_1_1c1c4b3b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format (">\n  1\n  2\n    3\n    4\n  5\n  6\n\n  1\n  2\n    3\n    4\n  5\n  6\n\n\n  1\n  2\n    3\n    4\n  5\n  6\n\n  1\n  2\n\n    3\n    4\n\n  5\n  6") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ">\n  1\n  2\n    3\n    4\n  5\n  6\n\n  1\n  2\n    3\n    4\n  5\n  6\n\n\n  1\n  2\n    3\n    4\n  5\n  6\n\n  1\n  2\n\n    3\n    4\n\n  5\n  6");
}
#[test]
fn test_props_yml_prose_wrapalways_format_1_67327b3f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!str &anchor >\n  123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!!str &anchor >\n  123");
}
#[test]
fn test_props_yml_format_1_67327b3f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!str &anchor >\n  123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!!str &anchor >\n  123");
}
#[test]
fn test_props_in_map_yml_prose_wrapalways_format_1_b8e9f473() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: !!str &anchor >\n  123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "a: !!str &anchor >\n  123");
}
#[test]
fn test_props_in_map_yml_format_1_b8e9f473() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: !!str &anchor >\n  123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "a: !!str &anchor >\n  123");
}
#[test]
fn test_prose_yml_prose_wrapalways_format_1_e3108f0f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format (">\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n---\n> \n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ">\n  123 456 789 123 456 789 123 456 789 123 456 789 123 456 789 123 456 789 123\n  456 789 123 456 789 123 456 789 123 456 789 123 456 789 123 456 789 123 456\n  789 123 456 789 123 456 789 123 456 789 123 456 789 123 456 789 123 456 789\n---\n>\n  123   456   789 123   456   789 123   456   789 123   456   789\n  123   456   789 123   456   789 123   456   789 123   456   789\n  123   456   789 123   456   789 123   456   789 123   456   789\n  123   456   789 123   456   789 123   456   789 123   456   789\n  123   456   789 123   456   789 123   456   789");
}
#[test]
fn test_prose_yml_format_1_e3108f0f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format (">\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n---\n> \n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ">\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n  123 456 789\n---\n>\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789\n  123   456   789");
}
#[test]
fn test_seq_yml_prose_wrapalways_format_1_74088378() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("- >\n  123\n  456\n  789\n- >1\n    123\n   456\n  789\n- 0");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "- >\n  123 456 789\n- >1\n    123\n   456\n  789\n- 0"
    );
}
#[test]
fn test_seq_yml_format_1_74088378() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("- >\n  123\n  456\n  789\n- >1\n    123\n   456\n  789\n- 0");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "- >\n  123\n  456\n  789\n- >1\n    123\n   456\n  789\n- 0"
    );
}
#[test]
fn test_strip_yml_prose_wrapalways_format_1_7c204036() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(">-\n    123\n    456\n    789\n\n");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, ">-\n  123 456 789");
}
#[test]
fn test_strip_yml_format_1_7c204036() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(">-\n    123\n    456\n    789\n\n");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, ">-\n  123\n  456\n  789");
}
#[test]
fn test_trailing_comment_yml_prose_wrapalways_format_1_49b1bb30() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: !!str > # hello\n  hello");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "a: !!str > # hello\n  hello");
}
#[test]
fn test_trailing_comment_yml_format_1_49b1bb30() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: !!str > # hello\n  hello");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "a: !!str > # hello\n  hello");
}
