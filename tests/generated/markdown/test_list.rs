#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_align_md_prose_wrapalways_format_1_98cfabf0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("1. 123\n\n---\n\n11. 123\n\n---\n\n111. 123\n\n---\n\n1111. 123\n\n---\n\n11111. 123\n\n---\n\n1.  123\n\n---\n\n1.  123\n2.  123\n\n---\n\n11. 123\n1. 123\n\n---\n\n11. 123\n1.  123\n\n---\n\n1. 123\n2. 123\n   1.   123\n   2.   123\n\n---\n\n1.  123\n2.  123\n    1.   123\n    2.   123\n\n---\n\n- 123\n- 123\n  1.  123\n  2.  123") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "1. 123\n\n---\n\n11. 123\n\n---\n\n111. 123\n\n---\n\n1111. 123\n\n---\n\n11111. 123\n\n---\n\n1.  123\n\n---\n\n1.  123\n2.  123\n\n---\n\n11. 123\n1. 123\n\n---\n\n11. 123\n1.  123\n\n---\n\n1. 123\n2. 123\n   1. 123\n   2. 123\n\n---\n\n1.  123\n2.  123\n    1.  123\n    2.  123\n\n---\n\n- 123\n- 123\n  1.  123\n  2.  123");
}
#[test]
fn test_checkbox_md_prose_wrapalways_format_1_079aba07() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("- [ ] this is a long long long long long long long long long long long long long long paragraph.\n- [x] this is a long long long long long long long long long long long long long long paragraph.") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "- [ ] this is a long long long long long long long long long long long long long\n      long paragraph.\n- [x] this is a long long long long long long long long long long long long long\n      long paragraph.");
}
#[test]
fn test_codeblock_md_prose_wrapalways_format_1_5b5b2a51() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("1. ol01\n\n    \\`\\`\\`js\n    const a = 1;\n\n\n    const b = 2;\n    \\`\\`\\`\n\n2. ol02\n\n    \\`\\`\\`js\n    const a = 1;\n\n\n    const b = 2;\n    \\`\\`\\`\n\n- ul01\n\n    \\`\\`\\`js\n    const a = 1;\n\n\n    const b = 2;\n    \\`\\`\\`\n\n- ul02\n\n    \\`\\`\\`js\n    const a = 1;\n\n\n    const b = 2;\n    \\`\\`\\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "1. ol01\n\n   \\`\\`\\`js\n   const a = 1;\n\n   const b = 2;\n   \\`\\`\\`\n\n2. ol02\n\n   \\`\\`\\`js\n   const a = 1;\n\n   const b = 2;\n   \\`\\`\\`\n\n- ul01\n\n  \\`\\`\\`js\n  const a = 1;\n\n  const b = 2;\n  \\`\\`\\`\n\n- ul02\n\n  \\`\\`\\`js\n  const a = 1;\n\n  const b = 2;\n  \\`\\`\\`");
}
#[test]
fn test_combined_lists_md_prose_wrapalways_format_1_7f30932c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("* hello1\n\n* hello2\n\n\n* hello3\n\n* hello4\n\n\n* hello5\n\n* hello");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "- hello1\n\n- hello2\n\n- hello3\n\n- hello4\n\n- hello5\n\n- hello6"
    );
}
#[test]
fn test_git_diff_friendly_md_prose_wrapalways_format_1_f52aed87() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("5. abc\n1. def\n999. ghi");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "5. abc\n1. def\n1. ghi");
}
#[test]
fn test_indent_md_prose_wrapalways_format_1_2d88480e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("- [ ] a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n\n    - a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n\n      b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n\n    - [ ] a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n\n      b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n\n    1. a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n\n       b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n\n    1. [ ] a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n\n       b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n\n    12345678) a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n\n              b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n\n    12345678. [ ] a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n\n              b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n\n    a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n\n1. [ ] a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n\n    - a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n\n      b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n\n    - [ ] a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n\n      b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n\n    1. a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n\n       b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n\n    1. [ ] a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n\n       b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n\n    12345678) a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n\n              b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n\n    12345678. [ ] a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n\n              b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n\n    a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n\n12345678) [ ] a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n\n          b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n\n          - a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n\n            b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n\n          - [ ] a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n\n            b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n\n          1. a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n\n             b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n\n          1. [ ] a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n\n             b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n\n          12345678) a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n\n                    b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n\n          12345678. [ ] a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n\n                    b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n\n          a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "- [ ] a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n      a a a a a a a a a a a a a a a\n\n  - a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n    a a a a a a a a a a a a a a\n\n    b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n    b b b b b b b b b b b b b b\n\n  - [ ] a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n        a a a a a a a a a a a a a a\n\n    b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n    b b b b b b b b b b b b b b\n\n  1. a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n     a a a a a a a a a a a a a a\n\n     b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n     b b b b b b b b b b b b b b\n\n  1. [ ] a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n         a a a a a a a a a a a a a a\n\n     b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n     b b b b b b b b b b b b b b\n\n  12345678) a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n            a a a a a a a a a a a a a a\n\n            b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n            b b b b b b b b b b b b b b\n\n  12345678. [ ] a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n                a a a a a a a a a a a a a a\n\n            b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n            b b b b b b b b b b b b b b\n\n  a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n  a a a a a a a a a a a a a a\n\n1. [ ] a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n       a a a a a a a a a a a a a a a\n\n   - a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n     a a a a a a a a a a a a a a\n\n     b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n     b b b b b b b b b b b b b b\n\n   - [ ] a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n         a a a a a a a a a a a a a a\n\n     b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n     b b b b b b b b b b b b b b\n\n   1. a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n      a a a a a a a a a a a a a a a\n\n      b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n      b b b b b b b b b b b b b b b\n\n   1. [ ] a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n          a a a a a a a a a a a a a a a\n\n      b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n      b b b b b b b b b b b b b b b\n\n   12345678) a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n             a a a a a a a a a a a a a a\n\n             b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n             b b b b b b b b b b b b b b\n\n   12345678. [ ] a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n                 a a a a a a a a a a a a a a\n\n             b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n             b b b b b b b b b b b b b b\n\n   a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n   a a a a a a a a a a a a a a\n\n12345678) [ ] a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n              a a a a a a a a a a a a a a a\n\n          b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n          b b b b b b b b b b b b b b b\n\n          - a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n            a a a a a a a a a a a a a a a\n\n            b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n            b b b b b b b b b b b b b b b\n\n          - [ ] a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n                a a a a a a a a a a a a a a a\n\n            b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n            b b b b b b b b b b b b b b b\n\n          1. a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n             a a a a a a a a a a a a a a a\n\n             b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n             b b b b b b b b b b b b b b b\n\n          1. [ ] a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n                 a a a a a a a a a a a a a a a\n\n             b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n             b b b b b b b b b b b b b b b\n\n          12345678) a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n                    a a a a a a a a a a a a a a a\n\n                    b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n                    b b b b b b b b b b b b b b b b\n\n          12345678. [ ] a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n                        a a a a a a a a a a a a a a a\n\n                    b b b b b b b b b b b b b b b b b b b b b b b b b b b b b b\n                    b b b b b b b b b b b b b b b b\n\n          a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a\n          a a a a a a a a a a a a a a a");
}
#[test]
fn test_interrupt_md_prose_wrapalways_format_1_16cf46a1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("* Something\n### Some heading");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- Something\n\n### Some heading");
}
#[test]
fn test_issue_7846_md_prose_wrapalways_format_1_26797665() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("- a a  \n   b b  \n    c c  \n   d d  \n  e e\n\n1.  a a a  \n   b b b  \n  c c c  \n   d d d  \n    e e ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "- a a  \n   b b  \n   c c  \n   d d  \n  e e\n\n1.  a a a  \n     b b b  \n    c c c  \n     d d d  \n     e e e");
}
#[test]
fn test_long_paragraph_md_prose_wrapalways_format_1_ca3870fd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("- This is a long long long long long long long long long long long long long long paragraph.") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "- This is a long long long long long long long long long long long long long\n  long paragraph.");
}
#[test]
fn test_loose_md_prose_wrapalways_format_1_3c793ea3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("- 123\n\n  - abc\n\n- 456\n\n  - def\n\n- 789\n\n  - ghi");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "- 123\n\n  - abc\n\n- 456\n\n  - def\n\n- 789\n\n  - ghi"
    );
}
#[test]
fn test_multiline_md_prose_wrapalways_format_1_d293ebc4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- 123\n  456\n  789");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- 123 456 789");
}
#[test]
fn test_nested_md_prose_wrapalways_format_1_70e1136a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- Level 1\n  - Level 2\n    - Level 3");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- Level 1\n  - Level 2\n    - Level 3");
}
#[test]
fn test_nested_checkbox_md_prose_wrapalways_format_1_c7fdb817() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("* parent list item parent list item parent list item parent list item parent list item parent list item\n\n     * child list item child list item child list item child list item child list item child list item\n\n    paragraph paragraph paragraph paragraph paragraph paragraph paragraph paragraph paragraph\n\n* [x] parent task list item parent task list item parent task list item parent task list item\n\n     * [x] child task list item child task list item child task list item child task list item\n\n    paragraph paragraph paragraph paragraph paragraph paragraph paragraph paragraph paragraph") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "- parent list item parent list item parent list item parent list item parent\n  list item parent list item\n\n  - child list item child list item child list item child list item child list\n    item child list item\n\n  paragraph paragraph paragraph paragraph paragraph paragraph paragraph\n  paragraph paragraph\n\n- [x] parent task list item parent task list item parent task list item parent\n      task list item\n\n  - [x] child task list item child task list item child task list item child\n        task list item\n\n  paragraph paragraph paragraph paragraph paragraph paragraph paragraph\n  paragraph paragraph");
}
#[test]
fn test_nested_tab_md_prose_wrapalways_format_1_7c348860() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("# List with 1 tab indentation and \\`-\\`\n- Top level list item 1\n- Top level list item 2\n\t- Nested List item 1\n\t- Nested List item 2\n\t\t- Sub-Nested List item 1\n\t\t- Sub-Nested List item 2\n\n# List with 1 tab indentation and \\`*\\`\n* Top level list item 1\n* Top level list item 2\n\t* Nested List item 1\n\t* Nested List item 2\n    \t* Sub-Nested List item 1\n    \t* Sub-Nested List item 2") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "# List with 1 tab indentation and \\`-\\`\n\n- Top level list item 1\n- Top level list item 2\n  - Nested List item 1\n  - Nested List item 2\n    - Sub-Nested List item 1\n    - Sub-Nested List item 2\n\n# List with 1 tab indentation and \\`*\\`\n\n- Top level list item 1\n- Top level list item 2\n  - Nested List item 1\n  - Nested List item 2 _ Sub-Nested List item 1 _ Sub-Nested List item 2");
}
#[test]
fn test_ordered_md_prose_wrapalways_format_1_3bb5073a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1. 123\n1. 456\n1. 789");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "1. 123\n1. 456\n1. 789");
}
#[test]
fn test_separate_md_prose_wrapalways_format_1_f1d8cef8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- 123\n- 123\n- 123\n\n* 123\n* 123\n* 123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- 123\n- 123\n- 123\n\n* 123\n* 123\n* 123");
}
#[test]
fn test_simple_md_prose_wrapalways_format_1_54f3cb90() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- 123\n- 456\n- 789");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- 123\n- 456\n- 789");
}
#[test]
fn test_start_md_prose_wrapalways_format_1_aa701344() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("5. abc\n6. def\n7. ghi\n\n---\n\n0. abc\n0. def\n0. ghi\n\n---\n\n1. abc\n1. def\n1. ghi\n\n---\n\n2. abc\n2. def\n2. ghi\n\n---\n\n0. abc\n1. def\n2. ghi\n\n---\n\n0. abc\n1. def\n1. ghi\n\n---\n\n1. abc\n2. def\n3. ghi\n\n---\n\n2. abc\n3. def\n4. ghi\n\n---\n\n999. abc\n145. def\n69. ghi\n\n---\n\n0. abc\n\n---\n\n1. abc\n\n---\n\n2. abc\n\n---\n\n999. abc\n\n---\n\n0. abc\n1. def\n\n---\n\n1. abc\n2. def\n\n---\n\n\n1. abc\n1. def\n\n---\n\n2. abc\n3. def\n\n---\n\n999. abc\n1. def\n\n---\n\n999. abc\n2. def") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "5. abc\n6. def\n7. ghi\n\n---\n\n0. abc\n1. def\n2. ghi\n\n---\n\n1. abc\n1. def\n1. ghi\n\n---\n\n2. abc\n3. def\n4. ghi\n\n---\n\n0. abc\n1. def\n2. ghi\n\n---\n\n0. abc\n1. def\n1. ghi\n\n---\n\n1. abc\n2. def\n3. ghi\n\n---\n\n2. abc\n3. def\n4. ghi\n\n---\n\n999. abc\n1000. def\n1001. ghi\n\n---\n\n0. abc\n\n---\n\n1. abc\n\n---\n\n2. abc\n\n---\n\n999. abc\n\n---\n\n0. abc\n1. def\n\n---\n\n1. abc\n2. def\n\n---\n\n1. abc\n1. def\n\n---\n\n2. abc\n3. def\n\n---\n\n999. abc\n1. def\n\n---\n\n999. abc\n1000. def");
}
#[test]
fn test_tab_md_prose_wrapalways_format_1_e5e1b1f1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("* Text\n\n \t[title](link)\n\n* Text\n\n \t- foo\n \t- foo\n \t- bar\n\n* Text\n\n \t# foo\n\n* Text\n\n \t\\`\\`\\`\n \tfoo\n \t\\`\\`\\`\n\n* Text\n\n \t\\`foo\\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "- Text\n\n  [title](link)\n\n- Text\n\n  - foo\n  - foo\n  - bar\n\n- Text\n\n  # foo\n\n- Text\n\n  \\`\\`\\`\n  foo\n  \\`\\`\\`\n\n- Text\n\n  \\`foo\\`");
}
