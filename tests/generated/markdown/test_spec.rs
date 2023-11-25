#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_commonmark_0_30_example_328_md_prose_wrapalways_format_1_63a4e7d9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("30-example-328.md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`foo\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`foo\\`");
}
#[test]
fn test_commonmark_0_30_example_329_md_prose_wrapalways_format_1_910f3423() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("30-example-329.md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`\\` foo \\` bar \\`\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`\\`foo \\` bar\\`\\`");
}
#[test]
fn test_commonmark_0_30_example_330_md_prose_wrapalways_format_1_2b165875() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("30-example-330.md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\` \\`\\` \\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\` \\`\\` \\`");
}
#[test]
fn test_commonmark_0_30_example_331_md_prose_wrapalways_format_1_5a8dd071() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("30-example-331.md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`  \\`\\`  \\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`  \\`\\`  \\`");
}
#[test]
fn test_commonmark_0_30_example_332_md_prose_wrapalways_format_1_261fbe1e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("30-example-332.md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\` a\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\` a\\`");
}
#[test]
fn test_commonmark_0_30_example_333_md_prose_wrapalways_format_1_5228bd13() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("30-example-333.md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`\u{a0}b\u{a0}\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`\u{a0}b\u{a0}\\`");
}
#[test]
fn test_commonmark_0_30_example_334_md_prose_wrapalways_format_1_82ac97f1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("30-example-334.md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\` \\`\n\\`  \\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\` \\` \\`  \\`");
}
#[test]
fn test_commonmark_0_30_example_335_md_prose_wrapalways_format_1_a68acf46() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("30-example-335.md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`\\`\nfoo\nbar  \nbaz\n\\`\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`foo bar   baz\\`");
}
#[test]
fn test_commonmark_0_30_example_336_md_prose_wrapalways_format_1_f8f1221d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("30-example-336.md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`\\`\nfoo \n\\`\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`foo \\`");
}
#[test]
fn test_commonmark_0_30_example_337_md_prose_wrapalways_format_1_7cbabf44() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("30-example-337.md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`foo   bar \nbaz\\");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`foo   bar  baz\\`");
}
#[test]
fn test_example_1_md_prose_wrapalways_format_1_849b62fe() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\tfoo\tbaz\t\tbim");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "    foo\tbaz\t\tbim");
}
#[test]
fn test_example_2_md_prose_wrapalways_format_1_aa560c30() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("  \tfoo\tbaz\t\tbim");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo baz bim");
}
#[test]
fn test_example_3_md_prose_wrapalways_format_1_1e5d2028() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("    a\ta\n    ὐ\ta");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "    a\ta\n    ὐ\ta");
}
#[test]
fn test_example_4_md_prose_wrapalways_format_1_25c7dc10() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("  - foo\n\n\tbar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- foo\n\n  bar");
}
#[test]
fn test_example_5_md_prose_wrapalways_format_1_88a4e15a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- foo\n\n\t\tbar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- foo\n\n      bar");
}
#[test]
fn test_example_6_md_prose_wrapalways_format_1_fabddaf7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(">\t\tfoo");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, ">     \tfoo");
}
#[test]
fn test_example_7_md_prose_wrapalways_format_1_8c04abd5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("-\t\tfoo");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "-     foo");
}
#[test]
fn test_example_8_md_prose_wrapalways_format_1_ea220e05() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("    foo\n\tbar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "    foo\n    bar");
}
#[test]
fn test_example_9_md_prose_wrapalways_format_1_c9ede15d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(" - foo\n   - bar\n\t - baz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- foo\n  - bar\n  - baz");
}
#[test]
fn test_example_10_md_prose_wrapalways_format_1_bf62e2d6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- \\`one\n- two\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- \\`one\n- two\\`");
}
#[test]
fn test_example_11_md_prose_wrapalways_format_1_d6ff794d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!-- placeholder for continuous thematic breaks not being treated as yaml -->\n\n***\n---\n___") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!-- placeholder for continuous thematic breaks not being treated as yaml -->\n\n---\n\n---\n\n---");
}
#[test]
fn test_example_12_md_prose_wrapalways_format_1_decc3707() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("+++");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "+++");
}
#[test]
fn test_example_13_md_prose_wrapalways_format_1_57de602f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("===");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "===");
}
#[test]
fn test_example_14_md_prose_wrapalways_format_1_c2ce7512() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("--\n**\n__");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "-- \\\\*\\\\* \\\\_\\\\_");
}
#[test]
fn test_example_15_md_prose_wrapalways_format_1_c3cfe817() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!-- placeholder for continuous thematic breaks not being treated as yaml -->\n\n ***\n  ***\n   ***") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!-- placeholder for continuous thematic breaks not being treated as yaml -->\n\n---\n\n---\n\n---");
}
#[test]
fn test_example_16_md_prose_wrapalways_format_1_7815e4a9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("    ***");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "    ***");
}
#[test]
fn test_example_17_md_prose_wrapalways_format_1_f79d607c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("Foo\n    ***");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "Foo \\\\*\\\\*\\\\*");
}
#[test]
fn test_example_18_md_prose_wrapalways_format_1_8ca779c8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_____________________________________");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "---");
}
#[test]
fn test_example_19_md_prose_wrapalways_format_1_f316afb0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(" - - -");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "---");
}
#[test]
fn test_example_20_md_prose_wrapalways_format_1_e0dc463c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(" **  * ** * ** * **");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "---");
}
#[test]
fn test_example_21_md_prose_wrapalways_format_1_60af7d80() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("-     -      -      -");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "---");
}
#[test]
fn test_example_22_md_prose_wrapalways_format_1_0113d938() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- - - -    ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "---");
}
#[test]
fn test_example_23_md_prose_wrapalways_format_1_c50320d6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_ _ _ _ a\n\na------\n\n---a---");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\\\_ \\\\_ \\\\_ \\\\_ a\n\na------\n\n---a---");
}
#[test]
fn test_example_25_md_prose_wrapalways_format_1_56180ac3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- foo\n***\n- bar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- foo\n\n---\n\n- bar");
}
#[test]
fn test_example_26_md_prose_wrapalways_format_1_b8bd3551() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("Foo\n***\nbar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "Foo\n\n---\n\nbar");
}
#[test]
fn test_example_27_md_prose_wrapalways_format_1_2d8ac444() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("Foo\n---\nbar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "## Foo\n\nbar");
}
#[test]
fn test_example_28_md_prose_wrapalways_format_1_9d914e96() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("* Foo\n* * *\n* Bar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- Foo\n\n---\n\n- Bar");
}
#[test]
fn test_example_29_md_prose_wrapalways_format_1_92e6098f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- Foo\n- * * *");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- Foo\n- ***");
}
#[test]
fn test_example_30_md_prose_wrapalways_format_1_e7607693() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("# foo\n## foo\n### foo\n#### foo\n##### foo\n###### foo");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "# foo\n\n## foo\n\n### foo\n\n#### foo\n\n##### foo\n\n###### foo"
    );
}
#[test]
fn test_example_31_md_prose_wrapalways_format_1_6588dd59() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("####### foo");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "####### foo");
}
#[test]
fn test_example_32_md_prose_wrapalways_format_1_ed01983c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("#5 bolt\n\n#hashtag");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "#5 bolt\n\n#hashtag");
}
#[test]
fn test_example_33_md_prose_wrapalways_format_1_1fd9a9f0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("#\tfoo");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "# foo");
}
#[test]
fn test_example_34_md_prose_wrapalways_format_1_4532c196() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\\\## foo");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\\\## foo");
}
#[test]
fn test_example_35_md_prose_wrapalways_format_1_c946094a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("# foo *bar* \\\\*baz\\\\*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "# foo _bar_ \\\\*baz\\\\*");
}
#[test]
fn test_example_36_md_prose_wrapalways_format_1_bb8a98e7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("#                  foo                     ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "# foo");
}
#[test]
fn test_example_37_md_prose_wrapalways_format_1_2e2ab660() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(" ### foo\n  ## foo\n   # foo");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "### foo\n\n## foo\n\n# foo");
}
#[test]
fn test_example_38_md_prose_wrapalways_format_1_2af104be() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("    # foo");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "    # foo");
}
#[test]
fn test_example_39_md_prose_wrapalways_format_1_2237e264() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo\n    # bar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo # bar");
}
#[test]
fn test_example_40_md_prose_wrapalways_format_1_7bbdee45() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("## foo ##\n  ###   bar    ###");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "## foo\n\n### bar");
}
#[test]
fn test_example_41_md_prose_wrapalways_format_1_4fbd2718() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("# foo ##################################\n##### foo ##");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "# foo\n\n##### foo");
}
#[test]
fn test_example_42_md_prose_wrapalways_format_1_1cdc4af1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("### foo ###     ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "### foo");
}
#[test]
fn test_example_43_md_prose_wrapalways_format_1_76af64d0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("### foo ### b");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "### foo ### b");
}
#[test]
fn test_example_44_md_prose_wrapalways_format_1_2fc6957c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("# foo#");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "# foo#");
}
#[test]
fn test_example_45_md_prose_wrapalways_format_1_67c0283e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("### foo \\\\###\n## foo #\\\\##\n# foo \\\\#");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "### foo \\\\###\n\n## foo #\\\\##\n\n# foo \\\\#"
    );
}
#[test]
fn test_example_46_md_prose_wrapalways_format_1_e9e5654f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!-- placeholder for continuous thematic breaks not being treated as yaml -->\n\n****\n## foo\n****") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!-- placeholder for continuous thematic breaks not being treated as yaml -->\n\n---\n\n## foo\n\n---");
}
#[test]
fn test_example_47_md_prose_wrapalways_format_1_abe45ed2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("Foo bar\n# baz\nBar foo");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "Foo bar\n\n# baz\n\nBar foo");
}
#[test]
fn test_example_48_md_prose_wrapalways_format_1_a13e1fd5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("## \n#\n### ###");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "##\n\n#\n\n###");
}
#[test]
fn test_example_49_md_prose_wrapalways_format_1_2ea5b2d4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("Foo *bar*\n=========\n\nFoo *bar*\n---------");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "# Foo _bar_\n\n## Foo _bar_");
}
#[test]
fn test_example_50_md_prose_wrapalways_format_1_21693463() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("Foo *bar\nbaz*\n====");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "Foo _bar baz_ ====");
}
#[test]
fn test_example_51_md_prose_wrapalways_format_1_9e78a4ee() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("Foo\n-------------------------\n\nFoo\n=");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "## Foo\n\n# Foo");
}
#[test]
fn test_example_52_md_prose_wrapalways_format_1_7b5330bd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("   Foo\n---\n\n  Foo\n-----\n\n  Foo\n  ===");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "## Foo\n\n## Foo\n\nFoo ===");
}
#[test]
fn test_example_53_md_prose_wrapalways_format_1_a42b0730() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("    Foo\n    ---\n\n    Foo\n---");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "    Foo\n    ---\n\n    Foo\n\n---");
}
#[test]
fn test_example_54_md_prose_wrapalways_format_1_cb1375af() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("Foo\n   ----      ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "Foo\n\n---");
}
#[test]
fn test_example_55_md_prose_wrapalways_format_1_fbe2e253() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("Foo\n    ---");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "Foo ---");
}
#[test]
fn test_example_56_md_prose_wrapalways_format_1_81c9870e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("Foo\n= =\n\nFoo\n--- -");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "Foo = =\n\nFoo\n\n---");
}
#[test]
fn test_example_57_md_prose_wrapalways_format_1_e32f33a1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("Foo  \n-----");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "## Foo");
}
#[test]
fn test_example_58_md_prose_wrapalways_format_1_b35547a9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("Foo\\\\\n----");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "## Foo\\\\");
}
#[test]
fn test_example_59_md_prose_wrapalways_format_1_fcf51c1e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("\\`Foo\n----\n\\`\n\n<a title=\"a lot\n---\nof dashes\"/>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "## \\`Foo\n\n\\`\n\n## <a title=\"a lot\n\nof dashes\"/>"
    );
}
#[test]
fn test_example_60_md_prose_wrapalways_format_1_7e387c98() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> Foo\n---");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "> Foo\n\n---");
}
#[test]
fn test_example_61_md_prose_wrapalways_format_1_eb8c1dd7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> foo\nbar\n===");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "> foo\n\n# bar");
}
#[test]
fn test_example_62_md_prose_wrapalways_format_1_0bd258cc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- Foo\n---");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- Foo\n\n---");
}
#[test]
fn test_example_63_md_prose_wrapalways_format_1_41500752() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("Foo\nBar\n---");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "Foo Bar\n\n---");
}
#[test]
fn test_example_64_md_prose_wrapalways_format_1_aa58f43b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\nFoo\n---\nBar\n---\nBaz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "---\nFoo\n---\n\n## Bar\n\nBaz");
}
#[test]
fn test_example_65_md_prose_wrapalways_format_1_fd0dc24a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\n====");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "====");
}
#[test]
fn test_example_66_md_prose_wrapalways_format_1_4b12a98d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\n---");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "---\n---");
}
#[test]
fn test_example_67_md_prose_wrapalways_format_1_fa86cc5a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- foo\n-----");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- foo\n\n---");
}
#[test]
fn test_example_68_md_prose_wrapalways_format_1_b46af2c0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("    foo\n---");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "    foo\n\n---");
}
#[test]
fn test_example_69_md_prose_wrapalways_format_1_c1abd92f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> foo\n-----");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "> foo\n\n---");
}
#[test]
fn test_example_70_md_prose_wrapalways_format_1_4c8e9ee7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\\\> foo\n------");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "## \\\\> foo");
}
#[test]
fn test_example_71_md_prose_wrapalways_format_1_69e40067() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("Foo\n\nbar\n---\nbaz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "Foo\n\n## bar\n\nbaz");
}
#[test]
fn test_example_72_md_prose_wrapalways_format_1_d401f9c5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("Foo\nbar\n\n---\n\nbaz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "Foo bar\n\n---\n\nbaz");
}
#[test]
fn test_example_73_md_prose_wrapalways_format_1_e29d2070() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("Foo\nbar\n* * *\nbaz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "Foo bar\n\n---\n\nbaz");
}
#[test]
fn test_example_74_md_prose_wrapalways_format_1_20490add() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("Foo\nbar\n\\\\---\nbaz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "Foo bar \\\\--- baz");
}
#[test]
fn test_example_75_md_prose_wrapalways_format_1_d713304f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("    a simple\n      indented code block");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "    a simple\n      indented code block");
}
#[test]
fn test_example_76_md_prose_wrapalways_format_1_edcc7d52() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("  - foo\n\n    bar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- foo\n\n  bar");
}
#[test]
fn test_example_77_md_prose_wrapalways_format_1_7446cee5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1.  foo\n\n    - bar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "1.  foo\n\n    - bar");
}
#[test]
fn test_example_78_md_prose_wrapalways_format_1_78488ccb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("    <a/>\n    *hi*\n\n    - one");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "    <a/>\n    *hi*\n\n    - one");
}
#[test]
fn test_example_79_md_prose_wrapalways_format_1_08b9915e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("    chunk1\n\n    chunk2\n  \n \n \n    chunk3");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "    chunk1\n\n    chunk2\n\n\n\n    chunk3");
}
#[test]
fn test_example_80_md_prose_wrapalways_format_1_b34724f4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("    chunk1\n      \n      chunk2");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "    chunk1\n\n      chunk2");
}
#[test]
fn test_example_81_md_prose_wrapalways_format_1_231b9453() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("Foo\n    bar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "Foo bar");
}
#[test]
fn test_example_82_md_prose_wrapalways_format_1_3c3d90c9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("    foo\nbar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "    foo\n\nbar");
}
#[test]
fn test_example_83_md_prose_wrapalways_format_1_10e90bc9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("# Heading\n    foo\nHeading\n------\n    foo\n----");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "# Heading\n\n    foo\n\n## Heading\n\n    foo\n\n---"
    );
}
#[test]
fn test_example_84_md_prose_wrapalways_format_1_69851ce7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("        foo\n    bar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "        foo\n    bar");
}
#[test]
fn test_example_85_md_prose_wrapalways_format_1_42fbafd2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\n    \n    foo\n    ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "    foo");
}
#[test]
fn test_example_86_md_prose_wrapalways_format_1_58f0098c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("    foo  ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "    foo");
}
#[test]
fn test_example_87_md_prose_wrapalways_format_1_07410780() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`\\`\\`\n<\n >\n\\`\\`\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`\\`\\`\n<\n >\n\\`\\`\\`");
}
#[test]
fn test_example_88_md_prose_wrapalways_format_1_20487dd8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("~~~\n<\n >\n~~~");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`\\`\\`\n<\n >\n\\`\\`\\`");
}
#[test]
fn test_example_89_md_prose_wrapalways_format_1_01c8ed54() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`\\`\\`\naaa\n~~~\n\\`\\`\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`\\`\\`\naaa\n~~~\n\\`\\`\\`");
}
#[test]
fn test_example_90_md_prose_wrapalways_format_1_4f6ce27f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("~~~\naaa\n\\`\\`\\`\n~~~");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`\\`\\`\\`\naaa\n\\`\\`\\`\n\\`\\`\\`\\`");
}
#[test]
fn test_example_91_md_prose_wrapalways_format_1_34dc9359() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`\\`\\`\\`\naaa\n\\`\\`\\`\n\\`\\`\\`\\`\\`\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`\\`\\`\\`\naaa\n\\`\\`\\`\n\\`\\`\\`\\`");
}
#[test]
fn test_example_92_md_prose_wrapalways_format_1_962895c7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("~~~~\naaa\n~~~\n~~~~");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`\\`\\`\naaa\n~~~\n\\`\\`\\`");
}
#[test]
fn test_example_93_md_prose_wrapalways_format_1_6bfec7e5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`\\`\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`\\`\\`\n\n\\`\\`\\`");
}
#[test]
fn test_example_94_md_prose_wrapalways_format_1_cff72e39() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`\\`\\`\\`\\`\n\n\\`\\`\\`\naaa");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`\\`\\`\\`\n\n\\`\\`\\`\naaa\n\\`\\`\\`\\`");
}
#[test]
fn test_example_95_md_prose_wrapalways_format_1_e9e25087() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> \\`\\`\\`\n> aaa\n\nbbb");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "> \\`\\`\\`\n> aaa\n> \\`\\`\\`\n\nbbb");
}
#[test]
fn test_example_96_md_prose_wrapalways_format_1_90f9a158() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`\\`\\`\n\n  \n\\`\\`\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`\\`\\`\n\n\n\\`\\`\\`");
}
#[test]
fn test_example_97_md_prose_wrapalways_format_1_9671175c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`\\`\\`\n\\`\\`\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`\\`\\`\n\n\\`\\`\\`");
}
#[test]
fn test_example_98_md_prose_wrapalways_format_1_1b84e454() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(" \\`\\`\\`\n aaa\naaa\n\\`\\`\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`\\`\\`\naaa\naaa\n\\`\\`\\`");
}
#[test]
fn test_example_99_md_prose_wrapalways_format_1_15e3730a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("  \\`\\`\\`\naaa\n  aaa\naaa\n  \\`\\`\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`\\`\\`\naaa\naaa\naaa\n\\`\\`\\`");
}
#[test]
fn test_example_100_md_prose_wrapalways_format_1_c3b997c2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("   \\`\\`\\`\n   aaa\n    aaa\n  aaa\n   \\`\\`\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`\\`\\`\naaa\n aaa\naaa\n\\`\\`\\`");
}
#[test]
fn test_example_101_md_prose_wrapalways_format_1_d6de8098() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("    \\`\\`\\`\n    aaa\n    \\`\\`\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "    \\`\\`\\`\n    aaa\n    \\`\\`\\`");
}
#[test]
fn test_example_102_md_prose_wrapalways_format_1_f1734785() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`\\`\\`\naaa\n  \\`\\`\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`\\`\\`\naaa\n\\`\\`\\`");
}
#[test]
fn test_example_103_md_prose_wrapalways_format_1_7a00aeef() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("   \\`\\`\\`\naaa\n  \\`\\`\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`\\`\\`\naaa\n\\`\\`\\`");
}
#[test]
fn test_example_104_md_prose_wrapalways_format_1_0b0f85fc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`\\`\\`\naaa\n    \\`\\`\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`\\`\\`\\`\naaa\n    \\`\\`\\`\n\\`\\`\\`\\`");
}
#[test]
fn test_example_105_md_prose_wrapalways_format_1_31fde435() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`\\`\\` \\`\\`\\`\naaa");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\` \\` aaa");
}
#[test]
fn test_example_106_md_prose_wrapalways_format_1_be7fd741() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("~~~~~~\naaa\n~~~ ~~");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`\\`\\`\naaa\n~~~ ~~\n\\`\\`\\`");
}
#[test]
fn test_example_107_md_prose_wrapalways_format_1_4b8cdb62() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo\n\\`\\`\\`\nbar\n\\`\\`\\`\nbaz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo\n\n\\`\\`\\`\nbar\n\\`\\`\\`\n\nbaz");
}
#[test]
fn test_example_108_md_prose_wrapalways_format_1_e497f992() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo\n---\n~~~\nbar\n~~~\n# baz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "## foo\n\n\\`\\`\\`\nbar\n\\`\\`\\`\n\n# baz");
}
#[test]
fn test_example_109_md_prose_wrapalways_format_1_a9eee036() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`\\`\\`ruby\ndef foo(x)\n  return 3\nend\n\\`\\`\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "\\`\\`\\`ruby\ndef foo(x)\n  return 3\nend\n\\`\\`\\`"
    );
}
#[test]
fn test_example_110_md_prose_wrapalways_format_1_23677125() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("~~~~    ruby startline=3 $%@#$\ndef foo(x)\n  return 3\nend\n~~~~~~~");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "\\`\\`\\`ruby startline=3 $%@#$\ndef foo(x)\n  return 3\nend\n\\`\\`\\`"
    );
}
#[test]
fn test_example_111_md_prose_wrapalways_format_1_6829adc9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`\\`\\`\\`;\n\\`\\`\\`\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`\\`\\`;\n\n\\`\\`\\`");
}
#[test]
fn test_example_112_md_prose_wrapalways_format_1_753e214f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`\\`\\` aa \\`\\`\\`\nfoo");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`aa\\` foo");
}
#[test]
fn test_example_113_md_prose_wrapalways_format_1_509d33a6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`\\`\\`\n\\`\\`\\` aaa\n\\`\\`\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`\\`\\`\\`\n\\`\\`\\` aaa\n\\`\\`\\`\\`");
}
#[test]
fn test_example_114_md_prose_wrapalways_format_1_ae07affe() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("<table>\n  <tr>\n    <td>\n           hi\n    </td>\n  </tr>\n</table>\n\nokay.");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<table>\n  <tr>\n    <td>\n           hi\n    </td>\n  </tr>\n</table>\n\nokay."
    );
}
#[test]
fn test_example_115_md_prose_wrapalways_format_1_cb9f3112() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(" <div>\n  *hello*\n         <foo><a>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, " <div>\n  *hello*\n         <foo><a>");
}
#[test]
fn test_example_116_md_prose_wrapalways_format_1_79ed9693() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("</div>\n*foo*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "</div>\n*foo*");
}
#[test]
fn test_example_117_md_prose_wrapalways_format_1_53aadf01() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<DIV CLASS=\"foo\">\n\n*Markdown*\n\n</DIV>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<DIV CLASS=\"foo\">\n\n_Markdown_\n\n</DIV>");
}
#[test]
fn test_example_118_md_prose_wrapalways_format_1_e19fe95d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<div id=\"foo\"\n  class=\"bar\">\n</div>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<div id=\"foo\"\n  class=\"bar\">\n</div>");
}
#[test]
fn test_example_119_md_prose_wrapalways_format_1_01bc6289() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<div id=\"foo\" class=\"bar\n  baz\">\n</div>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<div id=\"foo\" class=\"bar\n  baz\">\n</div>");
}
#[test]
fn test_example_120_md_prose_wrapalways_format_1_43bdb39e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<div>\n*foo*\n\n*bar*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<div>\n*foo*\n\n_bar_");
}
#[test]
fn test_example_121_md_prose_wrapalways_format_1_f9185592() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<div id=\"foo\"\n*hi*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<div id=\"foo\"\n*hi*");
}
#[test]
fn test_example_122_md_prose_wrapalways_format_1_bfaef784() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<div class\nfoo");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<div class\nfoo");
}
#[test]
fn test_example_123_md_prose_wrapalways_format_1_b920394f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<div *???-&&&-<---\n*foo*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<div *???-&&&-<---\n*foo*");
}
#[test]
fn test_example_124_md_prose_wrapalways_format_1_90faae4b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<div><a href=\"bar\">*foo*</a></div>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<div><a href=\"bar\">*foo*</a></div>");
}
#[test]
fn test_example_125_md_prose_wrapalways_format_1_520a13ba() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<table><tr><td>\nfoo\n</td></tr></table>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<table><tr><td>\nfoo\n</td></tr></table>");
}
#[test]
fn test_example_126_md_prose_wrapalways_format_1_a4f5edfa() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<div></div>\n\\`\\`\\` c\nint x = 33;\n\\`\\`\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<div></div>\n\\`\\`\\` c\nint x = 33;\n\\`\\`\\`"
    );
}
#[test]
fn test_example_127_md_prose_wrapalways_format_1_49785071() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<a href=\"foo\">\n*bar*\n</a>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<a href=\"foo\">\n*bar*\n</a>");
}
#[test]
fn test_example_128_md_prose_wrapalways_format_1_76868681() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<Warning>\n*bar*\n</Warning>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<Warning>\n*bar*\n</Warning>");
}
#[test]
fn test_example_129_md_prose_wrapalways_format_1_cda09266() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<i class=\"foo\">\n*bar*\n</i>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<i class=\"foo\">\n*bar*\n</i>");
}
#[test]
fn test_example_130_md_prose_wrapalways_format_1_42883de8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("</ins>\n*bar*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "</ins>\n*bar*");
}
#[test]
fn test_example_131_md_prose_wrapalways_format_1_db3a50cd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<del>\n*foo*\n</del>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<del>\n*foo*\n</del>");
}
#[test]
fn test_example_132_md_prose_wrapalways_format_1_0474d241() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<del>\n\n*foo*\n\n</del>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<del>\n\n_foo_\n\n</del>");
}
#[test]
fn test_example_133_md_prose_wrapalways_format_1_d9a2b7c4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<del>*foo*</del>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<del>_foo_</del>");
}
#[test]
fn test_example_134_md_prose_wrapalways_format_1_0c6bd53a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<pre language=\"haskell\"><code>\nimport Text.HTML.TagSoup\n\nmain :: IO ()\nmain = print $ parseTags tags\n</code></pre>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<pre language=\"haskell\"><code>\nimport Text.HTML.TagSoup\n\nmain :: IO ()\nmain = print $ parseTags tags\n</code></pre>");
}
#[test]
fn test_example_135_md_prose_wrapalways_format_1_da2b5872() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script type=\"text/javascript\">\n// JavaScript example\n\ndocument.getElementById(\"demo\").innerHTML = \"Hello JavaScript!\";\n</script>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<script type=\"text/javascript\">\n// JavaScript example\n\ndocument.getElementById(\"demo\").innerHTML = \"Hello JavaScript!\";\n</script>");
}
#[test]
fn test_example_136_md_prose_wrapalways_format_1_bc9be708() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("<style\n  type=\"text/css\">\nh1 {color:red;}\n\np {color:blue;}\n</style>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<style\n  type=\"text/css\">\nh1 {color:red;}\n\np {color:blue;}\n</style>"
    );
}
#[test]
fn test_example_137_md_prose_wrapalways_format_1_56a5b218() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<style\n  type=\"text/css\">\n\nfoo");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<style\n  type=\"text/css\">\n\nfoo");
}
#[test]
fn test_example_138_md_prose_wrapalways_format_1_92a8c0dd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> <div>\n> foo\n\nbar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "> <div>\n> foo\n\nbar");
}
#[test]
fn test_example_139_md_prose_wrapalways_format_1_8a5d08f5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- <div>\n- foo");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- <div>\n- foo");
}
#[test]
fn test_example_140_md_prose_wrapalways_format_1_1c5206af() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<style>p{color:red;}</style>\n*foo*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<style>p{color:red;}</style>\n\n_foo_");
}
#[test]
fn test_example_141_md_prose_wrapalways_format_1_f6396171() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<!-- foo -->*bar*\n*baz*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<!-- foo -->*bar*\n\n_baz_");
}
#[test]
fn test_example_142_md_prose_wrapalways_format_1_3139a4b3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<script>\nfoo\n</script>1. *bar*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<script>\nfoo\n</script>1. *bar*");
}
#[test]
fn test_example_143_md_prose_wrapalways_format_1_f6d4c9b8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<!-- Foo\n\nbar\n   baz -->");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<!-- Foo\n\nbar\n   baz -->");
}
#[test]
fn test_example_144_md_prose_wrapalways_format_1_754bd16b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<?php\n\n  echo '>';\n\n?>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<?php\n\n  echo '>';\n\n?>");
}
#[test]
fn test_example_145_md_prose_wrapalways_format_1_ff0248f1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<!DOCTYPE html>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<!DOCTYPE html>");
}
#[test]
fn test_example_146_md_prose_wrapalways_format_1_5fe81c48() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<![CDATA[\nfunction matchwo(a,b)\n{\n  if (a < b && a < 0) then {\n    return 1;\n\n  } else {\n\n    return 0;\n  }\n}\n]]>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<![CDATA[\nfunction matchwo(a,b)\n{\n  if (a < b && a < 0) then {\n    return 1;\n\n  } else {\n\n    return 0;\n  }\n}\n]]>");
}
#[test]
fn test_example_147_md_prose_wrapalways_format_1_996d5908() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("  <!-- foo -->\n\n    <!-- foo -->");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "  <!-- foo -->\n\n    <!-- foo -->");
}
#[test]
fn test_example_148_md_prose_wrapalways_format_1_0ea2cc2e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("  <div>\n\n    <div>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "  <div>\n\n    <div>");
}
#[test]
fn test_example_149_md_prose_wrapalways_format_1_f0c611fd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("Foo\n<div>\nbar\n</div>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "Foo\n\n<div>\nbar\n</div>");
}
#[test]
fn test_example_150_md_prose_wrapalways_format_1_e5a9adc3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<div>\nbar\n</div>\n*foo*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<div>\nbar\n</div>\n*foo*");
}
#[test]
fn test_example_151_md_prose_wrapalways_format_1_a2acba3c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("Foo\n<a href=\"bar\">\nbaz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "Foo <a href=\"bar\"> baz");
}
#[test]
fn test_example_152_md_prose_wrapalways_format_1_d88160ef() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<div>\n\n*Emphasized* text.\n\n</div>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<div>\n\n_Emphasized_ text.\n\n</div>");
}
#[test]
fn test_example_153_md_prose_wrapalways_format_1_ccc78af5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<div>\n*Emphasized* text.\n</div>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<div>\n*Emphasized* text.\n</div>");
}
#[test]
fn test_example_154_md_prose_wrapalways_format_1_edb5e75d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("<table>\n\n<tr>\n\n<td>\nHi\n</td>\n\n</tr>\n\n</table>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<table>\n\n<tr>\n\n<td>\nHi\n</td>\n\n</tr>\n\n</table>"
    );
}
#[test]
fn test_example_155_md_prose_wrapalways_format_1_0b3262d4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("<table>\n\n  <tr>\n\n    <td>\n      Hi\n    </td>\n\n  </tr>\n\n</table>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<table>\n\n  <tr>\n\n    <td>\n      Hi\n    </td>\n\n  </tr>\n\n</table>"
    );
}
#[test]
fn test_example_156_md_prose_wrapalways_format_1_e4bb2e2a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo]: /url \"title\"\n\n[foo]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo]: /url \"title\"\n\n[foo]");
}
#[test]
fn test_example_157_md_prose_wrapalways_format_1_9ab2cfee() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("   [foo]: \n      /url  \n           'the title'  \n\n[foo]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo]: /url \"the title\"\n\n[foo]");
}
#[test]
fn test_example_158_md_prose_wrapalways_format_1_9b9b2f74() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("[Foo*bar\\\\]]:my_(url) 'title (with parens)'\n\n[Foo*bar\\\\]]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "[Foo*bar\\\\]]: my_(url) \"title (with parens)\"\n\n[Foo*bar\\\\]]"
    );
}
#[test]
fn test_example_159_md_prose_wrapalways_format_1_11ca5786() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[Foo bar]:\n<my%20url>\n'title'\n\n[Foo bar]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[Foo bar]: my%20url \"title\"\n\n[Foo bar]");
}
#[test]
fn test_example_161_md_prose_wrapalways_format_1_0f33e142() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo]: /url 'title\n\nwith blank line'\n\n[foo]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo]: /url 'title\n\nwith blank line'\n\n[foo]");
}
#[test]
fn test_example_162_md_prose_wrapalways_format_1_46d97ade() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo]:\n/url\n\n[foo]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo]: /url\n\n[foo]");
}
#[test]
fn test_example_163_md_prose_wrapalways_format_1_7195a23f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo]:\n\n[foo]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo]:\n\n[foo]");
}
#[test]
fn test_example_164_md_prose_wrapalways_format_1_10e7a81a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("[foo]: /url\\\\bar\\\\*baz \"foo\\\\\"bar\\\\baz\"\n\n[foo]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "[foo]: /url\\\\bar\\\\*baz \"foo\\\\\"bar\\\\baz\"\n\n[foo]"
    );
}
#[test]
fn test_example_165_md_prose_wrapalways_format_1_1c83f5da() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo]\n\n[foo]: url");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo]\n\n[foo]: url");
}
#[test]
fn test_example_166_md_prose_wrapalways_format_1_f8bcbc2b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo]\n\n[foo]: first\n[foo]: second");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo]\n\n[foo]: first\n[foo]: second");
}
#[test]
fn test_example_167_md_prose_wrapalways_format_1_0dca8bce() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[FOO]: /url\n\n[Foo]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[FOO]: /url\n\n[Foo]");
}
#[test]
fn test_example_168_md_prose_wrapalways_format_1_941a31f3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[ΑΓΩ]: /φου\n\n[αγω]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[ΑΓΩ]: /φου\n\n[αγω]");
}
#[test]
fn test_example_169_md_prose_wrapalways_format_1_09ba42f9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo]: /url");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo]: /url");
}
#[test]
fn test_example_170_md_prose_wrapalways_format_1_92888674() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[\nfoo\n]: /url\nbar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[ foo ]: /url\n\nbar");
}
#[test]
fn test_example_171_md_prose_wrapalways_format_1_bd27a418() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo]: /url \"title\" ok");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo]: /url \"title\" ok");
}
#[test]
fn test_example_172_md_prose_wrapalways_format_1_cf9620c2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo]: /url\n\"title\" ok");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo]: /url \"title\" ok");
}
#[test]
fn test_example_173_md_prose_wrapalways_format_1_c814f36d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("    [foo]: /url \"title\"\n\n[foo]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "    [foo]: /url \"title\"\n\n[foo]");
}
#[test]
fn test_example_174_md_prose_wrapalways_format_1_afeeedb4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`\\`\\`\n[foo]: /url\n\\`\\`\\`\n\n[foo]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`\\`\\`\n[foo]: /url\n\\`\\`\\`\n\n[foo]");
}
#[test]
fn test_example_175_md_prose_wrapalways_format_1_4dcfe235() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("Foo\n[bar]: /baz\n\n[bar]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "Foo [bar]: /baz\n\n[bar]");
}
#[test]
fn test_example_176_md_prose_wrapalways_format_1_14ff9ba0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("# [Foo]\n[foo]: /url\n> bar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "# [Foo]\n\n[foo]: /url\n\n> bar");
}
#[test]
fn test_example_177_md_prose_wrapalways_format_1_ce30c820() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[foo]: /foo-url \"foo\"\n[bar]: /bar-url\n  \"bar\"\n[baz]: /baz-url\n\n[foo],\n[bar],\n[baz]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "[foo]: /foo-url \"foo\"\n[bar]: /bar-url \"bar\"\n[baz]: /baz-url\n\n[foo], [bar], [baz]"
    );
}
#[test]
fn test_example_178_md_prose_wrapalways_format_1_411dd538() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo]\n\n> [foo]: /url");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo]\n\n> [foo]: /url");
}
#[test]
fn test_example_179_md_prose_wrapalways_format_1_87fa291d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("aaa\n\nbbb");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "aaa\n\nbbb");
}
#[test]
fn test_example_180_md_prose_wrapalways_format_1_44b1f64c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("aaa\nbbb\n\nccc\nddd");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "aaa bbb\n\nccc ddd");
}
#[test]
fn test_example_181_md_prose_wrapalways_format_1_87367358() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("aaa\n\n\nbbb");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "aaa\n\nbbb");
}
#[test]
fn test_example_182_md_prose_wrapalways_format_1_6ffaeeb2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("  aaa\n bbb");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "aaa bbb");
}
#[test]
fn test_example_183_md_prose_wrapalways_format_1_2cb050bf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("aaa\n             bbb\n                                       ccc");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "aaa bbb ccc");
}
#[test]
fn test_example_184_md_prose_wrapalways_format_1_186add0d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("   aaa\nbbb");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "aaa bbb");
}
#[test]
fn test_example_185_md_prose_wrapalways_format_1_7b305e1f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("    aaa\nbbb");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "    aaa\n\nbbb");
}
#[test]
fn test_example_186_md_prose_wrapalways_format_1_7f2d9d08() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("aaa     \nbbb     ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "aaa  \nbbb");
}
#[test]
fn test_example_187_md_prose_wrapalways_format_1_604120aa() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("  \n\naaa\n  \n\n# aaa\n\n  ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "aaa\n\n# aaa");
}
#[test]
fn test_example_188_md_prose_wrapalways_format_1_d711d2cf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> # Foo\n> bar\n> baz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "> # Foo\n>\n> bar baz");
}
#[test]
fn test_example_189_md_prose_wrapalways_format_1_af5a3880() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("># Foo\n>bar\n> baz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "> # Foo\n>\n> bar baz");
}
#[test]
fn test_example_190_md_prose_wrapalways_format_1_162c2bb1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("   > # Foo\n   > bar\n > baz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "> # Foo\n>\n> bar baz");
}
#[test]
fn test_example_191_md_prose_wrapalways_format_1_6c171842() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("    > # Foo\n    > bar\n    > baz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "    > # Foo\n    > bar\n    > baz");
}
#[test]
fn test_example_192_md_prose_wrapalways_format_1_f9c48ff4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> # Foo\n> bar\nbaz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "> # Foo\n>\n> bar baz");
}
#[test]
fn test_example_193_md_prose_wrapalways_format_1_5cb4f714() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> bar\nbaz\n> foo");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "> bar baz foo");
}
#[test]
fn test_example_194_md_prose_wrapalways_format_1_28f46b0d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> foo\n---");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "> foo\n\n---");
}
#[test]
fn test_example_195_md_prose_wrapalways_format_1_00d85718() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> - foo\n- bar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "> - foo\n\n- bar");
}
#[test]
fn test_example_196_md_prose_wrapalways_format_1_119bc915() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(">     foo\n    bar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, ">     foo\n\n    bar");
}
#[test]
fn test_example_197_md_prose_wrapalways_format_1_f41e3b1e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> \\`\\`\\`\nfoo\n\\`\\`\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "> \\`\\`\\`\n> foo\n> \\`\\`\\`\n\n\\`\\`\\`\n\n\\`\\`\\`"
    );
}
#[test]
fn test_example_198_md_prose_wrapalways_format_1_44f39f13() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> foo\n    - bar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "> foo\n\n    - bar");
}
#[test]
fn test_example_199_md_prose_wrapalways_format_1_0063a1f4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(">");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, ">");
}
#[test]
fn test_example_200_md_prose_wrapalways_format_1_c7d60b96() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(">\n>  \n> ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, ">");
}
#[test]
fn test_example_201_md_prose_wrapalways_format_1_a4138a1b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(">\n> foo\n>  ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "> foo");
}
#[test]
fn test_example_202_md_prose_wrapalways_format_1_7d34d0a1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> foo\n\n> bar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "> foo\n\n> bar");
}
#[test]
fn test_example_203_md_prose_wrapalways_format_1_bce05d73() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> foo\n> bar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "> foo bar");
}
#[test]
fn test_example_204_md_prose_wrapalways_format_1_3524e6c5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> foo\n>\n> bar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "> foo\n>\n> bar");
}
#[test]
fn test_example_205_md_prose_wrapalways_format_1_261eebe0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo\n> bar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo\n\n> bar");
}
#[test]
fn test_example_206_md_prose_wrapalways_format_1_84bc1f1d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> aaa\n***\n> bbb");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "> aaa\n\n---\n\n> bbb");
}
#[test]
fn test_example_207_md_prose_wrapalways_format_1_5608c20d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> bar\nbaz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "> bar baz");
}
#[test]
fn test_example_208_md_prose_wrapalways_format_1_ccc38cf3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> bar\n\nbaz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "> bar\n\nbaz");
}
#[test]
fn test_example_209_md_prose_wrapalways_format_1_fdff58ed() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> bar\n>\nbaz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "> bar\n>\n> baz");
}
#[test]
fn test_example_210_md_prose_wrapalways_format_1_06a257a3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> > > foo\nbar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "> > > foo bar");
}
#[test]
fn test_example_211_md_prose_wrapalways_format_1_3b28849e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(">>> foo\n> bar\n>>baz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "> > > foo bar baz");
}
#[test]
fn test_example_212_md_prose_wrapalways_format_1_dcf26be2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(">     code\n\n>    not code");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, ">     code\n\n> not code");
}
#[test]
fn test_example_213_md_prose_wrapalways_format_1_15fb8255() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("A paragraph\nwith two lines.\n\n    indented code\n\n> A block quote.");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "A paragraph with two lines.\n\n    indented code\n\n> A block quote."
    );
}
#[test]
fn test_example_214_md_prose_wrapalways_format_1_f70da9d8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "1.  A paragraph\n    with two lines.\n\n        indented code\n\n    > A block quote.",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "1.  A paragraph with two lines.\n\n        indented code\n\n    > A block quote."
    );
}
#[test]
fn test_example_215_md_prose_wrapalways_format_1_8fb9a266() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- one\n\ntwo");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- one\n\ntwo");
}
#[test]
fn test_example_216_md_prose_wrapalways_format_1_25156c5c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- one\n\n  two");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- one\n\n  two");
}
#[test]
fn test_example_217_md_prose_wrapalways_format_1_544560a6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(" -    one\n\n     two");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- one\n\n  two");
}
#[test]
fn test_example_218_md_prose_wrapalways_format_1_9e89a3e9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(" -    one\n\n      two");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- one\n\n  two");
}
#[test]
fn test_example_219_md_prose_wrapalways_format_1_b1f056ab() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("   > > 1.  one\n>>\n>>     two");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "> > 1.  one\n> >\n> >     two");
}
#[test]
fn test_example_220_md_prose_wrapalways_format_1_4b0c5129() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(">>- one\n>>\n  >  > two");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "> > - one\n> >\n> > two");
}
#[test]
fn test_example_221_md_prose_wrapalways_format_1_1dbe200a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("-one\n\n2.two");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "-one\n\n2.two");
}
#[test]
fn test_example_222_md_prose_wrapalways_format_1_6b085cf7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("- foo\n\n  bar\n\n- foo\n\n\n  bar\n\n- \\`\\`\\`\n  foo\n\n\n  bar\n  \\`\\`\\`\n\n- baz\n\n  + \\`\\`\\`\n    foo\n\n\n    bar\n    \\`\\`\\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "- foo\n\n  bar\n\n- foo\n\n  bar\n\n- \\`\\`\\`\n  foo\n\n\n  bar\n  \\`\\`\\`\n\n- baz\n\n  - \\`\\`\\`\n    foo\n\n\n    bar\n    \\`\\`\\`");
}
#[test]
fn test_example_223_md_prose_wrapalways_format_1_a1341125() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("1.  foo\n\n    \\`\\`\\`\n    bar\n    \\`\\`\\`\n\n    baz\n\n    > bam");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "1.  foo\n\n    \\`\\`\\`\n    bar\n    \\`\\`\\`\n\n    baz\n\n    > bam"
    );
}
#[test]
fn test_example_224_md_prose_wrapalways_format_1_1c08aa8c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- Foo\n\n      bar\n\n      baz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- Foo\n\n      bar\n\n      baz");
}
#[test]
fn test_example_225_md_prose_wrapalways_format_1_4d91cb4f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- Foo\n\n      bar\n\n\n      baz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- Foo\n\n      bar\n\n\n      baz");
}
#[test]
fn test_example_226_md_prose_wrapalways_format_1_e207a224() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("123456789. ok");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "123456789. ok");
}
#[test]
fn test_example_227_md_prose_wrapalways_format_1_812a19e9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1234567890. not ok");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "1234567890. not ok");
}
#[test]
fn test_example_228_md_prose_wrapalways_format_1_1849411f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("0. ok");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "0. ok");
}
#[test]
fn test_example_229_md_prose_wrapalways_format_1_4d3a31c2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("003. ok");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "3. ok");
}
#[test]
fn test_example_230_md_prose_wrapalways_format_1_880b288b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("-1. not ok");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "-1. not ok");
}
#[test]
fn test_example_231_md_prose_wrapalways_format_1_43fe2dcd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- foo\n\n      bar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- foo\n\n      bar");
}
#[test]
fn test_example_232_md_prose_wrapalways_format_1_571c229e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("  10.  foo\n\n           bar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "10. foo\n\n        bar");
}
#[test]
fn test_example_233_md_prose_wrapalways_format_1_13aa1f65() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("    indented code\n\nparagraph\n\n    more code");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "    indented code\n\nparagraph\n\n    more code");
}
#[test]
fn test_example_234_md_prose_wrapalways_format_1_7f75eac3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("1.     indented code\n\n   paragraph\n\n       more code");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "1.      indented code\n\n    paragraph\n\n        more code"
    );
}
#[test]
fn test_example_235_md_prose_wrapalways_format_1_6db69f26() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("1.      indented code\n\n   paragraph\n\n       more code");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "1.       indented code\n\n    paragraph\n\n        more code"
    );
}
#[test]
fn test_example_236_md_prose_wrapalways_format_1_9080c5df() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("   foo\n\nbar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo\n\nbar");
}
#[test]
fn test_example_237_md_prose_wrapalways_format_1_021d177f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("-    foo\n\n  bar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- foo\n\nbar");
}
#[test]
fn test_example_238_md_prose_wrapalways_format_1_2a0bc94b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("-  foo\n\n   bar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- foo\n\n  bar");
}
#[test]
fn test_example_240_md_prose_wrapalways_format_1_c67c396a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("-\n\n  foo");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- foo");
}
#[test]
fn test_example_241_md_prose_wrapalways_format_1_84750dec() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- foo\n-\n- bar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- foo\n-\n- bar");
}
#[test]
fn test_example_242_md_prose_wrapalways_format_1_312a9b67() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- foo\n-   \n- bar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- foo\n-\n- bar");
}
#[test]
fn test_example_243_md_prose_wrapalways_format_1_f1248c81() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1. foo\n2.\n3. bar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "1. foo\n2.\n3. bar");
}
#[test]
fn test_example_244_md_prose_wrapalways_format_1_dc8bcff0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "-");
}
#[test]
fn test_example_245_md_prose_wrapalways_format_1_7614fca6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        " 1.  A paragraph\n     with two lines.\n\n         indented code\n\n     > A block quote.",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "1.  A paragraph with two lines.\n\n        indented code\n\n    > A block quote."
    );
}
#[test]
fn test_example_246_md_prose_wrapalways_format_1_e5978cd4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1.  A paragraph\n      with two lines.\n\n          indented code\n\n      > A block quote.") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "1.  A paragraph with two lines.\n\n        indented code\n\n    > A block quote."
    );
}
#[test]
fn test_example_247_md_prose_wrapalways_format_1_73c3374b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("   1.  A paragraph\n       with two lines.\n\n           indented code\n\n       > A block quote.") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "1.  A paragraph with two lines.\n\n        indented code\n\n    > A block quote."
    );
}
#[test]
fn test_example_248_md_prose_wrapalways_format_1_9fe02956() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("    1.  A paragraph\n        with two lines.\n\n            indented code\n\n        > A block quote.") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "    1.  A paragraph\n        with two lines.\n\n            indented code\n\n        > A block quote.");
}
#[test]
fn test_example_249_md_prose_wrapalways_format_1_43d812d0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "  1.  A paragraph\nwith two lines.\n\n          indented code\n\n      > A block quote.",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "1.  A paragraph with two lines.\n\n              indented code\n\n          > A block quote.");
}
#[test]
fn test_example_250_md_prose_wrapalways_format_1_723c21b2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("  1.  A paragraph\n    with two lines.");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "1.  A paragraph with two lines.");
}
#[test]
fn test_example_251_md_prose_wrapalways_format_1_ec0a822b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> 1. > Blockquote\ncontinued here.");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "> 1. > Blockquote continued here.");
}
#[test]
fn test_example_252_md_prose_wrapalways_format_1_07a03e79() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> 1. > Blockquote\n> continued here.");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "> 1. > Blockquote continued here.");
}
#[test]
fn test_example_253_md_prose_wrapalways_format_1_b0a9c8b4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- foo\n  - bar\n    - baz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- foo\n  - bar\n    - baz");
}
#[test]
fn test_example_254_md_prose_wrapalways_format_1_e84f2a3c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- foo\n - bar\n  - baz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- foo\n- bar\n- baz");
}
#[test]
fn test_example_255_md_prose_wrapalways_format_1_13ea35ff() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("10) foo\n    - bar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "10. foo\n    - bar");
}
#[test]
fn test_example_256_md_prose_wrapalways_format_1_063bda84() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("10) foo\n   - bar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "10. foo\n\n- bar");
}
#[test]
fn test_example_257_md_prose_wrapalways_format_1_8ad61b92() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- - foo");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- - foo");
}
#[test]
fn test_example_258_md_prose_wrapalways_format_1_7568cb65() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1. - 2. foo");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "1. - 2. foo");
}
#[test]
fn test_example_259_md_prose_wrapalways_format_1_80f155bf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- # Foo\n- Bar\n  ---\n  baz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- # Foo\n- ## Bar\n  baz");
}
#[test]
fn test_example_260_md_prose_wrapalways_format_1_b5eb24b7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- foo\n- bar\n+ baz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- foo\n- bar\n\n* baz");
}
#[test]
fn test_example_261_md_prose_wrapalways_format_1_163116df() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1. foo\n2. bar\n3) baz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "1. foo\n2. bar\n\n3) baz");
}
#[test]
fn test_example_262_md_prose_wrapalways_format_1_345e50d2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("Foo\n- bar\n- baz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "Foo\n\n- bar\n- baz");
}
#[test]
fn test_example_263_md_prose_wrapalways_format_1_48ea86ff() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("The number of windows in my house is\n14.  The number of doors is 6.");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "The number of windows in my house is 14. The number of doors is 6."
    );
}
#[test]
fn test_example_264_md_prose_wrapalways_format_1_9779a752() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- foo\n\n- bar\n\n\n- baz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- foo\n\n- bar\n\n- baz");
}
#[test]
fn test_example_265_md_prose_wrapalways_format_1_223ff845() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- foo\n\n\n  bar\n- baz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- foo\n\n  bar\n\n- baz");
}
#[test]
fn test_example_266_md_prose_wrapalways_format_1_2e89af7f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- foo\n  - bar\n    - baz\n\n\n      bim");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- foo\n\n  - bar\n\n    - baz\n\n      bim");
}
#[test]
fn test_example_267_md_prose_wrapalways_format_1_ff674f4b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- foo\n- bar\n\n\n- baz\n- bim");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- foo\n- bar\n\n- baz\n- bim");
}
#[test]
fn test_example_268_md_prose_wrapalways_format_1_481edb82() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("-   foo\n\n    notcode\n\n-   foo\n\n\n    code");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- foo\n\n  notcode\n\n- foo\n\n  code");
}
#[test]
fn test_example_269_md_prose_wrapalways_format_1_37337bb4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("- a\n - b\n  - c\n   - d\n    - e\n   - f\n  - g\n - h\n- i");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- a\n- b\n- c\n- d\n- e\n- f\n- g\n- h\n- i");
}
#[test]
fn test_example_271_md_prose_wrapalways_format_1_37e92052() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- a\n- b\n\n- c");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- a\n- b\n\n- c");
}
#[test]
fn test_example_272_md_prose_wrapalways_format_1_0c3a5ccb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("* a\n*\n\n* c");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- a\n-\n\n- c");
}
#[test]
fn test_example_273_md_prose_wrapalways_format_1_59edc505() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- a\n- b\n\n  c\n- d");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- a\n- b\n\n  c\n\n- d");
}
#[test]
fn test_example_274_md_prose_wrapalways_format_1_04b3f7b1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- a\n- b\n\n  [ref]: /url\n- d");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- a\n- b\n\n  [ref]: /url\n\n- d");
}
#[test]
fn test_example_276_md_prose_wrapalways_format_1_ad42f942() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- a\n  - b\n\n    c\n- d");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- a\n\n  - b\n\n    c\n\n- d");
}
#[test]
fn test_example_277_md_prose_wrapalways_format_1_cad68741() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("* a\n  > b\n  >\n* c");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- a\n  > b\n- c");
}
#[test]
fn test_example_278_md_prose_wrapalways_format_1_cda9fba3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- a\n  > b\n  \\`\\`\\`\n  c\n  \\`\\`\\`\n- d");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- a\n  > b\n  \\`\\`\\`\n  c\n  \\`\\`\\`\n- d");
}
#[test]
fn test_example_279_md_prose_wrapalways_format_1_49afb8e4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- a");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- a");
}
#[test]
fn test_example_280_md_prose_wrapalways_format_1_e057d96b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- a\n  - b");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- a\n  - b");
}
#[test]
fn test_example_281_md_prose_wrapalways_format_1_cc9e6f95() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1. \\`\\`\\`\n   foo\n   \\`\\`\\`\n\n   bar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "1. \\`\\`\\`\n   foo\n   \\`\\`\\`\n\n   bar");
}
#[test]
fn test_example_282_md_prose_wrapalways_format_1_554804b5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("* foo\n  * bar\n\n  baz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- foo\n\n  - bar\n\n  baz");
}
#[test]
fn test_example_283_md_prose_wrapalways_format_1_e10a8a35() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- a\n  - b\n  - c\n\n- d\n  - e\n  - f");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "- a\n\n  - b\n  - c\n\n- d\n  - e\n  - f");
}
#[test]
fn test_example_284_md_prose_wrapalways_format_1_e5f26a49() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`hi\\`lo\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`hi\\`lo\\`");
}
#[test]
fn test_example_285_md_prose_wrapalways_format_1_7ad3b74c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\\\\!\\\\\"\\\\#\\\\$\\\\%\\\\&\\\\'\\\\(\\\\)\\\\*\\\\+\\\\,\\\\-\\\\.\\\\/\\\\:\\\\;\\\\<\\\\=\\\\>\\\\?\\\\@\\\\[\\\\\\\\\\\\]\\\\^\\\\_\\\\\\`\\\\{\\\\|\\\\}\\\\~") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "\\\\!\\\\\"\\\\#\\\\$\\\\%\\\\&\\\\'\\\\(\\\\)\\\\*\\\\+\\\\,\\\\-\\\\.\\\\/\\\\:\\\\;\\\\<\\\\=\\\\>\\\\?\\\\@\\\\[\\\\\\\\\\\\]\\\\^\\\\_\\\\\\`\\\\{\\\\|\\\\}\\\\~");
}
#[test]
fn test_example_286_md_prose_wrapalways_format_1_e04d1ad8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\\\\t\\\\A\\\\a\\\\ \\\\3\\\\φ\\\\«");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\\\ \\\\A\\\\a\\\\ \\\\3\\\\φ\\\\«");
}
#[test]
fn test_example_287_md_prose_wrapalways_format_1_d6f7657a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\\\\*not emphasized*\n\\\\<br/> not a tag\n\\\\[not a link](/foo)\n\\\\\\`not code\\`\n1\\\\. not a list\n\\\\* not a list\n\\\\# not a heading\n\\\\[foo]: /url \"not a reference\"") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "\\\\*not emphasized\\\\* \\\\<br/> not a tag \\\\[not a link](/foo) \\\\\\`not code\\` 1\\\\. not a\nlist \\\\* not a list \\\\# not a heading \\\\[foo]: /url \"not a reference\"");
}
#[test]
fn test_example_288_md_prose_wrapalways_format_1_7bbafe9b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\\\\\\\*emphasis*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\\\\\\\_emphasis_");
}
#[test]
fn test_example_289_md_prose_wrapalways_format_1_ba92dbce() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo\\\\\nbar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo\\\\\nbar");
}
#[test]
fn test_example_290_md_prose_wrapalways_format_1_589c2c2b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`\\` \\\\[\\\\\\` \\`\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`\\` \\\\[\\\\\\` \\`\\`");
}
#[test]
fn test_example_291_md_prose_wrapalways_format_1_163a3e14() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("    \\\\[\\\\]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "    \\\\[\\\\]");
}
#[test]
fn test_example_292_md_prose_wrapalways_format_1_fe6ca467() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("~~~\n\\\\[\\\\]\n~~~");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`\\`\\`\n\\\\[\\\\]\n\\`\\`\\`");
}
#[test]
fn test_example_293_md_prose_wrapalways_format_1_0f5d2c72() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<http://example.com?find=\\\\*>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<http://example.com?find=\\\\*>");
}
#[test]
fn test_example_294_md_prose_wrapalways_format_1_077874f0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<a href=\"/bar\\\\/)\">");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<a href=\"/bar\\\\/)\">");
}
#[test]
fn test_example_295_md_prose_wrapalways_format_1_370c7517() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo](/bar\\\\* \"ti\\\\*tle\")");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo](/bar* \"ti*tle\")");
}
#[test]
fn test_example_296_md_prose_wrapalways_format_1_f6eae3b3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo]\n\n[foo]: /bar\\\\* \"ti\\\\*tle\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo]\n\n[foo]: /bar* \"ti*tle\"");
}
#[test]
fn test_example_297_md_prose_wrapalways_format_1_cc7392ac() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`\\`\\` foo\\\\+bar\nfoo\n\\`\\`\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`\\`\\`foo+bar\nfoo\n\\`\\`\\`");
}
#[test]
fn test_example_298_md_prose_wrapalways_format_1_205a38e4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("&nbsp; &amp; &copy; &AElig; &Dcaron;\n&frac34; &HilbertSpace; &DifferentialD;\n&ClockwiseContourIntegral; &ngE;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "&nbsp; &amp; &copy; &AElig; &Dcaron; &frac34; &HilbertSpace; &DifferentialD;\n&ClockwiseContourIntegral; ≧\u{338}");
}
#[test]
fn test_example_299_md_prose_wrapalways_format_1_4f754138() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("&#35; &#1234; &#992; &#98765432; &#0;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "&#35; &#1234; &#992; &#98765432; &#0;");
}
#[test]
fn test_example_300_md_prose_wrapalways_format_1_ba959753() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("&#X22; &#XD06; &#xcab;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "&#X22; &#XD06; &#xcab;");
}
#[test]
fn test_example_301_md_prose_wrapalways_format_1_483fb56e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("&nbsp &x; &#; &#x;\n&ThisIsNotDefined; &hi?;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "&nbsp &x; &#; &#x; &ThisIsNotDefined; &hi?;");
}
#[test]
fn test_example_302_md_prose_wrapalways_format_1_d2bf5053() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("&copy");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "&copy");
}
#[test]
fn test_example_303_md_prose_wrapalways_format_1_6bfdabb6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("&MadeUpEntity;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "&MadeUpEntity;");
}
#[test]
fn test_example_304_md_prose_wrapalways_format_1_0955593a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<a href=\"&ouml;&ouml;.html\">");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<a href=\"&ouml;&ouml;.html\">");
}
#[test]
fn test_example_305_md_prose_wrapalways_format_1_8a720c79() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo](/f&ouml;&ouml; \"f&ouml;&ouml;\")");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo](/föö \"föö\")");
}
#[test]
fn test_example_306_md_prose_wrapalways_format_1_faf5126d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo]\n\n[foo]: /f&ouml;&ouml; \"f&ouml;&ouml;\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo]\n\n[foo]: /föö \"föö\"");
}
#[test]
fn test_example_307_md_prose_wrapalways_format_1_7b3d3e79() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`\\`\\` f&ouml;&ouml;\nfoo\n\\`\\`\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`\\`\\`föö\nfoo\n\\`\\`\\`");
}
#[test]
fn test_example_308_md_prose_wrapalways_format_1_64dfcf08() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`f&ouml;&ouml;\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`f&ouml;&ouml;\\`");
}
#[test]
fn test_example_309_md_prose_wrapalways_format_1_89796a9c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("    f&ouml;f&ouml;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "    f&ouml;f&ouml;");
}
#[test]
fn test_example_310_md_prose_wrapalways_format_1_63a4e7d9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`foo\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`foo\\`");
}
#[test]
fn test_example_311_md_prose_wrapalways_format_1_ecc0847b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`\\` foo \\` bar  \\`\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`\\`foo \\` bar \\`\\`");
}
#[test]
fn test_example_312_md_prose_wrapalways_format_1_2b165875() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\` \\`\\` \\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\` \\`\\` \\`");
}
#[test]
fn test_example_313_md_prose_wrapalways_format_1_bc7f6ddd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`\\`\nfoo\n\\`\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`foo\\`");
}
#[test]
fn test_example_314_md_prose_wrapalways_format_1_ac2931b4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`foo   bar\n  baz\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`foo   bar   baz\\`");
}
#[test]
fn test_example_315_md_prose_wrapalways_format_1_18f31b02() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`foo \\`\\` bar\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`foo \\`\\` bar\\`");
}
#[test]
fn test_example_316_md_prose_wrapalways_format_1_df0e53f9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`foo\\\\\\`bar\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`foo\\\\\\`bar\\`");
}
#[test]
fn test_example_317_md_prose_wrapalways_format_1_2c664857() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*foo\\`*\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_foo\\`_\\`");
}
#[test]
fn test_example_318_md_prose_wrapalways_format_1_7eaf8b83() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[not a \\`link](/foo\\`)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[not a \\`link](/foo\\`)");
}
#[test]
fn test_example_319_md_prose_wrapalways_format_1_7a916613() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`<a href=\"\\`\">\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`<a href=\"\\`\">\\`");
}
#[test]
fn test_example_320_md_prose_wrapalways_format_1_ac1f2ff3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<a href=\"\\`\">\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<a href=\"\\`\">\\`");
}
#[test]
fn test_example_321_md_prose_wrapalways_format_1_c3497c5b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`<http://foo.bar.\\`baz>\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`<http://foo.bar.\\`baz>\\`");
}
#[test]
fn test_example_322_md_prose_wrapalways_format_1_d9d8fd78() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<http://foo.bar.\\`baz>\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<http://foo.bar.\\`baz>\\`");
}
#[test]
fn test_example_324_md_prose_wrapalways_format_1_60ccf997() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`foo");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`foo");
}
#[test]
fn test_example_325_md_prose_wrapalways_format_1_bb76c0b7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*foo bar*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_foo bar_");
}
#[test]
fn test_example_326_md_prose_wrapalways_format_1_1bd9bb5a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a * foo bar*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "a _ foo bar_");
}
#[test]
fn test_example_327_md_prose_wrapalways_format_1_6061030f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a*\"foo\"*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "a*\"foo\"*");
}
#[test]
fn test_example_328_md_prose_wrapalways_format_1_0eb97625() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*\u{a0}a\u{a0}*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_\u{a0}a\u{a0}_");
}
#[test]
fn test_example_329_md_prose_wrapalways_format_1_34d6497f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo*bar*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo*bar*");
}
#[test]
fn test_example_330_md_prose_wrapalways_format_1_505f0502() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("5*6*78");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "5*6*78");
}
#[test]
fn test_example_331_md_prose_wrapalways_format_1_388afe3e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_foo bar_");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_foo bar_");
}
#[test]
fn test_example_332_md_prose_wrapalways_format_1_09e2fc9b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_ foo bar_");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_ foo bar_");
}
#[test]
fn test_example_333_md_prose_wrapalways_format_1_e9f8d123() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a_\"foo\"_");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "a*\"foo\"*");
}
#[test]
fn test_example_334_md_prose_wrapalways_format_1_80b20754() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo_bar_");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo*bar*");
}
#[test]
fn test_example_335_md_prose_wrapalways_format_1_3f39d393() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("5_6_78");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "5_6_78");
}
#[test]
fn test_example_336_md_prose_wrapalways_format_1_69ab6f61() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("пристаням_стремятся_");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "пристаням*стремятся*");
}
#[test]
fn test_example_337_md_prose_wrapalways_format_1_7ddbc040() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("aa_\"bb\"_cc");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "aa\\\\_\"bb\"\\\\_cc");
}
#[test]
fn test_example_338_md_prose_wrapalways_format_1_0a876cf6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo-_(bar)_");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo-_(bar)_");
}
#[test]
fn test_example_339_md_prose_wrapalways_format_1_254951b3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_foo*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\\\_foo\\\\*");
}
#[test]
fn test_example_340_md_prose_wrapalways_format_1_fe02f654() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*foo bar *");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_foo bar _");
}
#[test]
fn test_example_341_md_prose_wrapalways_format_1_9946f0c2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*foo bar\n*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\\\*foo bar\n\n-");
}
#[test]
fn test_example_342_md_prose_wrapalways_format_1_db5b593c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*(*foo)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "*(*foo)");
}
#[test]
fn test_example_343_md_prose_wrapalways_format_1_101e7798() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*(*foo*)*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "*(*foo*)*");
}
#[test]
fn test_example_344_md_prose_wrapalways_format_1_4db6005d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*foo*bar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "*foo*bar");
}
#[test]
fn test_example_345_md_prose_wrapalways_format_1_cb787ee2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_foo bar _");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_foo bar _");
}
#[test]
fn test_example_346_md_prose_wrapalways_format_1_62547942() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_(_foo)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\\\_(\\\\_foo)");
}
#[test]
fn test_example_347_md_prose_wrapalways_format_1_a0394b71() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_(_foo_)_");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_(\\\\_foo_)\\\\_");
}
#[test]
fn test_example_348_md_prose_wrapalways_format_1_f395844b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_foo_bar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\\\_foo_bar");
}
#[test]
fn test_example_349_md_prose_wrapalways_format_1_70f0438e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_пристаням_стремятся");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "*пристаням*стремятся");
}
#[test]
fn test_example_350_md_prose_wrapalways_format_1_291e422d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_foo_bar_baz_");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_foo_bar_baz_");
}
#[test]
fn test_example_351_md_prose_wrapalways_format_1_9d063c62() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_(bar)_.");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_(bar)_.");
}
#[test]
fn test_example_352_md_prose_wrapalways_format_1_cf08b55d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("**foo bar**");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**foo bar**");
}
#[test]
fn test_example_353_md_prose_wrapalways_format_1_c81b6787() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("** foo bar**");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "** foo bar**");
}
#[test]
fn test_example_354_md_prose_wrapalways_format_1_41814942() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a**\"foo\"**");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "a**\"foo\"**");
}
#[test]
fn test_example_355_md_prose_wrapalways_format_1_61d95b1b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo**bar**");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo**bar**");
}
#[test]
fn test_example_356_md_prose_wrapalways_format_1_9462c999() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("__foo bar__");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**foo bar**");
}
#[test]
fn test_example_357_md_prose_wrapalways_format_1_a1ea1122() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("__ foo bar__");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "** foo bar**");
}
#[test]
fn test_example_358_md_prose_wrapalways_format_1_1e7359d1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("__\nfoo bar__");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "** foo bar**");
}
#[test]
fn test_example_359_md_prose_wrapalways_format_1_62d06a00() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a__\"foo\"__");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "a**\"foo\"**");
}
#[test]
fn test_example_360_md_prose_wrapalways_format_1_6800ea25() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo__bar__");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo**bar**");
}
#[test]
fn test_example_361_md_prose_wrapalways_format_1_514aa203() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("5__6__78");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "5**6**78");
}
#[test]
fn test_example_362_md_prose_wrapalways_format_1_90a79bcb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("пристаням__стремятся__");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "пристаням**стремятся**");
}
#[test]
fn test_example_363_md_prose_wrapalways_format_1_0ab411c9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("__foo, __bar__, baz__");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**foo, **bar**, baz**");
}
#[test]
fn test_example_364_md_prose_wrapalways_format_1_b253e1ec() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo-__(bar)__");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo-**(bar)**");
}
#[test]
fn test_example_365_md_prose_wrapalways_format_1_32ad9a89() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("**foo bar **");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**foo bar **");
}
#[test]
fn test_example_366_md_prose_wrapalways_format_1_2e93caa6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("**(**foo)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**(**foo)");
}
#[test]
fn test_example_367_md_prose_wrapalways_format_1_f45e6efd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*(**foo**)*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_(**foo**)_");
}
#[test]
fn test_example_368_md_prose_wrapalways_format_1_c09f8c82() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("**Gomphocarpus (*Gomphocarpus physocarpus*, syn.\n*Asclepias physocarpa*)**");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "**Gomphocarpus (_Gomphocarpus physocarpus_, syn. _Asclepias physocarpa_)**"
    );
}
#[test]
fn test_example_369_md_prose_wrapalways_format_1_3541f34c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("**foo \"*bar*\" foo**");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**foo \"_bar_\" foo**");
}
#[test]
fn test_example_370_md_prose_wrapalways_format_1_c4e87660() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("**foo**bar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**foo**bar");
}
#[test]
fn test_example_371_md_prose_wrapalways_format_1_27482f17() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("__foo bar __");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**foo bar **");
}
#[test]
fn test_example_372_md_prose_wrapalways_format_1_015002c9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("__(__foo)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**(**foo)");
}
#[test]
fn test_example_373_md_prose_wrapalways_format_1_80fbdf37() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_(__foo__)_");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_(**foo**)_");
}
#[test]
fn test_example_374_md_prose_wrapalways_format_1_7311063a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("__foo__bar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**foo**bar");
}
#[test]
fn test_example_375_md_prose_wrapalways_format_1_8a15e7af() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("__пристаням__стремятся");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**пристаням**стремятся");
}
#[test]
fn test_example_376_md_prose_wrapalways_format_1_7a717895() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("__foo__bar__baz__");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**foo**bar**baz**");
}
#[test]
fn test_example_377_md_prose_wrapalways_format_1_6bca7aa1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("__(bar)__.");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**(bar)**.");
}
#[test]
fn test_example_378_md_prose_wrapalways_format_1_928ea6bb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*foo [bar](/url)*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_foo [bar](/url)_");
}
#[test]
fn test_example_379_md_prose_wrapalways_format_1_c339d619() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*foo\nbar*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_foo bar_");
}
#[test]
fn test_example_380_md_prose_wrapalways_format_1_57470b44() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_foo __bar__ baz_");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_foo **bar** baz_");
}
#[test]
fn test_example_381_md_prose_wrapalways_format_1_0c79072d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_foo _bar_ baz_");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_foo \\\\_bar_ baz\\\\_");
}
#[test]
fn test_example_382_md_prose_wrapalways_format_1_8c3bad77() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("__foo_ bar_");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\\\__foo_ bar\\\\_");
}
#[test]
fn test_example_383_md_prose_wrapalways_format_1_ce5ef326() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*foo *bar**");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "*foo *bar\\\\*\\\\*");
}
#[test]
fn test_example_384_md_prose_wrapalways_format_1_2f445809() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*foo **bar** baz*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_foo **bar** baz_");
}
#[test]
fn test_example_385_md_prose_wrapalways_format_1_e86ce213() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*foo**bar**baz*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_foo**bar**baz_");
}
#[test]
fn test_example_386_md_prose_wrapalways_format_1_48a7eeb2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("***foo** bar*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**\\\\*foo** bar\\\\*");
}
#[test]
fn test_example_387_md_prose_wrapalways_format_1_dd604838() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*foo **bar***");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\\\*foo **bar\\\\***");
}
#[test]
fn test_example_388_md_prose_wrapalways_format_1_c584ffc3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*foo**bar***");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\\\*foo**bar\\\\***");
}
#[test]
fn test_example_390_md_prose_wrapalways_format_1_acd7852c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*foo [*bar*](/url)*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "*foo [*bar*](/url)*");
}
#[test]
fn test_example_391_md_prose_wrapalways_format_1_02330bb7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("** is not an empty emphasis");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\\\*\\\\* is not an empty emphasis");
}
#[test]
fn test_example_392_md_prose_wrapalways_format_1_0e03fac8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("**** is not an empty strong emphasis");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "\\\\*\\\\*\\\\*\\\\* is not an empty strong emphasis"
    );
}
#[test]
fn test_example_393_md_prose_wrapalways_format_1_f9650c6e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("**foo [bar](/url)**");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**foo [bar](/url)**");
}
#[test]
fn test_example_394_md_prose_wrapalways_format_1_43cb7ced() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("**foo\nbar**");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**foo bar**");
}
#[test]
fn test_example_395_md_prose_wrapalways_format_1_32b2d944() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("__foo _bar_ baz__");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**foo _bar_ baz**");
}
#[test]
fn test_example_396_md_prose_wrapalways_format_1_1fa80f28() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("__foo __bar__ baz__");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**foo **bar** baz**");
}
#[test]
fn test_example_397_md_prose_wrapalways_format_1_b315d53a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("____foo__ bar__");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\\\_**\\\\_foo** bar\\\\_\\\\_");
}
#[test]
fn test_example_398_md_prose_wrapalways_format_1_eb775d60() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("**foo **bar****");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**foo **bar\\\\*\\\\*\\\\*\\\\*");
}
#[test]
fn test_example_399_md_prose_wrapalways_format_1_781888c7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("**foo *bar* baz**");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**foo _bar_ baz**");
}
#[test]
fn test_example_400_md_prose_wrapalways_format_1_16c50860() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("**foo*bar*baz**");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**foo*bar*baz**");
}
#[test]
fn test_example_401_md_prose_wrapalways_format_1_43f03043() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("***foo* bar**");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**_foo_ bar**");
}
#[test]
fn test_example_402_md_prose_wrapalways_format_1_0df165a0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("**foo *bar***");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**foo _bar_**");
}
#[test]
fn test_example_403_md_prose_wrapalways_format_1_cd99a74b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("**foo *bar **baz**\nbim* bop**");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**foo \\\\*bar **baz** bim\\\\* bop**");
}
#[test]
fn test_example_404_md_prose_wrapalways_format_1_ec10d47e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("**foo [*bar*](/url)**");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**foo [_bar_](/url)**");
}
#[test]
fn test_example_405_md_prose_wrapalways_format_1_846c954a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("__ is not an empty emphasis");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\\\_\\\\_ is not an empty emphasis");
}
#[test]
fn test_example_406_md_prose_wrapalways_format_1_41b7deae() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("____ is not an empty strong emphasis");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "\\\\_\\\\_\\\\_\\\\_ is not an empty strong emphasis"
    );
}
#[test]
fn test_example_407_md_prose_wrapalways_format_1_c160213a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo ***");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo \\\\*\\\\*\\\\*");
}
#[test]
fn test_example_408_md_prose_wrapalways_format_1_d343a51c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo *\\\\**");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo \\\\*\\\\*\\\\*");
}
#[test]
fn test_example_410_md_prose_wrapalways_format_1_86332235() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo *****");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo **\\\\***");
}
#[test]
fn test_example_411_md_prose_wrapalways_format_1_c169d320() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo **\\\\***");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo **\\\\***");
}
#[test]
fn test_example_412_md_prose_wrapalways_format_1_9b63ffad() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo **_**");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo **\\\\_**");
}
#[test]
fn test_example_413_md_prose_wrapalways_format_1_13b0bdc8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("**foo*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\\\*_foo_");
}
#[test]
fn test_example_414_md_prose_wrapalways_format_1_4ac8262a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*foo**");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\\\*foo\\\\*\\\\*");
}
#[test]
fn test_example_415_md_prose_wrapalways_format_1_49326a2f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("***foo**");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**\\\\*foo**");
}
#[test]
fn test_example_417_md_prose_wrapalways_format_1_d21dc790() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("**foo***");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**foo\\\\***");
}
#[test]
fn test_example_418_md_prose_wrapalways_format_1_4318bf8a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*foo****");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\\\*foo\\\\*\\\\*\\\\*\\\\*");
}
#[test]
fn test_example_419_md_prose_wrapalways_format_1_53bb15e1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo ___");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo \\\\_\\\\_\\\\_");
}
#[test]
fn test_example_420_md_prose_wrapalways_format_1_d2f855a1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo _\\\\__");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo \\\\_\\\\_\\\\_");
}
#[test]
fn test_example_421_md_prose_wrapalways_format_1_baba90a0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo _*_");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo _\\\\*_");
}
#[test]
fn test_example_422_md_prose_wrapalways_format_1_a11aa384() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo _____");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo **\\\\_**");
}
#[test]
fn test_example_423_md_prose_wrapalways_format_1_f49d6338() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo __\\\\___");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo **\\\\_**");
}
#[test]
fn test_example_424_md_prose_wrapalways_format_1_fcddf6df() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo __*__");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo **\\\\***");
}
#[test]
fn test_example_425_md_prose_wrapalways_format_1_9c138e66() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("__foo_");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\\\__foo_");
}
#[test]
fn test_example_426_md_prose_wrapalways_format_1_071f67fa() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_foo__");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\\\_foo\\\\_\\\\_");
}
#[test]
fn test_example_427_md_prose_wrapalways_format_1_b1407bd4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("___foo__");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**\\\\_foo**");
}
#[test]
fn test_example_428_md_prose_wrapalways_format_1_7e25d28a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("____foo_");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\\\__\\\\_\\\\_foo_");
}
#[test]
fn test_example_429_md_prose_wrapalways_format_1_4ee66bcf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("__foo___");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**foo\\\\_**");
}
#[test]
fn test_example_430_md_prose_wrapalways_format_1_ac34c57a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_foo____");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\\\_foo\\\\_\\\\_\\\\_\\\\_");
}
#[test]
fn test_example_431_md_prose_wrapalways_format_1_880c9c3e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("**foo**");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**foo**");
}
#[test]
fn test_example_432_md_prose_wrapalways_format_1_6d70b16c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*_foo_*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_*foo*_");
}
#[test]
fn test_example_433_md_prose_wrapalways_format_1_5f97f64b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("__foo__");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**foo**");
}
#[test]
fn test_example_434_md_prose_wrapalways_format_1_6c5c2986() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_*foo*_");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_*foo*_");
}
#[test]
fn test_example_435_md_prose_wrapalways_format_1_1d394616() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("****foo****");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\\\***\\\\*foo\\\\*\\\\***");
}
#[test]
fn test_example_436_md_prose_wrapalways_format_1_ae628fe6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("____foo____");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\\\_**\\\\_foo\\\\_\\\\_**");
}
#[test]
fn test_example_437_md_prose_wrapalways_format_1_ff5567e1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("******foo******");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**\\\\*\\\\***foo**\\\\*\\\\***");
}
#[test]
fn test_example_438_md_prose_wrapalways_format_1_2af1b646() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("***foo***");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**_foo_**");
}
#[test]
fn test_example_439_md_prose_wrapalways_format_1_f12ef610() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_____foo_____");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**\\\\_**foo**\\\\_**");
}
#[test]
fn test_example_440_md_prose_wrapalways_format_1_84283087() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*foo _bar* baz_");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_foo \\\\_bar_ baz\\\\_");
}
#[test]
fn test_example_441_md_prose_wrapalways_format_1_ac46b3cb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("**foo*bar**");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**foo\\\\*bar**");
}
#[test]
fn test_example_442_md_prose_wrapalways_format_1_180d65c7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*foo __bar *baz bim__ bam*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "*foo \\\\_\\\\_bar *baz bim\\\\_\\\\_ bam\\\\*");
}
#[test]
fn test_example_443_md_prose_wrapalways_format_1_89a48b68() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("**foo **bar baz**");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**foo **bar baz\\\\*\\\\*");
}
#[test]
fn test_example_444_md_prose_wrapalways_format_1_86f44837() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*foo *bar baz*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "*foo *bar baz\\\\*");
}
#[test]
fn test_example_445_md_prose_wrapalways_format_1_db0dee5e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*[bar*](/url)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_[bar_](/url)");
}
#[test]
fn test_example_446_md_prose_wrapalways_format_1_305e46c4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_foo [bar_](/url)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_foo [bar_](/url)");
}
#[test]
fn test_example_447_md_prose_wrapalways_format_1_9d20f270() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*<img src=\"foo\" title=\"*\"/>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_<img src=\"foo\" title=\"_\"/>");
}
#[test]
fn test_example_448_md_prose_wrapalways_format_1_cbe70e4c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("**<a href=\"**\">");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**<a href=\"**\">");
}
#[test]
fn test_example_449_md_prose_wrapalways_format_1_b3756390() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("__<a href=\"__\">");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**<a href=\"**\">");
}
#[test]
fn test_example_450_md_prose_wrapalways_format_1_9aa68977() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*a \\`*\\`*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_a \\`_\\`\\\\*");
}
#[test]
fn test_example_451_md_prose_wrapalways_format_1_9ce23543() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("_a \\`_\\`_");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_a \\`_\\`\\\\_");
}
#[test]
fn test_example_452_md_prose_wrapalways_format_1_2522e15b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("**a<http://foo.bar/?q=**>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**a<http://foo.bar/?q=**>");
}
#[test]
fn test_example_453_md_prose_wrapalways_format_1_a712711c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("__a<http://foo.bar/?q=__>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "**a<http://foo.bar/?q=**>");
}
#[test]
fn test_example_454_md_prose_wrapalways_format_1_5c0d8372() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[link](/uri \"title\")");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[link](/uri \"title\")");
}
#[test]
fn test_example_455_md_prose_wrapalways_format_1_557d5597() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[link](/uri)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[link](/uri)");
}
#[test]
fn test_example_456_md_prose_wrapalways_format_1_d8510b2c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[link]()");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[link]()");
}
#[test]
fn test_example_457_md_prose_wrapalways_format_1_38b5e765() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[link](<>)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[link]()");
}
#[test]
fn test_example_458_md_prose_wrapalways_format_1_725d5309() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[link](/my uri)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[link](/my uri)");
}
#[test]
fn test_example_459_md_prose_wrapalways_format_1_edec73c7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[link](</my uri>)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[link](</my uri>)");
}
#[test]
fn test_example_460_md_prose_wrapalways_format_1_1c9cb15d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[link](foo\nbar)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[link](foo bar)");
}
#[test]
fn test_example_461_md_prose_wrapalways_format_1_c7b7fb38() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[link](<foo\nbar>)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[link](<foo\nbar>)");
}
#[test]
fn test_example_462_md_prose_wrapalways_format_1_a6a6a876() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[link](\\\\(foo\\\\))");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[link](<(foo)>)");
}
#[test]
fn test_example_463_md_prose_wrapalways_format_1_43ba7e0e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[link]((foo)and(bar))");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[link](<(foo)and(bar)>)");
}
#[test]
fn test_example_464_md_prose_wrapalways_format_1_21fddabf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[link](foo(and(bar)))");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[link](<foo(and(bar))>)");
}
#[test]
fn test_example_465_md_prose_wrapalways_format_1_e79cf989() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[link](foo(and\\\\(bar\\\\)))");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[link](<foo(and(bar))>)");
}
#[test]
fn test_example_466_md_prose_wrapalways_format_1_e647d3d8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[link](<foo(and(bar))>)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[link](<foo(and(bar))>)");
}
#[test]
fn test_example_467_md_prose_wrapalways_format_1_25335ec2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[link](foo\\\\)\\\\:)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[link](<foo):>)");
}
#[test]
fn test_example_468_md_prose_wrapalways_format_1_66568c90() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[link](#fragment)\n\n[link](http://example.com#fragment)\n\n[link](http://example.com?foo=3#frag)") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[link](#fragment)\n\n[link](http://example.com#fragment)\n\n[link](http://example.com?foo=3#frag)");
}
#[test]
fn test_example_469_md_prose_wrapalways_format_1_e6e6d268() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[link](foo\\\\bar)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[link](foo\\\\bar)");
}
#[test]
fn test_example_470_md_prose_wrapalways_format_1_f3b9cf49() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[link](foo%20b&auml;)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[link](foo%20bä)");
}
#[test]
fn test_example_471_md_prose_wrapalways_format_1_2457c38b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[link](\"title\")");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[link](\"title\")");
}
#[test]
fn test_example_472_md_prose_wrapalways_format_1_27fe4b0e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("[link](/url \"title\")\n[link](/url 'title')\n[link](/url (title))");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "[link](/url \"title\") [link](/url \"title\") [link](/url \"title\")"
    );
}
#[test]
fn test_example_473_md_prose_wrapalways_format_1_80eaec33() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[link](/url \"title \\\\\"&quot;\")");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[link](/url 'title \"\"')");
}
#[test]
fn test_example_474_md_prose_wrapalways_format_1_059f924f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[link](/url \"title \"and\" title\")");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[link](/url \"title \"and\" title\")");
}
#[test]
fn test_example_475_md_prose_wrapalways_format_1_bae11a74() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[link](/url 'title \"and\" title')");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[link](/url 'title \"and\" title')");
}
#[test]
fn test_example_476_md_prose_wrapalways_format_1_02e2d1d0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[link](   /uri\n  \"title\"  )");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[link](/uri \"title\")");
}
#[test]
fn test_example_477_md_prose_wrapalways_format_1_aab09571() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[link] (/uri)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[link] (/uri)");
}
#[test]
fn test_example_478_md_prose_wrapalways_format_1_b0ef3eb2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[link [foo [bar]]](/uri)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[link [foo [bar]]](/uri)");
}
#[test]
fn test_example_479_md_prose_wrapalways_format_1_2ebb5947() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[link] bar](/uri)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[link] bar](/uri)");
}
#[test]
fn test_example_480_md_prose_wrapalways_format_1_941e0772() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[link [bar](/uri)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[link [bar](/uri)");
}
#[test]
fn test_example_481_md_prose_wrapalways_format_1_9c0a05e5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[link \\\\[bar](/uri)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[link \\\\[bar](/uri)");
}
#[test]
fn test_example_482_md_prose_wrapalways_format_1_156d57e3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[link *foo **bar** \\`#\\`*](/uri)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[link _foo **bar** \\`#\\`_](/uri)");
}
#[test]
fn test_example_483_md_prose_wrapalways_format_1_4cb6459d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[![moon](moon.jpg)](/uri)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[![moon](moon.jpg)](/uri)");
}
#[test]
fn test_example_484_md_prose_wrapalways_format_1_469a115c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo [bar](/uri)](/uri)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo [bar](/uri)](/uri)");
}
#[test]
fn test_example_485_md_prose_wrapalways_format_1_868c0f96() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo *[bar [baz](/uri)](/uri)*](/uri)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo _[bar [baz](/uri)](/uri)_](/uri)");
}
#[test]
fn test_example_486_md_prose_wrapalways_format_1_d37950d3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("![[[foo](uri1)](uri2)](uri3)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "![[[foo](uri1)](uri2)](uri3)");
}
#[test]
fn test_example_487_md_prose_wrapalways_format_1_41f51741() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*[foo*](/uri)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_[foo_](/uri)");
}
#[test]
fn test_example_488_md_prose_wrapalways_format_1_2fac866f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo *bar](baz*)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo \\\\*bar](baz*)");
}
#[test]
fn test_example_489_md_prose_wrapalways_format_1_71da007a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*foo [bar* baz]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_foo [bar_ baz]");
}
#[test]
fn test_example_490_md_prose_wrapalways_format_1_2e8490dd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo <bar attr=\"](baz)\">");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo <bar attr=\"](baz)\">");
}
#[test]
fn test_example_491_md_prose_wrapalways_format_1_d865055c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo\\`](/uri)\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo\\`](/uri)\\`");
}
#[test]
fn test_example_492_md_prose_wrapalways_format_1_9fef3838() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo<http://example.com/?search=](uri)>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo<http://example.com/?search=](uri)>");
}
#[test]
fn test_example_493_md_prose_wrapalways_format_1_9726a0f1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo][bar]\n\n[bar]: /url \"title\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo][bar]\n\n[bar]: /url \"title\"");
}
#[test]
fn test_example_494_md_prose_wrapalways_format_1_9a4db51d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[link [foo [bar]]][ref]\n\n[ref]: /uri");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[link [foo [bar]]][ref]\n\n[ref]: /uri");
}
#[test]
fn test_example_495_md_prose_wrapalways_format_1_ea448480() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[link \\\\[bar][ref]\n\n[ref]: /uri");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[link \\\\[bar][ref]\n\n[ref]: /uri");
}
#[test]
fn test_example_496_md_prose_wrapalways_format_1_16b4b490() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[link *foo **bar** \\`#\\`*][ref]\n\n[ref]: /uri");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "[link _foo **bar** \\`#\\`_][ref]\n\n[ref]: /uri"
    );
}
#[test]
fn test_example_497_md_prose_wrapalways_format_1_1225d1fc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[![moon](moon.jpg)][ref]\n\n[ref]: /uri");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[![moon](moon.jpg)][ref]\n\n[ref]: /uri");
}
#[test]
fn test_example_498_md_prose_wrapalways_format_1_3defde95() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo [bar](/uri)][ref]\n\n[ref]: /uri");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo [bar](/uri)][ref]\n\n[ref]: /uri");
}
#[test]
fn test_example_499_md_prose_wrapalways_format_1_880e782f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo *bar [baz][ref]*][ref]\n\n[ref]: /uri");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo _bar [baz][ref]_][ref]\n\n[ref]: /uri");
}
#[test]
fn test_example_500_md_prose_wrapalways_format_1_8a890422() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*[foo*][ref]\n\n[ref]: /uri");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_[foo_][ref]\n\n[ref]: /uri");
}
#[test]
fn test_example_501_md_prose_wrapalways_format_1_148c1736() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo *bar][ref]\n\n[ref]: /uri");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo \\\\*bar][ref]\n\n[ref]: /uri");
}
#[test]
fn test_example_502_md_prose_wrapalways_format_1_6efec0ea() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo <bar attr=\"][ref]\">\n\n[ref]: /uri");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo <bar attr=\"][ref]\">\n\n[ref]: /uri");
}
#[test]
fn test_example_503_md_prose_wrapalways_format_1_d59849d2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo\\`][ref]\\`\n\n[ref]: /uri");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo\\`][ref]\\`\n\n[ref]: /uri");
}
#[test]
fn test_example_504_md_prose_wrapalways_format_1_5a09cf61() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo<http://example.com/?search=][ref]>\n\n[ref]: /uri");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "[foo<http://example.com/?search=][ref]>\n\n[ref]: /uri"
    );
}
#[test]
fn test_example_505_md_prose_wrapalways_format_1_3117696f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo][BaR]\n\n[bar]: /url \"title\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo][BaR]\n\n[bar]: /url \"title\"");
}
#[test]
fn test_example_506_md_prose_wrapalways_format_1_102e4445() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[Толпой][Толпой] is a Russian word.\n\n[ТОЛПОЙ]: /url");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "[Толпой][Толпой] is a Russian word.\n\n[ТОЛПОЙ]: /url"
    );
}
#[test]
fn test_example_507_md_prose_wrapalways_format_1_b8af7524() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[Foo\n  bar]: /url\n\n[Baz][Foo bar]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[Foo bar]: /url\n\n[Baz][Foo bar]");
}
#[test]
fn test_example_508_md_prose_wrapalways_format_1_13de9aed() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo] [bar]\n\n[bar]: /url \"title\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo] [bar]\n\n[bar]: /url \"title\"");
}
#[test]
fn test_example_509_md_prose_wrapalways_format_1_8759469a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo]\n[bar]\n\n[bar]: /url \"title\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo] [bar]\n\n[bar]: /url \"title\"");
}
#[test]
fn test_example_510_md_prose_wrapalways_format_1_4668d004() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo]: /url1\n\n[foo]: /url2\n\n[bar][foo]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo]: /url1\n[foo]: /url2\n\n[bar][foo]");
}
#[test]
fn test_example_511_md_prose_wrapalways_format_1_532e16d6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[bar][foo\\\\!]\n\n[foo!]: /url");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[bar][foo\\\\!]\n\n[foo!]: /url");
}
#[test]
fn test_example_512_md_prose_wrapalways_format_1_ca6d9093() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo][ref[]\n\n[ref[]: /uri");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo]ref[]\n\n[ref[]: /uri");
}
#[test]
fn test_example_513_md_prose_wrapalways_format_1_2d16742b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo][ref[bar]]\n\n[ref[bar]]: /uri");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo]ref[bar]]\n\n[ref[bar]]: /uri");
}
#[test]
fn test_example_514_md_prose_wrapalways_format_1_87efc012() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[[[foo]]]\n\n[[[foo]]]: /url");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[[[foo]]]\n\n[[[foo]]]: /url");
}
#[test]
fn test_example_515_md_prose_wrapalways_format_1_a3639bb1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo][ref\\\\[]\n\n[ref\\\\[]: /uri");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo][ref\\\\[]\n\n[ref\\\\[]: /uri");
}
#[test]
fn test_example_516_md_prose_wrapalways_format_1_c7cc3f0e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[bar\\\\\\\\]: /uri\n\n[bar\\\\\\\\]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[bar\\\\\\\\]: /uri\n\n[bar\\\\\\\\]");
}
#[test]
fn test_example_517_md_prose_wrapalways_format_1_967ee3ac() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[]\n\n[]: /uri");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[]\n\n[]: /uri");
}
#[test]
fn test_example_518_md_prose_wrapalways_format_1_a86c5fe0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[\n ]\n\n[\n ]: /uri");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[ ]\n\n[ ]: /uri");
}
#[test]
fn test_example_519_md_prose_wrapalways_format_1_c7333121() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo][]\n\n[foo]: /url \"title\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo][]\n\n[foo]: /url \"title\"");
}
#[test]
fn test_example_520_md_prose_wrapalways_format_1_257be9dd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[*foo* bar][]\n\n[*foo* bar]: /url \"title\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[*foo* bar][]\n\n[*foo* bar]: /url \"title\"");
}
#[test]
fn test_example_521_md_prose_wrapalways_format_1_a62d3275() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[Foo][]\n\n[foo]: /url \"title\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[Foo][]\n\n[foo]: /url \"title\"");
}
#[test]
fn test_example_522_md_prose_wrapalways_format_1_76758922() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo] \n[]\n\n[foo]: /url \"title\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo] []\n\n[foo]: /url \"title\"");
}
#[test]
fn test_example_523_md_prose_wrapalways_format_1_a9e8faa8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo]\n\n[foo]: /url \"title\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo]\n\n[foo]: /url \"title\"");
}
#[test]
fn test_example_524_md_prose_wrapalways_format_1_2fc5e76a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[*foo* bar]\n\n[*foo* bar]: /url \"title\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[*foo* bar]\n\n[*foo* bar]: /url \"title\"");
}
#[test]
fn test_example_525_md_prose_wrapalways_format_1_409b12c4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[[*foo* bar]]\n\n[*foo* bar]: /url \"title\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[[*foo* bar]]\n\n[*foo* bar]: /url \"title\"");
}
#[test]
fn test_example_526_md_prose_wrapalways_format_1_8cf82231() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[[bar [foo]\n\n[foo]: /url");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[[bar [foo]\n\n[foo]: /url");
}
#[test]
fn test_example_527_md_prose_wrapalways_format_1_5e17d63c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[Foo]\n\n[foo]: /url \"title\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[Foo]\n\n[foo]: /url \"title\"");
}
#[test]
fn test_example_528_md_prose_wrapalways_format_1_ddc28f12() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo] bar\n\n[foo]: /url");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo] bar\n\n[foo]: /url");
}
#[test]
fn test_example_529_md_prose_wrapalways_format_1_7a83dc15() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\\\[foo]\n\n[foo]: /url \"title\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\\\[foo]\n\n[foo]: /url \"title\"");
}
#[test]
fn test_example_530_md_prose_wrapalways_format_1_4905b106() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo*]: /url\n\n*[foo*]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo*]: /url\n\n_[foo_]");
}
#[test]
fn test_example_531_md_prose_wrapalways_format_1_46237e8f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo][bar]\n\n[foo]: /url1\n[bar]: /url2");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo][bar]\n\n[foo]: /url1\n[bar]: /url2");
}
#[test]
fn test_example_532_md_prose_wrapalways_format_1_81b29ff6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo][bar][baz]\n\n[baz]: /url");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo][bar][baz]\n\n[baz]: /url");
}
#[test]
fn test_example_533_md_prose_wrapalways_format_1_e58987f9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo][bar][baz]\n\n[baz]: /url1\n[bar]: /url2");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo][bar][baz]\n\n[baz]: /url1\n[bar]: /url2");
}
#[test]
fn test_example_534_md_prose_wrapalways_format_1_0e32b66b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo][bar][baz]\n\n[baz]: /url1\n[foo]: /url2");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[foo][bar][baz]\n\n[baz]: /url1\n[foo]: /url2");
}
#[test]
fn test_example_535_md_prose_wrapalways_format_1_13e462c1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("![foo](/url \"title\")");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "![foo](/url \"title\")");
}
#[test]
fn test_example_536_md_prose_wrapalways_format_1_a10351c1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("![foo *bar*]\n\n[foo *bar*]: train.jpg \"train & tracks\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "![foo *bar*]\n\n[foo *bar*]: train.jpg \"train & tracks\""
    );
}
#[test]
fn test_example_537_md_prose_wrapalways_format_1_88151b95() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("![foo ![bar](/url)](/url2)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "![foo ![bar](/url)](/url2)");
}
#[test]
fn test_example_538_md_prose_wrapalways_format_1_f6e1623b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("![foo [bar](/url)](/url2)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "![foo [bar](/url)](/url2)");
}
#[test]
fn test_example_539_md_prose_wrapalways_format_1_d2cd30ba() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("![foo *bar*][]\n\n[foo *bar*]: train.jpg \"train & tracks\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "![foo *bar*][]\n\n[foo *bar*]: train.jpg \"train & tracks\""
    );
}
#[test]
fn test_example_540_md_prose_wrapalways_format_1_37e47977() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("![foo *bar*][foobar]\n\n[FOOBAR]: train.jpg \"train & tracks\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "![foo *bar*][foobar]\n\n[FOOBAR]: train.jpg \"train & tracks\""
    );
}
#[test]
fn test_example_541_md_prose_wrapalways_format_1_98bb67bb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("![foo](train.jpg)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "![foo](train.jpg)");
}
#[test]
fn test_example_542_md_prose_wrapalways_format_1_4f9c8014() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("My ![foo bar](/path/to/train.jpg  \"title\"   )");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "My ![foo bar](/path/to/train.jpg \"title\")");
}
#[test]
fn test_example_543_md_prose_wrapalways_format_1_c42fe64c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("![foo](<url>)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "![foo](url)");
}
#[test]
fn test_example_544_md_prose_wrapalways_format_1_73cb9be1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("![](/url)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "![](/url)");
}
#[test]
fn test_example_545_md_prose_wrapalways_format_1_b1ba6215() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("![foo][bar]\n\n[bar]: /url");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "![foo][bar]\n\n[bar]: /url");
}
#[test]
fn test_example_546_md_prose_wrapalways_format_1_bef2c415() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("![foo][bar]\n\n[BAR]: /url");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "![foo][bar]\n\n[BAR]: /url");
}
#[test]
fn test_example_547_md_prose_wrapalways_format_1_e7ffd2eb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("![foo][]\n\n[foo]: /url \"title\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "![foo][]\n\n[foo]: /url \"title\"");
}
#[test]
fn test_example_548_md_prose_wrapalways_format_1_53406bd7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("![*foo* bar][]\n\n[*foo* bar]: /url \"title\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "![*foo* bar][]\n\n[*foo* bar]: /url \"title\"");
}
#[test]
fn test_example_549_md_prose_wrapalways_format_1_029f3afe() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("![Foo][]\n\n[foo]: /url \"title\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "![Foo][]\n\n[foo]: /url \"title\"");
}
#[test]
fn test_example_550_md_prose_wrapalways_format_1_c3ba35ca() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("![foo] \n[]\n\n[foo]: /url \"title\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "![foo] []\n\n[foo]: /url \"title\"");
}
#[test]
fn test_example_551_md_prose_wrapalways_format_1_4241d567() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("![foo]\n\n[foo]: /url \"title\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "![foo]\n\n[foo]: /url \"title\"");
}
#[test]
fn test_example_552_md_prose_wrapalways_format_1_a5f873b9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("![*foo* bar]\n\n[*foo* bar]: /url \"title\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "![*foo* bar]\n\n[*foo* bar]: /url \"title\"");
}
#[test]
fn test_example_553_md_prose_wrapalways_format_1_112b8606() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("![[foo]]\n\n[[foo]]: /url \"title\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "![[foo]]\n\n[[foo]]: /url \"title\"");
}
#[test]
fn test_example_554_md_prose_wrapalways_format_1_9855495e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("![Foo]\n\n[foo]: /url \"title\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "![Foo]\n\n[foo]: /url \"title\"");
}
#[test]
fn test_example_555_md_prose_wrapalways_format_1_0e1004a8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\\\!\\\\[foo]\n\n[foo]: /url \"title\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\\\!\\\\[foo]\n\n[foo]: /url \"title\"");
}
#[test]
fn test_example_556_md_prose_wrapalways_format_1_cb15089e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\\\![foo]\n\n[foo]: /url \"title\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\\\![foo]\n\n[foo]: /url \"title\"");
}
#[test]
fn test_example_557_md_prose_wrapalways_format_1_51be872e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<http://foo.bar.baz>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<http://foo.bar.baz>");
}
#[test]
fn test_example_558_md_prose_wrapalways_format_1_86dc15eb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<http://foo.bar.baz/test?q=hello&id=22&boolean>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<http://foo.bar.baz/test?q=hello&id=22&boolean>");
}
#[test]
fn test_example_559_md_prose_wrapalways_format_1_68316275() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<irc://foo.bar:2233/baz>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<irc://foo.bar:2233/baz>");
}
#[test]
fn test_example_560_md_prose_wrapalways_format_1_ab655ad8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<MAILTO:FOO@BAR.BAZ>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<MAILTO:FOO@BAR.BAZ>");
}
#[test]
fn test_example_561_md_prose_wrapalways_format_1_9e440c52() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<a+b+c:d>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<a+b+c:d>");
}
#[test]
fn test_example_562_md_prose_wrapalways_format_1_02ffcf5a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<made-up-scheme://foo,bar>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<made-up-scheme://foo,bar>");
}
#[test]
fn test_example_563_md_prose_wrapalways_format_1_f2ef15e1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<http://../>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<http://../>");
}
#[test]
fn test_example_564_md_prose_wrapalways_format_1_c4f495ab() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<localhost:5001/foo>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<localhost:5001/foo>");
}
#[test]
fn test_example_565_md_prose_wrapalways_format_1_35298842() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<http://foo.bar/baz bim>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<http://foo.bar/baz bim>");
}
#[test]
fn test_example_566_md_prose_wrapalways_format_1_b66c1a5a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<http://example.com/\\\\[\\\\>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<http://example.com/\\\\[\\\\>");
}
#[test]
fn test_example_567_md_prose_wrapalways_format_1_3243da52() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<foo@bar.example.com>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<foo@bar.example.com>");
}
#[test]
fn test_example_568_md_prose_wrapalways_format_1_58beaf7a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<foo+special@Bar.baz-bar0.com>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<foo+special@Bar.baz-bar0.com>");
}
#[test]
fn test_example_569_md_prose_wrapalways_format_1_dbc9dd0e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<foo\\\\+@bar.example.com>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<foo\\\\+@bar.example.com>");
}
#[test]
fn test_example_570_md_prose_wrapalways_format_1_4baba3be() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<>");
}
#[test]
fn test_example_571_md_prose_wrapalways_format_1_c6f39887() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("< http://foo.bar >");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "< http://foo.bar >");
}
#[test]
fn test_example_572_md_prose_wrapalways_format_1_30af2663() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<m:abc>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<m:abc>");
}
#[test]
fn test_example_573_md_prose_wrapalways_format_1_3fd5b56e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<foo.bar.baz>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<foo.bar.baz>");
}
#[test]
fn test_example_574_md_prose_wrapalways_format_1_f43e2e83() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("http://example.com");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "http://example.com");
}
#[test]
fn test_example_575_md_prose_wrapalways_format_1_623a0657() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo@bar.example.com");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo@bar.example.com");
}
#[test]
fn test_example_576_md_prose_wrapalways_format_1_a232c84e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<a><bab><c2c>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<a><bab><c2c>");
}
#[test]
fn test_example_577_md_prose_wrapalways_format_1_4f004699() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<a/><b2/>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<a/><b2/>");
}
#[test]
fn test_example_578_md_prose_wrapalways_format_1_7066ed5f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<a  /><b2\ndata=\"foo\" >");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<a  /><b2\ndata=\"foo\" >");
}
#[test]
fn test_example_579_md_prose_wrapalways_format_1_e25613d5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("<a foo=\"bar\" bam = 'baz <em>\"</em>'\n_boolean zoop:33=zoop:33 />");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<a foo=\"bar\" bam = 'baz <em>\"</em>'\n_boolean zoop:33=zoop:33 />"
    );
}
#[test]
fn test_example_580_md_prose_wrapalways_format_1_5b4e1845() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("Foo <responsive-image src=\"foo.jpg\" />");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "Foo <responsive-image src=\"foo.jpg\" />");
}
#[test]
fn test_example_581_md_prose_wrapalways_format_1_03773f2c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<33> <__>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<33> <\\\\_\\\\_>");
}
#[test]
fn test_example_582_md_prose_wrapalways_format_1_00c02070() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<a h*#ref=\"hi\">");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<a h\\\\*#ref=\"hi\">");
}
#[test]
fn test_example_583_md_prose_wrapalways_format_1_f03442e4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<a href=\"hi'> <a href=hi'>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<a href=\"hi'> <a href=hi'>");
}
#[test]
fn test_example_584_md_prose_wrapalways_format_1_758f73ab() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("< a><\nfoo><bar/ >");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "< a>< foo><bar/ >");
}
#[test]
fn test_example_585_md_prose_wrapalways_format_1_8c528e32() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<a href='bar'title=title>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<a href='bar'title=title>");
}
#[test]
fn test_example_586_md_prose_wrapalways_format_1_d12b4478() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("</a></foo >");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "</a></foo >");
}
#[test]
fn test_example_587_md_prose_wrapalways_format_1_3f5fcd72() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("</a href=\"foo\">");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "</a href=\"foo\">");
}
#[test]
fn test_example_588_md_prose_wrapalways_format_1_149ea60d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo <!-- this is a\ncomment - with hyphen -->");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo <!-- this is a\ncomment - with hyphen -->");
}
#[test]
fn test_example_589_md_prose_wrapalways_format_1_ed082330() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo <!-- not a comment -- two hyphens -->");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo <!-- not a comment -- two hyphens -->");
}
#[test]
fn test_example_590_md_prose_wrapalways_format_1_3b40ffa6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo <!--> foo -->\n\nfoo <!-- foo--->");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo <!--> foo -->\n\nfoo <!-- foo--->");
}
#[test]
fn test_example_591_md_prose_wrapalways_format_1_5cefe37d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo <?php echo $a; ?>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo <?php echo $a; ?>");
}
#[test]
fn test_example_592_md_prose_wrapalways_format_1_7d039f6b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo <!ELEMENT br EMPTY>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo <!ELEMENT br EMPTY>");
}
#[test]
fn test_example_593_md_prose_wrapalways_format_1_c9a2c063() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo <![CDATA[>&<]]>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo <![CDATA[>&<]]>");
}
#[test]
fn test_example_594_md_prose_wrapalways_format_1_0e388cae() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo <a href=\"&ouml;\">");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo <a href=\"&ouml;\">");
}
#[test]
fn test_example_595_md_prose_wrapalways_format_1_c565b477() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo <a href=\"\\\\*\">");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo <a href=\"\\\\*\">");
}
#[test]
fn test_example_596_md_prose_wrapalways_format_1_5b28bdac() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<a href=\"\\\\\"\">");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<a href=\"\\\\\"\">");
}
#[test]
fn test_example_597_md_prose_wrapalways_format_1_a64146d9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo  \nbaz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo  \nbaz");
}
#[test]
fn test_example_598_md_prose_wrapalways_format_1_7e8eaeae() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo\\\\\nbaz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo\\\\\nbaz");
}
#[test]
fn test_example_599_md_prose_wrapalways_format_1_811ddbb0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo       \nbaz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo  \nbaz");
}
#[test]
fn test_example_600_md_prose_wrapalways_format_1_367fba40() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo  \n     bar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo  \n bar");
}
#[test]
fn test_example_601_md_prose_wrapalways_format_1_41c10565() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo\\\\\n     bar");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo\\\\\n bar");
}
#[test]
fn test_example_602_md_prose_wrapalways_format_1_52611e04() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*foo  \nbar*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_foo  \nbar_");
}
#[test]
fn test_example_603_md_prose_wrapalways_format_1_2b0592d3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("*foo\\\\\nbar*");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "_foo\\\\\nbar_");
}
#[test]
fn test_example_604_md_prose_wrapalways_format_1_6e2e55a4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`code  \nspan\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`code   span\\`");
}
#[test]
fn test_example_605_md_prose_wrapalways_format_1_43bcf31a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`code\\\\\nspan\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`code\\\\ span\\`");
}
#[test]
fn test_example_606_md_prose_wrapalways_format_1_ae89856e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<a href=\"foo  \nbar\">");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<a href=\"foo  \nbar\">");
}
#[test]
fn test_example_607_md_prose_wrapalways_format_1_d37da033() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<a href=\"foo\\\\\nbar\">");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<a href=\"foo\\\\\nbar\">");
}
#[test]
fn test_example_608_md_prose_wrapalways_format_1_80b9eafa() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo\\\\");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo\\\\");
}
#[test]
fn test_example_609_md_prose_wrapalways_format_1_6140adac() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo  ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo");
}
#[test]
fn test_example_610_md_prose_wrapalways_format_1_9f62088d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("### foo\\\\");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "### foo\\\\");
}
#[test]
fn test_example_611_md_prose_wrapalways_format_1_069a9125() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("### foo  ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "### foo");
}
#[test]
fn test_example_612_md_prose_wrapalways_format_1_2b755fd7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo\nbaz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo baz");
}
#[test]
fn test_example_613_md_prose_wrapalways_format_1_fd68b4d9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo \n baz");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo baz");
}
#[test]
fn test_example_614_md_prose_wrapalways_format_1_92e34d30() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("hello $.;'there");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "hello $.;'there");
}
#[test]
fn test_example_615_md_prose_wrapalways_format_1_bd8fbc68() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("Foo χρῆν");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "Foo χρῆν");
}
#[test]
fn test_example_616_md_prose_wrapalways_format_1_d6f1ff02() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("Multiple     spaces");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "Multiple spaces");
}
