use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_3_style_yml_tab_width_4_format_1_2801d17f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa]: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n\n[aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa]: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n\n[aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa]: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa]: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n\n[aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa]:\n    aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n\n? [\n      aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,\n  ]\n: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
}
#[test]
fn test_3_style_yml_format_1_2801d17f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa]: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n\n[aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa]: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n\n[aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa]: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa]: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n\n[aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa]:\n  aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n\n? [\n    aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,\n  ]\n: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
}
#[test]
fn test_anchor_yml_tab_width_4_format_1_6912f0dc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("key1: &default\n\n  subkey1: value1\n\nkey2:\n  <<: *default");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "key1: &default\n    subkey1: value1\n\nkey2:\n    <<: *default"
    );
}
#[test]
fn test_anchor_yml_format_1_6912f0dc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("key1: &default\n\n  subkey1: value1\n\nkey2:\n  <<: *default");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "key1: &default\n  subkey1: value1\n\nkey2:\n  <<: *default"
    );
}
#[test]
fn test_anchor_2_yml_tab_width_4_format_1_38047a46() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("key1: &default\n\n  # This key ...\n  subkey1: value1\n\nkey2:\n  <<: *default");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "key1: &default # This key ...\n    subkey1: value1\n\nkey2:\n    <<: *default"
    );
}
#[test]
fn test_anchor_2_yml_format_1_38047a46() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("key1: &default\n\n  # This key ...\n  subkey1: value1\n\nkey2:\n  <<: *default");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "key1: &default # This key ...\n  subkey1: value1\n\nkey2:\n  <<: *default"
    );
}
#[test]
fn test_anchor_3_yml_tab_width_4_format_1_c60615ea() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("key1: &default\n# This key\n  subkey1: value1\n\nkey2:\n  <<: *default");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "key1: &default # This key\n    subkey1: value1\n\nkey2:\n    <<: *default"
    );
}
#[test]
fn test_anchor_3_yml_format_1_c60615ea() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("key1: &default\n# This key\n  subkey1: value1\n\nkey2:\n  <<: *default");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "key1: &default # This key\n  subkey1: value1\n\nkey2:\n  <<: *default"
    );
}
#[test]
fn test_array_key_yml_tab_width_4_format_1_d739f998() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[1, 2, 3]: 123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[1, 2, 3]: 123");
}
#[test]
fn test_array_key_yml_format_1_d739f998() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[1, 2, 3]: 123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[1, 2, 3]: 123");
}
#[test]
fn test_array_value_yml_tab_width_4_format_1_e74cecbe() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("123: [1, 2, 3]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "123: [1, 2, 3]");
}
#[test]
fn test_array_value_yml_format_1_e74cecbe() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("123: [1, 2, 3]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "123: [1, 2, 3]");
}
#[test]
fn test_comment_yml_tab_width_4_format_1_42f9c3b7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("? key\n# comment\n: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "? key\n# comment\n: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong");
}
#[test]
fn test_comment_yml_format_1_42f9c3b7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("? key\n# comment\n: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "? key\n# comment\n: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong");
}
#[test]
fn test_comment_value_yml_tab_width_4_format_1_442a5eb3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("123: # hello");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "123: # hello");
}
#[test]
fn test_comment_value_yml_format_1_442a5eb3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("123: # hello");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "123: # hello");
}
#[test]
fn test_common_yml_tab_width_4_format_1_2af5ace0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("key1: value\nkey2: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "key1: value\nkey2: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong");
}
#[test]
fn test_common_yml_format_1_2af5ace0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("key1: value\nkey2: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "key1: value\nkey2: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong");
}
#[test]
fn test_explicit_key_yml_tab_width_4_format_1_529c9a9d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("? key1\n: value\n? key2\n: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\n? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\n: value\n? solongitshouldbreakbutitcannot_longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\n: # Comment\n  foo: bar\n? multiline\n  scalar\n  key\n: value") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "key1: value\nkey2: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\nlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: value\nsolongitshouldbreakbutitcannot_longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong:\n    # Comment\n    foo: bar\n? multiline\n  scalar\n  key\n: value");
}
#[test]
fn test_explicit_key_yml_format_1_529c9a9d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("? key1\n: value\n? key2\n: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\n? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\n: value\n? solongitshouldbreakbutitcannot_longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\n: # Comment\n  foo: bar\n? multiline\n  scalar\n  key\n: value") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "key1: value\nkey2: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\nlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: value\nsolongitshouldbreakbutitcannot_longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong:\n  # Comment\n  foo: bar\n? multiline\n  scalar\n  key\n: value");
}
#[test]
fn test_in_sequence_yml_tab_width_4_format_1_04b13794() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- a: b\n  c: d");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- a: b\n  c: d");
}
#[test]
fn test_in_sequence_yml_format_1_04b13794() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- a: b\n  c: d");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- a: b\n  c: d");
}
#[test]
fn test_key_with_leading_comment_yml_tab_width_4_format_1_c0e337e3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("? # comment\n  key\n: value");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "? # comment\n  key\n: value");
}
#[test]
fn test_key_with_leading_comment_yml_format_1_c0e337e3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("? # comment\n  key\n: value");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "? # comment\n  key\n: value");
}
#[test]
fn test_mapping_yml_tab_width_4_format_1_beaf606f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("key:\n  key: value");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "key:\n    key: value");
}
#[test]
fn test_mapping_yml_format_1_beaf606f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("key:\n  key: value");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "key:\n  key: value");
}
#[test]
fn test_merge_twice_yml_tab_width_4_format_1_58f814cf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format (".anchors:\n  - &anchor1\n    key: value\n  - &anchor2\n    another: prop\n\nfoo:\n  bar: baz\n  <<: *anchor1\n  <<: *anchor2") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ".anchors:\n    - &anchor1\n      key: value\n    - &anchor2\n      another: prop\n\nfoo:\n    bar: baz\n    <<: *anchor1\n    <<: *anchor2");
}
#[test]
fn test_merge_twice_yml_format_1_58f814cf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format (".anchors:\n  - &anchor1\n    key: value\n  - &anchor2\n    another: prop\n\nfoo:\n  bar: baz\n  <<: *anchor1\n  <<: *anchor2") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ".anchors:\n  - &anchor1\n    key: value\n  - &anchor2\n    another: prop\n\nfoo:\n  bar: baz\n  <<: *anchor1\n  <<: *anchor2");
}
#[test]
fn test_middle_comment_yml_tab_width_4_format_1_071cfdf8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!map # comment\na: 123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!!map # comment\na: 123");
}
#[test]
fn test_middle_comment_yml_format_1_071cfdf8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!map # comment\na: 123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!!map # comment\na: 123");
}
#[test]
fn test_middle_comments_yml_tab_width_4_format_1_8649b2aa() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!map # comment 1\n# comment 2\na: 123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!!map\n# comment 1\n# comment 2\na: 123");
}
#[test]
fn test_middle_comments_yml_format_1_8649b2aa() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!map # comment 1\n# comment 2\na: 123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!!map\n# comment 1\n# comment 2\na: 123");
}
#[test]
fn test_props_yml_tab_width_4_format_1_2b98eea7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("--- !!map &anchor\na: 123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "---\n!!map &anchor\na: 123");
}
#[test]
fn test_props_yml_format_1_2b98eea7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("--- !!map &anchor\na: 123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "---\n!!map &anchor\na: 123");
}
#[test]
fn test_props_in_map_yml_tab_width_4_format_1_6cccda99() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: !!map &anchor\n  a: 123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "a: !!map &anchor\n    a: 123");
}
#[test]
fn test_props_in_map_yml_format_1_6cccda99() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: !!map &anchor\n  a: 123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "a: !!map &anchor\n  a: 123");
}
#[test]
fn test_quote_key_yml_tab_width_4_format_1_87a70a43() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\"a\": 123\n'b': 123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\"a\": 123\n\"b\": 123");
}
#[test]
fn test_quote_key_yml_format_1_87a70a43() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\"a\": 123\n'b': 123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\"a\": 123\n\"b\": 123");
}
#[test]
fn test_sequence_yml_tab_width_4_format_1_7f9c795d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("key:\n- value");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "key:\n    - value");
}
#[test]
fn test_sequence_yml_format_1_7f9c795d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("key:\n- value");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "key:\n  - value");
}
#[test]
fn test_tag_key_yml_tab_width_4_format_1_8d746a30() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("? !!tag key\n: value");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!!tag key: value");
}
#[test]
fn test_tag_key_yml_format_1_8d746a30() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("? !!tag key\n: value");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!!tag key: value");
}
