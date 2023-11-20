#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_indented_md_prose_wrapalways_format_1_a9ca9e80() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("- 123\n- 456\n- 789\n  <!-- prettier-ignore -->\n  - This is a long long\n    long long long long\n    long long paragraph.") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "- 123\n- 456\n- 789\n  <!-- prettier-ignore -->\n  - This is a long long\n    long long long long\n    long long paragraph.");
}
#[test]
fn test_simple_md_prose_wrapalways_format_1_9a85c1b2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!-- prettier-ignore -->\nThis is a long long long long long long long long long long long long long long long paragraph.") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!-- prettier-ignore -->\nThis is a long long long long long long long long long long long long long long long paragraph.");
}
#[test]
fn test_top_level_range_md_prose_wrapalways_format_1_854daa9e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!-- prettier-ignore-start -->\n<!-- some tool start (this should be ignored) -->\n\n| some | table |\n| - | - |\n| 1 | a |\n| 2 | b |\n\n<!-- some tool end -->\n<!-- prettier-ignore-end -->\n\n> <!-- prettier-ignore-start -->\n> <!-- some tool start (this shouldn't be ignored) -->\n>\n> | some | table |\n> | - | - |\n> | 1 | a |\n> | 2 | b |\n>\n> <!-- some tool end -->\n> <!-- prettier-ignore-end -->") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!-- prettier-ignore-start -->\n<!-- some tool start (this should be ignored) -->\n\n| some | table |\n| - | - |\n| 1 | a |\n| 2 | b |\n\n<!-- some tool end -->\n<!-- prettier-ignore-end -->\n\n> <!-- prettier-ignore-start -->\n> <!-- some tool start (this shouldn't be ignored) -->\n>\n> | some | table |\n> | ---- | ----- |\n> | 1    | a     |\n> | 2    | b     |\n>\n> <!-- some tool end -->\n> <!-- prettier-ignore-end -->");
}
