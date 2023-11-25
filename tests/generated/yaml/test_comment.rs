#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_collection_yml_format_1_ce52413e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("a: # a.trailingComment\n  123\n  # implicitMappingValue\n\n? b\n  # explicitMappingKey\n: c\n  # explicitMappingValue\n\nd:\n  - 123\n  # sequence\n\ne:\n  - 123\n    # sequenceItem\n\nf:\n  - a\n  # b.leadingComments\n  - b\n    # b.endComments\n  - c\n    # c.endComments\n  # sequence.endComments\n# documentBody.children\n\nempty_content:\n  # hello world") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "a: # a.trailingComment\n  123\n  # implicitMappingValue\n\n? b\n  # explicitMappingKey\n: c\n  # explicitMappingValue\n\nd:\n  - 123\n  # sequence\n\ne:\n  - 123\n    # sequenceItem\n\nf:\n  - a\n  # b.leadingComments\n  - b\n    # b.endComments\n  - c\n    # c.endComments\n  # sequence.endComments\n# documentBody.children\n\nempty_content:\n  # hello world");
}
#[test]
fn test_end_comment_yml_format_1_c30dc1d4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("parent:\n  one: 1\n  # two: 2\n\na:\n  b:\n   #b\n #a\n\nA:\n  B:\n #A\n   #A");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "parent:\n  one: 1\n  # two: 2\n\na:\n  b:\n    #b\n  #a\n\nA:\n  B:\n  #A\n  #A"
    );
}
#[test]
fn test_flow_sequence_mapping_yml_format_1_57293524() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("    a:\n      [\n        check-format, check-lint,\n        check-spelling,\n        # coverage,\n        # install-and-run-from-git,\n      ]\n\n    b:\n      {\n        a: check-format, b: check-lint,\n        c: check-spelling,\n        # d: coverage,\n        # e: install-and-run-from-git,\n      }\n\n    d:\n      # prettier-ignore\n      [\n        check-format, check-lint,\n        check-spelling,\n        # coverage,\n        # install-and-run-from-git,\n      ]\n\n    e:\n      # prettier-ignore\n      {\n        a: check-format, b: check-lint,\n        c: check-spelling,\n        # d: coverage,\n        # e: install-and-run-from-git,\n      }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "a: [\n    check-format,\n    check-lint,\n    check-spelling,\n    # coverage,\n    # install-and-run-from-git,\n  ]\n\nb: {\n    a: check-format,\n    b: check-lint,\n    c: check-spelling,\n    # d: coverage,\n    # e: install-and-run-from-git,\n  }\n\nd:\n  # prettier-ignore\n  [\n        check-format, check-lint,\n        check-spelling,\n        # coverage,\n        # install-and-run-from-git,\n      ]\n\ne:\n  # prettier-ignore\n  {\n        a: check-format, b: check-lint,\n        c: check-spelling,\n        # d: coverage,\n        # e: install-and-run-from-git,\n      }");
}
#[test]
fn test_in_empty_item_without_newlline_yml_format_1_729b95c1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a:\n  #12");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "a:\n  #123");
}
#[test]
fn test_issue_8378_yml_format_1_4f800cb3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("# --- comments ---");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "# --- comments ---");
}
#[test]
fn test_issue_9130_yml_format_1_ca9184ce() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("- foo: 0\n  bar: 1\n\n  # baz: 2\n- quux: 3\n\n- foo: 0\n  bar: 1\n\n  # baz: 2\n\n  # baz: 3\n- quux: 3") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "- foo: 0\n  bar: 1\n\n  # baz: 2\n- quux: 3\n\n- foo: 0\n  bar: 1\n\n  # baz: 2\n\n  # baz: 3\n- quux: 3");
}
#[test]
fn test_map_yml_format_1_c7da1b84() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("foo1:\n  - foo: item1\n    bar: item1\n\n  # - foo: item2\n  #   bar: item2\n\n  # - foo: item3\n  #   bar: item3\n\n  - foo: item4\n    bar: item4\n\nfoo2:\n  - foo: item11\n    bar: item11\n\n  # - foo: item22\n  #   bar: item22\n\n  # - foo: item33\n  #   bar: item33") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "foo1:\n  - foo: item1\n    bar: item1\n\n  # - foo: item2\n  #   bar: item2\n\n  # - foo: item3\n  #   bar: item3\n\n  - foo: item4\n    bar: item4\n\nfoo2:\n  - foo: item11\n    bar: item11\n\n  # - foo: item22\n  #   bar: item22\n\n  # - foo: item33\n  #   bar: item33");
}
#[test]
fn test_map_2_yml_format_1_4cd67815() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("foo1:\n  - foo\n\n  # - foo\n\n  # - foo\n\n  - foo\n\nfoo2:\n  - foo2\n\n  # - foo2\n\n\n\n\n  \n\n  # - foo2\n  # - foo2") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "foo1:\n  - foo\n\n  # - foo\n\n  # - foo\n\n  - foo\n\nfoo2:\n  - foo2\n\n  # - foo2\n\n  # - foo2\n  # - foo2");
}
#[test]
fn test_map_3_yml_format_1_3a6ab800() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("foo1:\n  - foo\n\n  # - foo\n\n  # - foo\n\n  - foo\n\nfoo2:\n  - foo2\n\n  # first line\n  # next line") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "foo1:\n  - foo\n\n  # - foo\n\n  # - foo\n\n  - foo\n\nfoo2:\n  - foo2\n\n  # first line\n  # next line");
}
#[test]
fn test_map_4_yml_format_1_f3679c56() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("before:\n\n  # before.comment\nafter:\n  # after.comment\n\nbefore-after:\n\n  # before-after.comment\n\nnone:\n  # none.comment\nbefore(2 line):\n\n\n  # before.comment\nafter(2 line):\n  # after.comment\n\n\nbefore-after(2 line):\n\n\n  # before-after.comment\n\n\nnone(2):\n  # none.commen") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "before:\n\n  # before.comment\nafter:\n  # after.comment\n\nbefore-after:\n\n  # before-after.comment\n\nnone:\n  # none.comment\nbefore(2 line):\n\n  # before.comment\nafter(2 line):\n  # after.comment\n\nbefore-after(2 line):\n\n  # before-after.comment\n\nnone(2):\n  # none.comment");
}
#[test]
fn test_object_yml_format_1_aa6c252b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "#6445\n\nobj:\n  # before\n\n\n  # before\n\n\n  key: value\n\n\n  # after\n\n\n  # after",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "#6445\n\nobj:\n  # before\n\n  # before\n\n  key: value\n\n  # after\n\n  # after"
    );
}
#[test]
fn test_root_yml_format_1_516fdcb9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("#hello world");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "#hello world");
}
#[test]
fn test_sequence_yml_format_1_ab6ffb45() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("-  - a\n\n   # - b\n\n   # - c\n\n   - e\n\n-  - a\n\n   # - b\n\n   # - c");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "- - a\n\n  # - b\n\n  # - c\n\n  - e\n\n- - a\n\n  # - b\n\n  # - c"
    );
}
#[test]
fn test_sequence_2_yml_format_1_6bb4e281() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("- before\n\n  # before.comment\n- after\n  # after.comment\n\n- before-after\n\n  # before-after.comment\n\n- none:\n  # none.comment\n- before(2 line)\n\n\n  # before.comment\n- after(2 line)\n  # after.comment\n\n\n- before-after(2 line)\n\n\n  # before-after.comment\n\n\n- none(2)\n  # none.commen") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "- before\n\n  # before.comment\n- after\n  # after.comment\n\n- before-after\n\n  # before-after.comment\n\n- none:\n  # none.comment\n- before(2 line)\n\n  # before.comment\n- after(2 line)\n  # after.comment\n\n- before-after(2 line)\n\n  # before-after.comment\n\n- none(2)\n  # none.comment");
}
#[test]
fn test_set_yml_format_1_f13e16ab() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- 123\n  # 456");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- 123\n  # 456");
}
