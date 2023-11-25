#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_aliases_in_block_sequence_yml_prose_wrapalways_format_1_2292ef49() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- &a a\n- &b b\n- *a\n- *b")?;
    assert_eq!(formatted, "- &a a\n- &b b\n- *a\n- *b");
    Ok(())
}
#[test]
fn test_aliases_in_block_sequence_yml_use_tabstrue_format_1_2292ef49() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- &a a\n- &b b\n- *a\n- *b")?;
    assert_eq!(formatted, "- &a a\n- &b b\n- *a\n- *b");
    Ok(())
}
#[test]
fn test_aliases_in_explicit_block_mapping_yml_prose_wrapalways_format_1_145fd1bc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("? &a a\n: &b b\n: *a")?;
    assert_eq!(formatted, "&a a: &b b\n: *a");
    Ok(())
}
#[test]
fn test_aliases_in_explicit_block_mapping_yml_use_tabstrue_format_1_145fd1bc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("? &a a\n: &b b\n: *a")?;
    assert_eq!(formatted, "&a a: &b b\n: *a");
    Ok(())
}
#[test]
fn test_allowed_characters_in_alias_yml_prose_wrapalways_format_1_f2c677c0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: &:@*!$\"<foo>: scalar a\nb: *:@*!$\"<foo>:")?;
    assert_eq!(formatted, "a: &:@*!$\"<foo>: scalar a\nb: *:@*!$\"<foo>:");
    Ok(())
}
#[test]
fn test_allowed_characters_in_alias_yml_use_tabstrue_format_1_f2c677c0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: &:@*!$\"<foo>: scalar a\nb: *:@*!$\"<foo>:")?;
    assert_eq!(formatted, "a: &:@*!$\"<foo>: scalar a\nb: *:@*!$\"<foo>:");
    Ok(())
}
#[test]
fn test_allowed_characters_in_keys_yml_prose_wrapalways_format_1_d9c6c095() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("a!\"#$%&'()*+,-./09:;<=>?@AZ[\\]^_`az{|}~: safe\n?foo: safe question mark\n:foo: safe colon\n-foo: safe dash\nthis is#not: a comment") ? ;
    assert_eq ! (formatted , "a!\"#$%&'()*+,-./09:;<=>?@AZ[\\]^_`az{|}~: safe\n?foo: safe question mark\n:foo: safe colon\n-foo: safe dash\nthis is#not: a comment");
    Ok(())
}
#[test]
fn test_allowed_characters_in_keys_yml_use_tabstrue_format_1_d9c6c095() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("a!\"#$%&'()*+,-./09:;<=>?@AZ[\\]^_`az{|}~: safe\n?foo: safe question mark\n:foo: safe colon\n-foo: safe dash\nthis is#not: a comment") ? ;
    assert_eq ! (formatted , "a!\"#$%&'()*+,-./09:;<=>?@AZ[\\]^_`az{|}~: safe\n?foo: safe question mark\n:foo: safe colon\n-foo: safe dash\nthis is#not: a comment");
    Ok(())
}
#[test]
fn test_allowed_characters_in_plain_scalars_yml_prose_wrapalways_format_1_8c3a1111() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("safe: a!\"#$%&'()*+,-./09:;<=>?@AZ[\\]^_`az{|}~\n     !\"#$%&'()*+,-./09:;<=>?@AZ[\\]^_`az{|}~\nsafe question mark: ?foo\nsafe colon: :foo\nsafe dash: -foo") ? ;
    assert_eq ! (formatted , "safe:\n  a!\"#$%&'()*+,-./09:;<=>?@AZ[\\]^_`az{|}~ !\"#$%&'()*+,-./09:;<=>?@AZ[\\]^_`az{|}~\nsafe question mark: ?foo\nsafe colon: :foo\nsafe dash: -foo");
    Ok(())
}
#[test]
fn test_allowed_characters_in_plain_scalars_yml_use_tabstrue_format_1_8c3a1111() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("safe: a!\"#$%&'()*+,-./09:;<=>?@AZ[\\]^_`az{|}~\n     !\"#$%&'()*+,-./09:;<=>?@AZ[\\]^_`az{|}~\nsafe question mark: ?foo\nsafe colon: :foo\nsafe dash: -foo") ? ;
    assert_eq ! (formatted , "safe: a!\"#$%&'()*+,-./09:;<=>?@AZ[\\]^_`az{|}~\n  !\"#$%&'()*+,-./09:;<=>?@AZ[\\]^_`az{|}~\nsafe question mark: ?foo\nsafe colon: :foo\nsafe dash: -foo");
    Ok(())
}
#[test]
fn test_allowed_characters_in_quoted_mapping_key_yml_prose_wrapalways_format_1_9c39f69a(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("\"foo\\nbar:baz\\tx \\\\$%^&*()x\": 23\n'x\\ny:z\\tx $%^&*()x': 24")?;
    assert_eq!(
        formatted,
        "\"foo\\nbar:baz\\tx \\\\$%^&*()x\": 23\n'x\\ny:z\\tx $%^&*()x': 24"
    );
    Ok(())
}
#[test]
fn test_allowed_characters_in_quoted_mapping_key_yml_use_tabstrue_format_1_9c39f69a() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("\"foo\\nbar:baz\\tx \\\\$%^&*()x\": 23\n'x\\ny:z\\tx $%^&*()x': 24")?;
    assert_eq!(
        formatted,
        "\"foo\\nbar:baz\\tx \\\\$%^&*()x\": 23\n'x\\ny:z\\tx $%^&*()x': 24"
    );
    Ok(())
}
#[test]
fn test_anchor_before_zero_indented_sequence_yml_prose_wrapalways_format_1_c96e9fc1() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\nseq:\n &anchor\n- a\n- b")?;
    assert_eq!(formatted, "---\nseq: &anchor\n  - a\n  - b");
    Ok(())
}
#[test]
fn test_anchor_before_zero_indented_sequence_yml_use_tabstrue_format_1_c96e9fc1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\nseq:\n &anchor\n- a\n- b")?;
    assert_eq!(formatted, "---\nseq: &anchor\n  - a\n  - b");
    Ok(())
}
#[test]
fn test_anchor_with_unicode_character_yml_prose_wrapalways_format_1_0f18d56d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\n- &😁 unicode anchor")?;
    assert_eq!(formatted, "---\n- &😁 unicode anchor");
    Ok(())
}
#[test]
fn test_anchor_with_unicode_character_yml_use_tabstrue_format_1_0f18d56d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\n- &😁 unicode anchor")?;
    assert_eq!(formatted, "---\n- &😁 unicode anchor");
    Ok(())
}
#[test]
fn test_anchors_and_tags_yml_prose_wrapalways_format_1_c5fc8380() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(" - &a !!str a\n - !!int 2\n - !!int &c 4\n - &d d")?;
    assert_eq!(formatted, "- !!str &a a\n- !!int 2\n- !!int &c 4\n- &d d");
    Ok(())
}
#[test]
fn test_anchors_and_tags_yml_use_tabstrue_format_1_c5fc8380() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(" - &a !!str a\n - !!int 2\n - !!int &c 4\n - &d d")?;
    assert_eq!(formatted, "- !!str &a a\n- !!int 2\n- !!int &c 4\n- &d d");
    Ok(())
}
#[test]
fn test_anchors_in_mapping_yml_prose_wrapalways_format_1_d4d01cec() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("&a a: b\nc: &d d")?;
    assert_eq!(formatted, "&a a: b\nc: &d d");
    Ok(())
}
#[test]
fn test_anchors_in_mapping_yml_use_tabstrue_format_1_d4d01cec() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("&a a: b\nc: &d d")?;
    assert_eq!(formatted, "&a a: b\nc: &d d");
    Ok(())
}
#[test]
fn test_anchors_with_colon_in_name_yml_prose_wrapalways_format_1_8e3f210c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("&a: key: &a value\nfoo:\n  *a:")?;
    assert_eq!(formatted, "&a: key: &a value\nfoo: *a:");
    Ok(())
}
#[test]
fn test_anchors_with_colon_in_name_yml_use_tabstrue_format_1_8e3f210c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("&a: key: &a value\nfoo:\n  *a:")?;
    assert_eq!(formatted, "&a: key: &a value\nfoo: *a:");
    Ok(())
}
#[test]
fn test_backslashes_in_singlequotes_yml_prose_wrapalways_format_1_6d3047ac() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("'foo: bar\\': baz'")?;
    assert_eq!(formatted, "'foo: bar\\': baz'");
    Ok(())
}
#[test]
fn test_backslashes_in_singlequotes_yml_use_tabstrue_format_1_6d3047ac() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("'foo: bar\\': baz'")?;
    assert_eq!(formatted, "'foo: bar\\': baz'");
    Ok(())
}
#[test]
fn test_bare_document_after_document_end_marker_yml_prose_wrapalways_format_1_4ceb495f(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\nscalar1\n...\nkey: value")?;
    assert_eq!(formatted, "---\nscalar1\n---\nkey: value");
    Ok(())
}
#[test]
fn test_bare_document_after_document_end_marker_yml_use_tabstrue_format_1_4ceb495f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\nscalar1\n...\nkey: value")?;
    assert_eq!(formatted, "---\nscalar1\n---\nkey: value");
    Ok(())
}
#[test]
fn test_blank_lines_yml_prose_wrapalways_format_1_15747532() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("foo: 1\n\nbar: 2\n    \ntext: |\n  a\n    \n  b\n\n  c\n \n  d")?;
    assert_eq!(
        formatted,
        "foo: 1\n\nbar: 2\n\ntext: |\n  a\n    \n  b\n\n  c\n\n  d"
    );
    Ok(())
}
#[test]
fn test_blank_lines_yml_use_tabstrue_format_1_15747532() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("foo: 1\n\nbar: 2\n    \ntext: |\n  a\n    \n  b\n\n  c\n \n  d")?;
    assert_eq!(
        formatted,
        "foo: 1\n\nbar: 2\n\ntext: |\n  a\n    \n  b\n\n  c\n\n  d"
    );
    Ok(())
}
#[test]
fn test_block_mapping_with_missing_values_yml_prose_wrapalways_format_1_fc2829f1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("? a\n? b\nc:")?;
    assert_eq!(formatted, "a:\nb:\nc:");
    Ok(())
}
#[test]
fn test_block_mapping_with_missing_values_yml_use_tabstrue_format_1_fc2829f1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("? a\n? b\nc:")?;
    assert_eq!(formatted, "a:\nb:\nc:");
    Ok(())
}
#[test]
fn test_block_mapping_with_multiline_scalars_yml_prose_wrapalways_format_1_5c0faab1() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("? a\n  true\n: null\n  d\n? e\n  42")?;
    assert_eq!(formatted, "a true: null d\n? e 42");
    Ok(())
}
#[test]
fn test_block_mapping_with_multiline_scalars_yml_use_tabstrue_format_1_5c0faab1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("? a\n  true\n: null\n  d\n? e\n  42")?;
    assert_eq!(formatted, "? a\n  true\n: null\n  d\n? e\n  42");
    Ok(())
}
#[test]
fn test_block_mappings_in_block_sequence_yml_prose_wrapalways_format_1_36198fa8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(" - key: value\n   key2: value2\n -\n   key3: value3")?;
    assert_eq!(formatted, "- key: value\n  key2: value2\n- key3: value3");
    Ok(())
}
#[test]
fn test_block_mappings_in_block_sequence_yml_use_tabstrue_format_1_36198fa8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(" - key: value\n   key2: value2\n -\n   key3: value3")?;
    assert_eq!(formatted, "- key: value\n  key2: value2\n- key3: value3");
    Ok(())
}
#[test]
fn test_block_scalar_indicator_order_yml_prose_wrapalways_format_1_c9bd8bf5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("- |2-\n  explicit indent and chomp\n- |-2\n  chomp and explicit indent")?;
    assert_eq!(
        formatted,
        "- |2-\n  explicit indent and chomp\n- |2-\n  chomp and explicit indent"
    );
    Ok(())
}
#[test]
fn test_block_scalar_indicator_order_yml_use_tabstrue_format_1_c9bd8bf5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("- |2-\n  explicit indent and chomp\n- |-2\n  chomp and explicit indent")?;
    assert_eq!(
        formatted,
        "- |2-\n  explicit indent and chomp\n- |2-\n  chomp and explicit indent"
    );
    Ok(())
}
#[test]
fn test_block_scalar_keep_yml_prose_wrapalways_format_1_33ea4e34() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("--- |+\n ab\n \n  \n...")?;
    assert_eq!(formatted, "---\n|+\n  ab\n\n   ");
    Ok(())
}
#[test]
fn test_block_scalar_keep_yml_use_tabstrue_format_1_33ea4e34() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("--- |+\n ab\n \n  \n...")?;
    assert_eq!(formatted, "---\n|+\n  ab\n\n   ");
    Ok(())
}
#[test]
fn test_block_scalar_strip_yml_prose_wrapalways_format_1_8f5808e3() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("|-\n ab\n \n \n...")?;
    assert_eq!(formatted, "|-\n  ab");
    Ok(())
}
#[test]
fn test_block_scalar_strip_yml_use_tabstrue_format_1_8f5808e3() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("|-\n ab\n \n \n...")?;
    assert_eq!(formatted, "|-\n  ab");
    Ok(())
}
#[test]
fn test_block_sequence_in_block_mapping_yml_prose_wrapalways_format_1_87d4c889() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("key:\n - item1\n - item2")?;
    assert_eq!(formatted, "key:\n  - item1\n  - item2");
    Ok(())
}
#[test]
fn test_block_sequence_in_block_mapping_yml_use_tabstrue_format_1_87d4c889() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("key:\n - item1\n - item2")?;
    assert_eq!(formatted, "key:\n  - item1\n  - item2");
    Ok(())
}
#[test]
fn test_block_sequence_in_block_sequence_yml_prose_wrapalways_format_1_20658a26() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- - s1_i1\n  - s1_i2\n- s2")?;
    assert_eq!(formatted, "- - s1_i1\n  - s1_i2\n- s2");
    Ok(())
}
#[test]
fn test_block_sequence_in_block_sequence_yml_use_tabstrue_format_1_20658a26() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- - s1_i1\n  - s1_i2\n- s2")?;
    assert_eq!(formatted, "- - s1_i1\n  - s1_i2\n- s2");
    Ok(())
}
#[test]
fn test_block_submapping_yml_prose_wrapalways_format_1_920dcb3a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo:\n  bar: 1\nbaz: 2")?;
    assert_eq!(formatted, "foo:\n  bar: 1\nbaz: 2");
    Ok(())
}
#[test]
fn test_block_submapping_yml_use_tabstrue_format_1_920dcb3a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo:\n  bar: 1\nbaz: 2")?;
    assert_eq!(formatted, "foo:\n  bar: 1\nbaz: 2");
    Ok(())
}
#[test]
fn test_colon_in_double_quoted_string_yml_prose_wrapalways_format_1_d3c52967() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\"foo: bar\\\": baz\"")?;
    assert_eq!(formatted, "'foo: bar\": baz'");
    Ok(())
}
#[test]
fn test_colon_in_double_quoted_string_yml_use_tabstrue_format_1_d3c52967() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\"foo: bar\\\": baz\"")?;
    assert_eq!(formatted, "'foo: bar\": baz'");
    Ok(())
}
#[test]
fn test_comment_in_flow_sequence_before_comma_yml_prose_wrapalways_format_1_58ec4cd2() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\n[ word1\n# comment\n, word2]")?;
    assert_eq!(formatted, "---\n[\n  word1,\n  # comment\n  word2,\n]");
    Ok(())
}
#[test]
fn test_comment_in_flow_sequence_before_comma_yml_use_tabstrue_format_1_58ec4cd2() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\n[ word1\n# comment\n, word2]")?;
    assert_eq!(formatted, "---\n[\n  word1,\n  # comment\n  word2,\n]");
    Ok(())
}
#[test]
fn test_construct_binary_yml_prose_wrapalways_format_1_c880ae44() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("canonical: !!binary \"\\\n R0lGODlhDAAMAIQAAP//9/X17unp5WZmZgAAAOfn515eXvPz7Y6OjuDg4J+fn5\\\n OTk6enp56enmlpaWNjY6Ojo4SEhP/++f/++f/++f/++f/++f/++f/++f/++f/+\\\n +f/++f/++f/++f/++f/++SH+Dk1hZGUgd2l0aCBHSU1QACwAAAAADAAMAAAFLC\\\n AgjoEwnuNAFOhpEMTRiggcz4BNJHrv/zCFcLiwMWYNG84BwwEeECcgggoBADs=\"\ngeneric: !!binary |\n R0lGODlhDAAMAIQAAP//9/X17unp5WZmZgAAAOfn515eXvPz7Y6OjuDg4J+fn5\n OTk6enp56enmlpaWNjY6Ojo4SEhP/++f/++f/++f/++f/++f/++f/++f/++f/+\n +f/++f/++f/++f/++f/++SH+Dk1hZGUgd2l0aCBHSU1QACwAAAAADAAMAAAFLC\n AgjoEwnuNAFOhpEMTRiggcz4BNJHrv/zCFcLiwMWYNG84BwwEeECcgggoBADs=\ndescription:\n The binary value above is a tiny arrow encoded as a gif image.") ? ;
    assert_eq ! (formatted , "canonical: !!binary \"\\\n  R0lGODlhDAAMAIQAAP//9/X17unp5WZmZgAAAOfn515eXvPz7Y6OjuDg4J+fn5\\\n  OTk6enp56enmlpaWNjY6Ojo4SEhP/++f/++f/++f/++f/++f/++f/++f/++f/+\\\n  +f/++f/++f/++f/++f/++SH+Dk1hZGUgd2l0aCBHSU1QACwAAAAADAAMAAAFLC\\\n  AgjoEwnuNAFOhpEMTRiggcz4BNJHrv/zCFcLiwMWYNG84BwwEeECcgggoBADs=\"\ngeneric: !!binary |\n  R0lGODlhDAAMAIQAAP//9/X17unp5WZmZgAAAOfn515eXvPz7Y6OjuDg4J+fn5\n  OTk6enp56enmlpaWNjY6Ojo4SEhP/++f/++f/++f/++f/++f/++f/++f/++f/+\n  +f/++f/++f/++f/++f/++SH+Dk1hZGUgd2l0aCBHSU1QACwAAAAADAAMAAAFLC\n  AgjoEwnuNAFOhpEMTRiggcz4BNJHrv/zCFcLiwMWYNG84BwwEeECcgggoBADs=\ndescription: The binary value above is a tiny arrow encoded as a gif image.");
    Ok(())
}
#[test]
fn test_construct_binary_yml_use_tabstrue_format_1_c880ae44() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("canonical: !!binary \"\\\n R0lGODlhDAAMAIQAAP//9/X17unp5WZmZgAAAOfn515eXvPz7Y6OjuDg4J+fn5\\\n OTk6enp56enmlpaWNjY6Ojo4SEhP/++f/++f/++f/++f/++f/++f/++f/++f/+\\\n +f/++f/++f/++f/++f/++SH+Dk1hZGUgd2l0aCBHSU1QACwAAAAADAAMAAAFLC\\\n AgjoEwnuNAFOhpEMTRiggcz4BNJHrv/zCFcLiwMWYNG84BwwEeECcgggoBADs=\"\ngeneric: !!binary |\n R0lGODlhDAAMAIQAAP//9/X17unp5WZmZgAAAOfn515eXvPz7Y6OjuDg4J+fn5\n OTk6enp56enmlpaWNjY6Ojo4SEhP/++f/++f/++f/++f/++f/++f/++f/++f/+\n +f/++f/++f/++f/++f/++SH+Dk1hZGUgd2l0aCBHSU1QACwAAAAADAAMAAAFLC\n AgjoEwnuNAFOhpEMTRiggcz4BNJHrv/zCFcLiwMWYNG84BwwEeECcgggoBADs=\ndescription:\n The binary value above is a tiny arrow encoded as a gif image.") ? ;
    assert_eq ! (formatted , "canonical: !!binary \"\\\n  R0lGODlhDAAMAIQAAP//9/X17unp5WZmZgAAAOfn515eXvPz7Y6OjuDg4J+fn5\\\n  OTk6enp56enmlpaWNjY6Ojo4SEhP/++f/++f/++f/++f/++f/++f/++f/++f/+\\\n  +f/++f/++f/++f/++f/++SH+Dk1hZGUgd2l0aCBHSU1QACwAAAAADAAMAAAFLC\\\n  AgjoEwnuNAFOhpEMTRiggcz4BNJHrv/zCFcLiwMWYNG84BwwEeECcgggoBADs=\"\ngeneric: !!binary |\n  R0lGODlhDAAMAIQAAP//9/X17unp5WZmZgAAAOfn515eXvPz7Y6OjuDg4J+fn5\n  OTk6enp56enmlpaWNjY6Ojo4SEhP/++f/++f/++f/++f/++f/++f/++f/++f/+\n  +f/++f/++f/++f/++f/++SH+Dk1hZGUgd2l0aCBHSU1QACwAAAAADAAMAAAFLC\n  AgjoEwnuNAFOhpEMTRiggcz4BNJHrv/zCFcLiwMWYNG84BwwEeECcgggoBADs=\ndescription: The binary value above is a tiny arrow encoded as a gif image.");
    Ok(())
}
#[test]
fn test_document_start_on_last_line_yml_prose_wrapalways_format_1_e31113f8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\na: b\n---")?;
    assert_eq!(formatted, "---\na: b\n---\n");
    Ok(())
}
#[test]
fn test_document_start_on_last_line_yml_use_tabstrue_format_1_e31113f8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\na: b\n---")?;
    assert_eq!(formatted, "---\na: b\n---\n");
    Ok(())
}
#[test]
fn test_document_with_footer_yml_prose_wrapalways_format_1_d460397f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("aaa: bbb\n...")?;
    assert_eq!(formatted, "aaa: bbb");
    Ok(())
}
#[test]
fn test_document_with_footer_yml_use_tabstrue_format_1_d460397f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("aaa: bbb\n...")?;
    assert_eq!(formatted, "aaa: bbb");
    Ok(())
}
#[test]
fn test_empty_lines_at_end_of_document_yml_prose_wrapalways_format_1_97782dcc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(":\n\n\n")?;
    assert_eq!(formatted, ":");
    Ok(())
}
#[test]
fn test_empty_lines_at_end_of_document_yml_use_tabstrue_format_1_97782dcc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(":\n\n\n")?;
    assert_eq!(formatted, ":");
    Ok(())
}
#[test]
fn test_empty_lines_between_mapping_elements_yml_prose_wrapalways_format_1_b19dc8bb() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1: 2\n\n\n3: 4")?;
    assert_eq!(formatted, "1: 2\n\n3: 4");
    Ok(())
}
#[test]
fn test_empty_lines_between_mapping_elements_yml_use_tabstrue_format_1_b19dc8bb() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1: 2\n\n\n3: 4")?;
    assert_eq!(formatted, "1: 2\n\n3: 4");
    Ok(())
}
#[test]
fn test_empty_stream_yml_prose_wrapalways_format_1_68b329da() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_empty_stream_yml_use_tabstrue_format_1_68b329da() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_escaped_slash_in_double_quotes_yml_prose_wrapalways_format_1_1947b750() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("escaped slash: \"a\\/b\"")?;
    assert_eq!(formatted, "escaped slash: \"a\\/b\"");
    Ok(())
}
#[test]
fn test_escaped_slash_in_double_quotes_yml_use_tabstrue_format_1_1947b750() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("escaped slash: \"a\\/b\"")?;
    assert_eq!(formatted, "escaped slash: \"a\\/b\"");
    Ok(())
}
#[test]
fn test_explicit_non_specific_tag_yml_prose_wrapalways_format_1_7e2db921() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("! a")?;
    assert_eq!(formatted, "! a");
    Ok(())
}
#[test]
fn test_explicit_non_specific_tag_yml_use_tabstrue_format_1_7e2db921() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("! a")?;
    assert_eq!(formatted, "! a");
    Ok(())
}
#[test]
fn test_flow_mapping_yml_prose_wrapalways_format_1_103170f8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{foo: you, bar: far}")?;
    assert_eq!(formatted, "{ foo: you, bar: far }");
    Ok(())
}
#[test]
fn test_flow_mapping_yml_use_tabstrue_format_1_103170f8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{foo: you, bar: far}")?;
    assert_eq!(formatted, "{ foo: you, bar: far }");
    Ok(())
}
#[test]
fn test_flow_mapping_in_block_sequence_yml_prose_wrapalways_format_1_7195d4dc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- {a: b}")?;
    assert_eq!(formatted, "- { a: b }");
    Ok(())
}
#[test]
fn test_flow_mapping_in_block_sequence_yml_use_tabstrue_format_1_7195d4dc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- {a: b}")?;
    assert_eq!(formatted, "- { a: b }");
    Ok(())
}
#[test]
fn test_flow_sequence_yml_prose_wrapalways_format_1_1a4491d5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo, bar, 42]")?;
    assert_eq!(formatted, "[foo, bar, 42]");
    Ok(())
}
#[test]
fn test_flow_sequence_yml_use_tabstrue_format_1_1a4491d5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[foo, bar, 42]")?;
    assert_eq!(formatted, "[foo, bar, 42]");
    Ok(())
}
#[test]
fn test_flow_sequence_in_block_mapping_yml_prose_wrapalways_format_1_053b8339() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: [b, c]")?;
    assert_eq!(formatted, "a: [b, c]");
    Ok(())
}
#[test]
fn test_flow_sequence_in_block_mapping_yml_use_tabstrue_format_1_053b8339() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: [b, c]")?;
    assert_eq!(formatted, "a: [b, c]");
    Ok(())
}
#[test]
fn test_flow_sequence_in_flow_mapping_yml_prose_wrapalways_format_1_6ba7a48a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{a: [b, c], [d, e]: f}")?;
    assert_eq!(formatted, "{ a: [b, c], [d, e]: f }");
    Ok(())
}
#[test]
fn test_flow_sequence_in_flow_mapping_yml_use_tabstrue_format_1_6ba7a48a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{a: [b, c], [d, e]: f}")?;
    assert_eq!(formatted, "{ a: [b, c], [d, e]: f }");
    Ok(())
}
#[test]
fn test_flow_sequence_in_flow_sequence_yml_prose_wrapalways_format_1_22eb12b7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[a, [b, c]]")?;
    assert_eq!(formatted, "[a, [b, c]]");
    Ok(())
}
#[test]
fn test_flow_sequence_in_flow_sequence_yml_use_tabstrue_format_1_22eb12b7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[a, [b, c]]")?;
    assert_eq!(formatted, "[a, [b, c]]");
    Ok(())
}
#[test]
fn test_folded_block_scalar_yml_prose_wrapalways_format_1_12114c15() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(">\n ab\n cd\n \n ef\n\n\n gh")?;
    assert_eq!(formatted, ">\n  ab cd\n\n  ef\n\n\n  gh");
    Ok(())
}
#[test]
fn test_folded_block_scalar_yml_use_tabstrue_format_1_12114c15() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(">\n ab\n cd\n \n ef\n\n\n gh")?;
    assert_eq!(formatted, ">\n  ab\n  cd\n\n  ef\n\n\n  gh");
    Ok(())
}
#[test]
fn test_implicit_flow_mapping_key_on_one_line_yml_prose_wrapalways_format_1_13bbbc2c() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[flow]: block")?;
    assert_eq!(formatted, "[flow]: block");
    Ok(())
}
#[test]
fn test_implicit_flow_mapping_key_on_one_line_yml_use_tabstrue_format_1_13bbbc2c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[flow]: block")?;
    assert_eq!(formatted, "[flow]: block");
    Ok(())
}
#[test]
fn test_key_with_anchor_after_missing_explicit_mapping_value_yml_prose_wrapalways_format_1_43c85604(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\na: 1\n? b\n&anchor c: 3")?;
    assert_eq!(formatted, "---\na: 1\nb:\n&anchor c: 3");
    Ok(())
}
#[test]
fn test_key_with_anchor_after_missing_explicit_mapping_value_yml_use_tabstrue_format_1_43c85604(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\na: 1\n? b\n&anchor c: 3")?;
    assert_eq!(formatted, "---\na: 1\nb:\n&anchor c: 3");
    Ok(())
}
#[test]
fn test_literal_block_scalar_yml_prose_wrapalways_format_1_f8a2d50c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: |\n ab\n \n cd\n ef\n \n\n...")?;
    assert_eq!(formatted, "a: |\n  ab\n\n  cd\n  ef");
    Ok(())
}
#[test]
fn test_literal_block_scalar_yml_use_tabstrue_format_1_f8a2d50c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: |\n ab\n \n cd\n ef\n \n\n...")?;
    assert_eq!(formatted, "a: |\n  ab\n\n  cd\n  ef");
    Ok(())
}
#[test]
fn test_literal_unicode_yml_prose_wrapalways_format_1_2b7e1146() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\nwanted: love ♥ and peace ☮")?;
    assert_eq!(formatted, "---\nwanted: love ♥ and peace ☮");
    Ok(())
}
#[test]
fn test_literal_unicode_yml_use_tabstrue_format_1_2b7e1146() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\nwanted: love ♥ and peace ☮")?;
    assert_eq!(formatted, "---\nwanted: love ♥ and peace ☮");
    Ok(())
}
#[test]
fn test_lookahead_test_cases_yml_prose_wrapalways_format_1_9faf3299() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- bla\"keks: foo\n- bla]keks: foo")?;
    assert_eq!(formatted, "- bla\"keks: foo\n- bla]keks: foo");
    Ok(())
}
#[test]
fn test_lookahead_test_cases_yml_use_tabstrue_format_1_9faf3299() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- bla\"keks: foo\n- bla]keks: foo")?;
    assert_eq!(formatted, "- bla\"keks: foo\n- bla]keks: foo");
    Ok(())
}
#[test]
fn test_mapping_key_and_flow_sequence_item_anchors_yml_prose_wrapalways_format_1_bbbaf0ae(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\n&mapping\n&key [ &item a, b, c ]: value")?;
    assert_eq!(formatted, "---\n&mapping\n&key [&item a, b, c]: value");
    Ok(())
}
#[test]
fn test_mapping_key_and_flow_sequence_item_anchors_yml_use_tabstrue_format_1_bbbaf0ae() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\n&mapping\n&key [ &item a, b, c ]: value")?;
    assert_eq!(formatted, "---\n&mapping\n&key [&item a, b, c]: value");
    Ok(())
}
#[test]
fn test_mixed_block_mapping_explicit_to_implicit_yml_prose_wrapalways_format_1_c896f33c(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("? a\n: 13\n1.5: d")?;
    assert_eq!(formatted, "a: 13\n1.5: d");
    Ok(())
}
#[test]
fn test_mixed_block_mapping_explicit_to_implicit_yml_use_tabstrue_format_1_c896f33c() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("? a\n: 13\n1.5: d")?;
    assert_eq!(formatted, "a: 13\n1.5: d");
    Ok(())
}
#[test]
fn test_mixed_block_mapping_implicit_to_explicit_yml_prose_wrapalways_format_1_98bd8f68(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: 4.2\n? 23\n: d")?;
    assert_eq!(formatted, "a: 4.2\n23: d");
    Ok(())
}
#[test]
fn test_mixed_block_mapping_implicit_to_explicit_yml_use_tabstrue_format_1_98bd8f68() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: 4.2\n? 23\n: d")?;
    assert_eq!(formatted, "a: 4.2\n23: d");
    Ok(())
}
#[test]
fn test_multi_level_mapping_indent_yml_prose_wrapalways_format_1_e5e199b7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a:\n  b:\n    c: d\n  e:\n    f: g\nh: i")?;
    assert_eq!(formatted, "a:\n  b:\n    c: d\n  e:\n    f: g\nh: i");
    Ok(())
}
#[test]
fn test_multi_level_mapping_indent_yml_use_tabstrue_format_1_e5e199b7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a:\n  b:\n    c: d\n  e:\n    f: g\nh: i")?;
    assert_eq!(formatted, "a:\n  b:\n    c: d\n  e:\n    f: g\nh: i");
    Ok(())
}
#[test]
fn test_multiline_plain_scalar_with_empty_line_yml_prose_wrapalways_format_1_6a64e9d1() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\nplain: a\n b\n\n c")?;
    assert_eq!(formatted, "---\nplain: a b\n\n  c");
    Ok(())
}
#[test]
fn test_multiline_plain_scalar_with_empty_line_yml_use_tabstrue_format_1_6a64e9d1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\nplain: a\n b\n\n c")?;
    assert_eq!(formatted, "---\nplain: a\n  b\n\n  c");
    Ok(())
}
#[test]
fn test_multiline_scalar_at_top_level_yml_prose_wrapalways_format_1_7e99396c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a\nb  \n  c\nd\n\ne")?;
    assert_eq!(formatted, "a b c d\n\ne");
    Ok(())
}
#[test]
fn test_multiline_scalar_at_top_level_yml_use_tabstrue_format_1_7e99396c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a\nb  \n  c\nd\n\ne")?;
    assert_eq!(formatted, "a\nb\nc\nd\n\ne");
    Ok(())
}
#[test]
fn test_multiline_scalar_in_mapping_yml_prose_wrapalways_format_1_69a5269c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: b\n c\nd:\n e\n  f")?;
    assert_eq!(formatted, "a: b c\nd: e f");
    Ok(())
}
#[test]
fn test_multiline_scalar_in_mapping_yml_use_tabstrue_format_1_69a5269c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: b\n c\nd:\n e\n  f")?;
    assert_eq!(formatted, "a: b\n  c\nd: e\n  f");
    Ok(())
}
#[test]
fn test_multiline_scalar_that_looks_like_a_yaml_directive_yml_prose_wrapalways_format_1_ab1a4e3e(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\nscalar\n%YAML 1.2")?;
    assert_eq!(formatted, "---\nscalar %YAML 1.2");
    Ok(())
}
#[test]
fn test_multiline_scalar_that_looks_like_a_yaml_directive_yml_use_tabstrue_format_1_ab1a4e3e(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\nscalar\n%YAML 1.2")?;
    assert_eq!(formatted, "---\nscalar\n%YAML 1.2");
    Ok(())
}
#[test]
fn test_multiple_entry_block_sequence_yml_prose_wrapalways_format_1_f4e3ab46() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- foo\n- bar\n- 42")?;
    assert_eq!(formatted, "- foo\n- bar\n- 42");
    Ok(())
}
#[test]
fn test_multiple_entry_block_sequence_yml_use_tabstrue_format_1_f4e3ab46() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- foo\n- bar\n- 42")?;
    assert_eq!(formatted, "- foo\n- bar\n- 42");
    Ok(())
}
#[test]
fn test_multiple_pair_block_mapping_yml_prose_wrapalways_format_1_bf5a14ad() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo: blue\nbar: arrr\nbaz: jazz")?;
    assert_eq!(formatted, "foo: blue\nbar: arrr\nbaz: jazz");
    Ok(())
}
#[test]
fn test_multiple_pair_block_mapping_yml_use_tabstrue_format_1_bf5a14ad() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo: blue\nbar: arrr\nbaz: jazz")?;
    assert_eq!(formatted, "foo: blue\nbar: arrr\nbaz: jazz");
    Ok(())
}
#[test]
fn test_nested_flow_collections_yml_prose_wrapalways_format_1_8bc100e6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\n{\n a: [\n  b, c, {\n   d: [e, f]\n  }\n ]\n}")?;
    assert_eq!(formatted, "---\n{ a: [b, c, { d: [e, f] }] }");
    Ok(())
}
#[test]
fn test_nested_flow_collections_yml_use_tabstrue_format_1_8bc100e6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\n{\n a: [\n  b, c, {\n   d: [e, f]\n  }\n ]\n}")?;
    assert_eq!(formatted, "---\n{ a: [b, c, { d: [e, f] }] }");
    Ok(())
}
#[test]
fn test_nested_flow_collections_on_one_line_yml_prose_wrapalways_format_1_2001eeac() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\n{ a: [b, c, { d: [e, f] } ] }")?;
    assert_eq!(formatted, "---\n{ a: [b, c, { d: [e, f] }] }");
    Ok(())
}
#[test]
fn test_nested_flow_collections_on_one_line_yml_use_tabstrue_format_1_2001eeac() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\n{ a: [b, c, { d: [e, f] } ] }")?;
    assert_eq!(formatted, "---\n{ a: [b, c, { d: [e, f] }] }");
    Ok(())
}
#[test]
fn test_node_anchor_and_tag_on_seperate_lines_yml_prose_wrapalways_format_1_198ff717() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("key: &anchor\n !!map\n  a: b")?;
    assert_eq!(formatted, "key: !!map &anchor\n  a: b");
    Ok(())
}
#[test]
fn test_node_anchor_and_tag_on_seperate_lines_yml_use_tabstrue_format_1_198ff717() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("key: &anchor\n !!map\n  a: b")?;
    assert_eq!(formatted, "key: !!map &anchor\n  a: b");
    Ok(())
}
#[test]
fn test_node_and_mapping_key_anchors_yml_prose_wrapalways_format_1_2c282907() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("---\ntop1: &node1\n  &k1 key1: one\ntop2: &node2 # comment\n  key2: two\ntop3:\n  &k3 key3: three\ntop4:\n  &node4\n  &k4 key4: four\ntop5:\n  &node5\n  key5: five\ntop6: &val6\n  six\ntop7:\n  &val7 seven") ? ;
    assert_eq ! (formatted , "---\ntop1: &node1\n  &k1 key1: one\ntop2: &node2 # comment\n  key2: two\ntop3:\n  &k3 key3: three\ntop4: &node4\n  &k4 key4: four\ntop5: &node5\n  key5: five\ntop6: &val6 six\ntop7: &val7 seven");
    Ok(())
}
#[test]
fn test_node_and_mapping_key_anchors_yml_use_tabstrue_format_1_2c282907() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("---\ntop1: &node1\n  &k1 key1: one\ntop2: &node2 # comment\n  key2: two\ntop3:\n  &k3 key3: three\ntop4:\n  &node4\n  &k4 key4: four\ntop5:\n  &node5\n  key5: five\ntop6: &val6\n  six\ntop7:\n  &val7 seven") ? ;
    assert_eq ! (formatted , "---\ntop1: &node1\n  &k1 key1: one\ntop2: &node2 # comment\n  key2: two\ntop3:\n  &k3 key3: three\ntop4: &node4\n  &k4 key4: four\ntop5: &node5\n  key5: five\ntop6: &val6 six\ntop7: &val7 seven");
    Ok(())
}
#[test]
fn test_non_specific_tags_on_scalars_yml_prose_wrapalways_format_1_83c3229d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("- plain\n- \"double quoted\"\n- 'single quoted'\n- >\n  block\n- plain again")?;
    assert_eq!(
        formatted,
        "- plain\n- \"double quoted\"\n- \"single quoted\"\n- >\n  block\n- plain again"
    );
    Ok(())
}
#[test]
fn test_non_specific_tags_on_scalars_yml_use_tabstrue_format_1_83c3229d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("- plain\n- \"double quoted\"\n- 'single quoted'\n- >\n  block\n- plain again")?;
    assert_eq!(
        formatted,
        "- plain\n- \"double quoted\"\n- \"single quoted\"\n- >\n  block\n- plain again"
    );
    Ok(())
}
#[test]
fn test_plain_mapping_key_ending_with_colon_yml_prose_wrapalways_format_1_fe33580e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\nkey ends with two colons::: value")?;
    assert_eq!(formatted, "---\nkey ends with two colons::: value");
    Ok(())
}
#[test]
fn test_plain_mapping_key_ending_with_colon_yml_use_tabstrue_format_1_fe33580e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\nkey ends with two colons::: value")?;
    assert_eq!(formatted, "---\nkey ends with two colons::: value");
    Ok(())
}
#[test]
fn test_plain_scalar_looking_like_key_comment_anchor_and_tag_yml_prose_wrapalways_format_1_6c0ef8c9(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("k:#foo\n &a !t s")?;
    assert_eq!(formatted, "k:#foo &a !t s");
    Ok(())
}
#[test]
fn test_plain_scalar_looking_like_key_comment_anchor_and_tag_yml_use_tabstrue_format_1_6c0ef8c9(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("k:#foo\n &a !t s")?;
    assert_eq!(formatted, "k:#foo\n&a !t s");
    Ok(())
}
#[test]
fn test_plain_scalar_with_backslashes_yml_prose_wrapalways_format_1_0709ff4d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\nplain\\value\\with\\backslashes")?;
    assert_eq!(formatted, "---\nplain\\value\\with\\backslashes");
    Ok(())
}
#[test]
fn test_plain_scalar_with_backslashes_yml_use_tabstrue_format_1_0709ff4d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\nplain\\value\\with\\backslashes")?;
    assert_eq!(formatted, "---\nplain\\value\\with\\backslashes");
    Ok(())
}
#[test]
fn test_plain_url_in_flow_mapping_yml_prose_wrapalways_format_1_ffb9b1df() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- { url: http://example.org }")?;
    assert_eq!(formatted, "- { url: http://example.org }");
    Ok(())
}
#[test]
fn test_plain_url_in_flow_mapping_yml_use_tabstrue_format_1_ffb9b1df() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- { url: http://example.org }")?;
    assert_eq!(formatted, "- { url: http://example.org }");
    Ok(())
}
#[test]
fn test_scalars_on_line_yml_prose_wrapalways_format_1_ec30f6d9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("--- \"quoted\nstring\"\n--- &node foo")?;
    assert_eq!(formatted, "---\n\"quoted string\"\n---\n&node foo");
    Ok(())
}
#[test]
fn test_scalars_on_line_yml_use_tabstrue_format_1_ec30f6d9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("--- \"quoted\nstring\"\n--- &node foo")?;
    assert_eq!(formatted, "---\n\"quoted\nstring\"\n---\n&node foo");
    Ok(())
}
#[test]
fn test_sequence_entry_that_looks_like_two_with_wrong_indentation_yml_prose_wrapalways_format_1_c8a47f4c(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- single multiline\n - sequence entry")?;
    assert_eq!(formatted, "- single multiline - sequence entry");
    Ok(())
}
#[test]
fn test_sequence_entry_that_looks_like_two_with_wrong_indentation_yml_use_tabstrue_format_1_c8a47f4c(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- single multiline\n - sequence entry")?;
    assert_eq!(formatted, "- single multiline\n  - sequence entry");
    Ok(())
}
#[test]
fn test_sequence_indent_yml_prose_wrapalways_format_1_fd4fd10d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo:\n- 42\nbar:\n  - 44")?;
    assert_eq!(formatted, "foo:\n  - 42\nbar:\n  - 44");
    Ok(())
}
#[test]
fn test_sequence_indent_yml_use_tabstrue_format_1_fd4fd10d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo:\n- 42\nbar:\n  - 44")?;
    assert_eq!(formatted, "foo:\n  - 42\nbar:\n  - 44");
    Ok(())
}
#[test]
fn test_sequence_with_same_indentation_as_parent_mapping_yml_prose_wrapalways_format_1_d33d8a8c(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1:\n- 2\n- 3\n4: 5")?;
    assert_eq!(formatted, "1:\n  - 2\n  - 3\n4: 5");
    Ok(())
}
#[test]
fn test_sequence_with_same_indentation_as_parent_mapping_yml_use_tabstrue_format_1_d33d8a8c(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1:\n- 2\n- 3\n4: 5")?;
    assert_eq!(formatted, "1:\n  - 2\n  - 3\n4: 5");
    Ok(())
}
#[test]
fn test_simple_mapping_indent_yml_prose_wrapalways_format_1_a6fc5dbb() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo:\n  bar: baz")?;
    assert_eq!(formatted, "foo:\n  bar: baz");
    Ok(())
}
#[test]
fn test_simple_mapping_indent_yml_use_tabstrue_format_1_a6fc5dbb() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo:\n  bar: baz")?;
    assert_eq!(formatted, "foo:\n  bar: baz");
    Ok(())
}
#[test]
fn test_single_entry_block_sequence_yml_prose_wrapalways_format_1_0947182a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- foo")?;
    assert_eq!(formatted, "- foo");
    Ok(())
}
#[test]
fn test_single_entry_block_sequence_yml_use_tabstrue_format_1_0947182a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- foo")?;
    assert_eq!(formatted, "- foo");
    Ok(())
}
#[test]
fn test_single_pair_block_mapping_yml_prose_wrapalways_format_1_f5abb3e0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo: bar")?;
    assert_eq!(formatted, "foo: bar");
    Ok(())
}
#[test]
fn test_single_pair_block_mapping_yml_use_tabstrue_format_1_f5abb3e0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo: bar")?;
    assert_eq!(formatted, "foo: bar");
    Ok(())
}
#[test]
fn test_spec_example_2_1_sequence_of_scalars_yml_prose_wrapalways_format_1_67795e8c() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- Mark McGwire\n- Sammy Sosa\n- Ken Griffey")?;
    assert_eq!(formatted, "- Mark McGwire\n- Sammy Sosa\n- Ken Griffey");
    Ok(())
}
#[test]
fn test_spec_example_2_1_sequence_of_scalars_yml_use_tabstrue_format_1_67795e8c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- Mark McGwire\n- Sammy Sosa\n- Ken Griffey")?;
    assert_eq!(formatted, "- Mark McGwire\n- Sammy Sosa\n- Ken Griffey");
    Ok(())
}
#[test]
fn test_spec_example_2_2_mapping_scalars_to_scalars_yml_prose_wrapalways_format_1_de9d2930(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "hr:  65    # Home runs\navg: 0.278 # Batting average\nrbi: 147   # Runs Batted In",
    )?;
    assert_eq!(
        formatted,
        "hr: 65 # Home runs\navg: 0.278 # Batting average\nrbi: 147 # Runs Batted In"
    );
    Ok(())
}
#[test]
fn test_spec_example_2_2_mapping_scalars_to_scalars_yml_use_tabstrue_format_1_de9d2930(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "hr:  65    # Home runs\navg: 0.278 # Batting average\nrbi: 147   # Runs Batted In",
    )?;
    assert_eq!(
        formatted,
        "hr: 65 # Home runs\navg: 0.278 # Batting average\nrbi: 147 # Runs Batted In"
    );
    Ok(())
}
#[test]
fn test_spec_example_2_3_mapping_scalars_to_sequences_yml_prose_wrapalways_format_1_452d4963(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("american:\n  - Boston Red Sox\n  - Detroit Tigers\n  - New York Yankees\nnational:\n  - New York Mets\n  - Chicago Cubs\n  - Atlanta Braves") ? ;
    assert_eq ! (formatted , "american:\n  - Boston Red Sox\n  - Detroit Tigers\n  - New York Yankees\nnational:\n  - New York Mets\n  - Chicago Cubs\n  - Atlanta Braves");
    Ok(())
}
#[test]
fn test_spec_example_2_3_mapping_scalars_to_sequences_yml_use_tabstrue_format_1_452d4963(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("american:\n  - Boston Red Sox\n  - Detroit Tigers\n  - New York Yankees\nnational:\n  - New York Mets\n  - Chicago Cubs\n  - Atlanta Braves") ? ;
    assert_eq ! (formatted , "american:\n  - Boston Red Sox\n  - Detroit Tigers\n  - New York Yankees\nnational:\n  - New York Mets\n  - Chicago Cubs\n  - Atlanta Braves");
    Ok(())
}
#[test]
fn test_spec_example_2_4_sequence_of_mappings_yml_prose_wrapalways_format_1_745dc419() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("-\n  name: Mark McGwire\n  hr:   65\n  avg:  0.278\n-\n  name: Sammy Sosa\n  hr:   63\n  avg:  0.288") ? ;
    assert_eq!(
        formatted,
        "- name: Mark McGwire\n  hr: 65\n  avg: 0.278\n- name: Sammy Sosa\n  hr: 63\n  avg: 0.288"
    );
    Ok(())
}
#[test]
fn test_spec_example_2_4_sequence_of_mappings_yml_use_tabstrue_format_1_745dc419() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("-\n  name: Mark McGwire\n  hr:   65\n  avg:  0.278\n-\n  name: Sammy Sosa\n  hr:   63\n  avg:  0.288") ? ;
    assert_eq!(
        formatted,
        "- name: Mark McGwire\n  hr: 65\n  avg: 0.278\n- name: Sammy Sosa\n  hr: 63\n  avg: 0.288"
    );
    Ok(())
}
#[test]
fn test_spec_example_2_5_sequence_of_sequences_yml_prose_wrapalways_format_1_4b206cf0() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "- [name        , hr, avg  ]\n- [Mark McGwire, 65, 0.278]\n- [Sammy Sosa  , 63, 0.288]",
    )?;
    assert_eq!(
        formatted,
        "- [name, hr, avg]\n- [Mark McGwire, 65, 0.278]\n- [Sammy Sosa, 63, 0.288]"
    );
    Ok(())
}
#[test]
fn test_spec_example_2_5_sequence_of_sequences_yml_use_tabstrue_format_1_4b206cf0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "- [name        , hr, avg  ]\n- [Mark McGwire, 65, 0.278]\n- [Sammy Sosa  , 63, 0.288]",
    )?;
    assert_eq!(
        formatted,
        "- [name, hr, avg]\n- [Mark McGwire, 65, 0.278]\n- [Sammy Sosa, 63, 0.288]"
    );
    Ok(())
}
#[test]
fn test_spec_example_2_6_mapping_of_mappings_yml_prose_wrapalways_format_1_c894c183() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "Mark McGwire: {hr: 65, avg: 0.278}\nSammy Sosa: {\n    hr: 63,\n    avg: 0.288\n  }",
    )?;
    assert_eq!(
        formatted,
        "Mark McGwire: { hr: 65, avg: 0.278 }\nSammy Sosa: { hr: 63, avg: 0.288 }"
    );
    Ok(())
}
#[test]
fn test_spec_example_2_6_mapping_of_mappings_yml_use_tabstrue_format_1_c894c183() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "Mark McGwire: {hr: 65, avg: 0.278}\nSammy Sosa: {\n    hr: 63,\n    avg: 0.288\n  }",
    )?;
    assert_eq!(
        formatted,
        "Mark McGwire: { hr: 65, avg: 0.278 }\nSammy Sosa: { hr: 63, avg: 0.288 }"
    );
    Ok(())
}
#[test]
fn test_spec_example_2_7_two_documents_in_a_stream_yml_prose_wrapalways_format_1_e2b10e9a(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("# Ranking of 1998 home runs\n---\n- Mark McGwire\n- Sammy Sosa\n- Ken Griffey\n\n# Team ranking\n---\n- Chicago Cubs\n- St Louis Cardinals") ? ;
    assert_eq ! (formatted , "# Ranking of 1998 home runs\n---\n- Mark McGwire\n- Sammy Sosa\n- Ken Griffey\n\n# Team ranking\n---\n- Chicago Cubs\n- St Louis Cardinals");
    Ok(())
}
#[test]
fn test_spec_example_2_7_two_documents_in_a_stream_yml_use_tabstrue_format_1_e2b10e9a() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("# Ranking of 1998 home runs\n---\n- Mark McGwire\n- Sammy Sosa\n- Ken Griffey\n\n# Team ranking\n---\n- Chicago Cubs\n- St Louis Cardinals") ? ;
    assert_eq ! (formatted , "# Ranking of 1998 home runs\n---\n- Mark McGwire\n- Sammy Sosa\n- Ken Griffey\n\n# Team ranking\n---\n- Chicago Cubs\n- St Louis Cardinals");
    Ok(())
}
#[test]
fn test_spec_example_2_8_play_by_play_feed_from_a_game_yml_prose_wrapalways_format_1_8c7bd218(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("---\ntime: 20:03:20\nplayer: Sammy Sosa\naction: strike (miss)\n...\n---\ntime: 20:03:47\nplayer: Sammy Sosa\naction: grand slam\n...") ? ;
    assert_eq ! (formatted , "---\ntime: 20:03:20\nplayer: Sammy Sosa\naction: strike (miss)\n---\ntime: 20:03:47\nplayer: Sammy Sosa\naction: grand slam");
    Ok(())
}
#[test]
fn test_spec_example_2_8_play_by_play_feed_from_a_game_yml_use_tabstrue_format_1_8c7bd218(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("---\ntime: 20:03:20\nplayer: Sammy Sosa\naction: strike (miss)\n...\n---\ntime: 20:03:47\nplayer: Sammy Sosa\naction: grand slam\n...") ? ;
    assert_eq ! (formatted , "---\ntime: 20:03:20\nplayer: Sammy Sosa\naction: strike (miss)\n---\ntime: 20:03:47\nplayer: Sammy Sosa\naction: grand slam");
    Ok(())
}
#[test]
fn test_spec_example_2_9_single_document_with_two_comments_yml_prose_wrapalways_format_1_8cd0c691(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("---\nhr: # 1998 hr ranking\n  - Mark McGwire\n  - Sammy Sosa\nrbi:\n  # 1998 rbi ranking\n  - Sammy Sosa\n  - Ken Griffey") ? ;
    assert_eq ! (formatted , "---\nhr: # 1998 hr ranking\n  - Mark McGwire\n  - Sammy Sosa\nrbi:\n  # 1998 rbi ranking\n  - Sammy Sosa\n  - Ken Griffey");
    Ok(())
}
#[test]
fn test_spec_example_2_9_single_document_with_two_comments_yml_use_tabstrue_format_1_8cd0c691(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("---\nhr: # 1998 hr ranking\n  - Mark McGwire\n  - Sammy Sosa\nrbi:\n  # 1998 rbi ranking\n  - Sammy Sosa\n  - Ken Griffey") ? ;
    assert_eq ! (formatted , "---\nhr: # 1998 hr ranking\n  - Mark McGwire\n  - Sammy Sosa\nrbi:\n  # 1998 rbi ranking\n  - Sammy Sosa\n  - Ken Griffey");
    Ok(())
}
#[test]
fn test_spec_example_2_10_node_for_sammy_sosa_appears_twice_in_this_document_yml_prose_wrapalways_format_1_14681139(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("---\nhr:\n  - Mark McGwire\n  # Following node labeled SS\n  - &SS Sammy Sosa\nrbi:\n  - *SS # Subsequent occurrence\n  - Ken Griffey") ? ;
    assert_eq ! (formatted , "---\nhr:\n  - Mark McGwire\n  # Following node labeled SS\n  - &SS Sammy Sosa\nrbi:\n  - *SS # Subsequent occurrence\n  - Ken Griffey");
    Ok(())
}
#[test]
fn test_spec_example_2_10_node_for_sammy_sosa_appears_twice_in_this_document_yml_use_tabstrue_format_1_14681139(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("---\nhr:\n  - Mark McGwire\n  # Following node labeled SS\n  - &SS Sammy Sosa\nrbi:\n  - *SS # Subsequent occurrence\n  - Ken Griffey") ? ;
    assert_eq ! (formatted , "---\nhr:\n  - Mark McGwire\n  # Following node labeled SS\n  - &SS Sammy Sosa\nrbi:\n  - *SS # Subsequent occurrence\n  - Ken Griffey");
    Ok(())
}
#[test]
fn test_spec_example_2_11_mapping_between_sequences_yml_prose_wrapalways_format_1_652c6d10(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("? - Detroit Tigers\n  - Chicago cubs\n:\n  - 2001-07-23\n\n? [ New York Yankees,\n    Atlanta Braves ]\n: [ 2001-07-02, 2001-08-12,\n    2001-08-14 ]") ? ;
    assert_eq ! (formatted , "? - Detroit Tigers\n  - Chicago cubs\n: - 2001-07-23\n\n[New York Yankees, Atlanta Braves]: [2001-07-02, 2001-08-12, 2001-08-14]");
    Ok(())
}
#[test]
fn test_spec_example_2_11_mapping_between_sequences_yml_use_tabstrue_format_1_652c6d10(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("? - Detroit Tigers\n  - Chicago cubs\n:\n  - 2001-07-23\n\n? [ New York Yankees,\n    Atlanta Braves ]\n: [ 2001-07-02, 2001-08-12,\n    2001-08-14 ]") ? ;
    assert_eq ! (formatted , "? - Detroit Tigers\n  - Chicago cubs\n: - 2001-07-23\n\n[New York Yankees, Atlanta Braves]: [2001-07-02, 2001-08-12, 2001-08-14]");
    Ok(())
}
#[test]
fn test_spec_example_2_12_compact_nested_mapping_yml_prose_wrapalways_format_1_cfe2575f(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("---\n# Products purchased\n- item    : Super Hoop\n  quantity: 1\n- item    : Basketball\n  quantity: 4\n- item    : Big Shoes\n  quantity: 1") ? ;
    assert_eq ! (formatted , "---\n# Products purchased\n- item: Super Hoop\n  quantity: 1\n- item: Basketball\n  quantity: 4\n- item: Big Shoes\n  quantity: 1");
    Ok(())
}
#[test]
fn test_spec_example_2_12_compact_nested_mapping_yml_use_tabstrue_format_1_cfe2575f() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("---\n# Products purchased\n- item    : Super Hoop\n  quantity: 1\n- item    : Basketball\n  quantity: 4\n- item    : Big Shoes\n  quantity: 1") ? ;
    assert_eq ! (formatted , "---\n# Products purchased\n- item: Super Hoop\n  quantity: 1\n- item: Basketball\n  quantity: 4\n- item: Big Shoes\n  quantity: 1");
    Ok(())
}
#[test]
fn test_spec_example_2_13_in_literals_newlines_are_preserved_yml_prose_wrapalways_format_1_0f991bb4(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("# ASCII Art\n--- |\n  \\//||\\/||\n  // ||  ||__")?;
    assert_eq!(
        formatted,
        "# ASCII Art\n---\n|\n  \\//||\\/||\n  // ||  ||__"
    );
    Ok(())
}
#[test]
fn test_spec_example_2_13_in_literals_newlines_are_preserved_yml_use_tabstrue_format_1_0f991bb4(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("# ASCII Art\n--- |\n  \\//||\\/||\n  // ||  ||__")?;
    assert_eq!(
        formatted,
        "# ASCII Art\n---\n|\n  \\//||\\/||\n  // ||  ||__"
    );
    Ok(())
}
#[test]
fn test_spec_example_2_14_in_the_folded_scalars_newlines_become_spaces_yml_prose_wrapalways_format_1_ea00b733(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("--- >\n  Mark McGwire's\n  year was crippled\n  by a knee injury.")?;
    assert_eq!(
        formatted,
        "---\n>\n  Mark McGwire's year was crippled by a knee injury."
    );
    Ok(())
}
#[test]
fn test_spec_example_2_14_in_the_folded_scalars_newlines_become_spaces_yml_use_tabstrue_format_1_ea00b733(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("--- >\n  Mark McGwire's\n  year was crippled\n  by a knee injury.")?;
    assert_eq!(
        formatted,
        "---\n>\n  Mark McGwire's\n  year was crippled\n  by a knee injury."
    );
    Ok(())
}
#[test]
fn test_spec_example_2_15_folded_newlines_are_preserved_for_more_indented_and_blank_lines_yml_prose_wrapalways_format_1_479bbbd6(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format (">\n Sammy Sosa completed another\n fine season with great stats.\n\n   63 Home Runs\n   0.288 Batting Average\n\n What a year!") ? ;
    assert_eq ! (formatted , ">\n  Sammy Sosa completed another fine season with great stats.\n\n    63 Home Runs\n    0.288 Batting Average\n\n  What a year!");
    Ok(())
}
#[test]
fn test_spec_example_2_15_folded_newlines_are_preserved_for_more_indented_and_blank_lines_yml_use_tabstrue_format_1_479bbbd6(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format (">\n Sammy Sosa completed another\n fine season with great stats.\n\n   63 Home Runs\n   0.288 Batting Average\n\n What a year!") ? ;
    assert_eq ! (formatted , ">\n  Sammy Sosa completed another\n  fine season with great stats.\n\n    63 Home Runs\n    0.288 Batting Average\n\n  What a year!");
    Ok(())
}
#[test]
fn test_spec_example_2_16_indentation_determines_scope_yml_prose_wrapalways_format_1_d45be913(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("name: Mark McGwire\naccomplishment: >\n  Mark set a major league\n  home run record in 1998.\nstats: |\n  65 Home Runs\n  0.278 Batting Average") ? ;
    assert_eq ! (formatted , "name: Mark McGwire\naccomplishment: >\n  Mark set a major league home run record in 1998.\nstats: |\n  65 Home Runs\n  0.278 Batting Average");
    Ok(())
}
#[test]
fn test_spec_example_2_16_indentation_determines_scope_yml_use_tabstrue_format_1_d45be913(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("name: Mark McGwire\naccomplishment: >\n  Mark set a major league\n  home run record in 1998.\nstats: |\n  65 Home Runs\n  0.278 Batting Average") ? ;
    assert_eq ! (formatted , "name: Mark McGwire\naccomplishment: >\n  Mark set a major league\n  home run record in 1998.\nstats: |\n  65 Home Runs\n  0.278 Batting Average");
    Ok(())
}
#[test]
fn test_spec_example_2_17_quoted_scalars_yml_prose_wrapalways_format_1_e27c9de0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("unicode: \"Sosa did fine.\\u263A\"\ncontrol: \"\\b1998\\t1999\\t2000\\n\"\nhex esc: \"\\x0d\\x0a is \\r\\n\"\n\nsingle: '\"Howdy!\" he cried.'\nquoted: ' # Not a ''comment''.'\ntie-fighter: '|\\-*-/|'") ? ;
    assert_eq ! (formatted , "unicode: \"Sosa did fine.\\u263A\"\ncontrol: \"\\b1998\\t1999\\t2000\\n\"\nhex esc: \"\\x0d\\x0a is \\r\\n\"\n\nsingle: '\"Howdy!\" he cried.'\nquoted: \" # Not a 'comment'.\"\ntie-fighter: '|\\-*-/|'");
    Ok(())
}
#[test]
fn test_spec_example_2_17_quoted_scalars_yml_use_tabstrue_format_1_e27c9de0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("unicode: \"Sosa did fine.\\u263A\"\ncontrol: \"\\b1998\\t1999\\t2000\\n\"\nhex esc: \"\\x0d\\x0a is \\r\\n\"\n\nsingle: '\"Howdy!\" he cried.'\nquoted: ' # Not a ''comment''.'\ntie-fighter: '|\\-*-/|'") ? ;
    assert_eq ! (formatted , "unicode: \"Sosa did fine.\\u263A\"\ncontrol: \"\\b1998\\t1999\\t2000\\n\"\nhex esc: \"\\x0d\\x0a is \\r\\n\"\n\nsingle: '\"Howdy!\" he cried.'\nquoted: \" # Not a 'comment'.\"\ntie-fighter: '|\\-*-/|'");
    Ok(())
}
#[test]
fn test_spec_example_2_18_multi_line_flow_scalars_yml_prose_wrapalways_format_1_add8f696(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("plain:\n  This unquoted scalar\n  spans many lines.\n\nquoted: \"So does this\n  quoted scalar.\\n\"") ? ;
    assert_eq ! (formatted , "plain: This unquoted scalar spans many lines.\n\nquoted: \"So does this quoted scalar.\\n\"");
    Ok(())
}
#[test]
fn test_spec_example_2_18_multi_line_flow_scalars_yml_use_tabstrue_format_1_add8f696() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("plain:\n  This unquoted scalar\n  spans many lines.\n\nquoted: \"So does this\n  quoted scalar.\\n\"") ? ;
    assert_eq ! (formatted , "plain: This unquoted scalar\n  spans many lines.\n\nquoted: \"So does this\n  quoted scalar.\\n\"");
    Ok(())
}
#[test]
fn test_spec_example_2_23_various_explicit_tags_yml_prose_wrapalways_format_1_2f108a22(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("---\nnot-date: !!str 2002-04-28\n\npicture: !!binary |\n R0lGODlhDAAMAIQAAP//9/X\n 17unp5WZmZgAAAOfn515eXv\n Pz7Y6OjuDg4J+fn5OTk6enp\n 56enmleECcgggoBADs=\n\napplication specific tag: !something |\n The semantics of the tag\n above may be different for\n different documents.") ? ;
    assert_eq ! (formatted , "---\nnot-date: !!str 2002-04-28\n\npicture: !!binary |\n  R0lGODlhDAAMAIQAAP//9/X\n  17unp5WZmZgAAAOfn515eXv\n  Pz7Y6OjuDg4J+fn5OTk6enp\n  56enmleECcgggoBADs=\n\napplication specific tag: !something |\n  The semantics of the tag\n  above may be different for\n  different documents.");
    Ok(())
}
#[test]
fn test_spec_example_2_23_various_explicit_tags_yml_use_tabstrue_format_1_2f108a22() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("---\nnot-date: !!str 2002-04-28\n\npicture: !!binary |\n R0lGODlhDAAMAIQAAP//9/X\n 17unp5WZmZgAAAOfn515eXv\n Pz7Y6OjuDg4J+fn5OTk6enp\n 56enmleECcgggoBADs=\n\napplication specific tag: !something |\n The semantics of the tag\n above may be different for\n different documents.") ? ;
    assert_eq ! (formatted , "---\nnot-date: !!str 2002-04-28\n\npicture: !!binary |\n  R0lGODlhDAAMAIQAAP//9/X\n  17unp5WZmZgAAAOfn515eXv\n  Pz7Y6OjuDg4J+fn5OTk6enp\n  56enmleECcgggoBADs=\n\napplication specific tag: !something |\n  The semantics of the tag\n  above may be different for\n  different documents.");
    Ok(())
}
#[test]
fn test_spec_example_2_24_global_tags_yml_prose_wrapalways_format_1_51d9dce9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("%TAG ! tag:clarkevans.com,2002:\n--- !shape\n  # Use the ! handle for presenting\n  # tag:clarkevans.com,2002:circle\n- !circle\n  center: &ORIGIN {x: 73, y: 129}\n  radius: 7\n- !line\n  start: *ORIGIN\n  finish: { x: 89, y: 102 }\n- !label\n  start: *ORIGIN\n  color: 0xFFEEBB\n  text: Pretty vector drawing.") ? ;
    assert_eq ! (formatted , "%TAG ! tag:clarkevans.com,2002:\n---\n!shape\n# Use the ! handle for presenting\n# tag:clarkevans.com,2002:circle\n- !circle\n  center: &ORIGIN { x: 73, y: 129 }\n  radius: 7\n- !line\n  start: *ORIGIN\n  finish: { x: 89, y: 102 }\n- !label\n  start: *ORIGIN\n  color: 0xFFEEBB\n  text: Pretty vector drawing.");
    Ok(())
}
#[test]
fn test_spec_example_2_24_global_tags_yml_use_tabstrue_format_1_51d9dce9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("%TAG ! tag:clarkevans.com,2002:\n--- !shape\n  # Use the ! handle for presenting\n  # tag:clarkevans.com,2002:circle\n- !circle\n  center: &ORIGIN {x: 73, y: 129}\n  radius: 7\n- !line\n  start: *ORIGIN\n  finish: { x: 89, y: 102 }\n- !label\n  start: *ORIGIN\n  color: 0xFFEEBB\n  text: Pretty vector drawing.") ? ;
    assert_eq ! (formatted , "%TAG ! tag:clarkevans.com,2002:\n---\n!shape\n# Use the ! handle for presenting\n# tag:clarkevans.com,2002:circle\n- !circle\n  center: &ORIGIN { x: 73, y: 129 }\n  radius: 7\n- !line\n  start: *ORIGIN\n  finish: { x: 89, y: 102 }\n- !label\n  start: *ORIGIN\n  color: 0xFFEEBB\n  text: Pretty vector drawing.");
    Ok(())
}
#[test]
fn test_spec_example_2_25_unordered_sets_yml_prose_wrapalways_format_1_f597d76e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("# Sets are represented as a\n# Mapping where each key is\n# associated with a null value\n--- !!set\n? Mark McGwire\n? Sammy Sosa\n? Ken Griff") ? ;
    assert_eq ! (formatted , "# Sets are represented as a\n# Mapping where each key is\n# associated with a null value\n---\n!!set\n? Mark McGwire\n? Sammy Sosa\n? Ken Griff");
    Ok(())
}
#[test]
fn test_spec_example_2_25_unordered_sets_yml_use_tabstrue_format_1_f597d76e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("# Sets are represented as a\n# Mapping where each key is\n# associated with a null value\n--- !!set\n? Mark McGwire\n? Sammy Sosa\n? Ken Griff") ? ;
    assert_eq ! (formatted , "# Sets are represented as a\n# Mapping where each key is\n# associated with a null value\n---\n!!set\n? Mark McGwire\n? Sammy Sosa\n? Ken Griff");
    Ok(())
}
#[test]
fn test_spec_example_2_26_ordered_mappings_yml_prose_wrapalways_format_1_4837c909() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("# Ordered maps are represented as\n# A sequence of mappings, with\n# each mapping having one key\n--- !!omap\n- Mark McGwire: 65\n- Sammy Sosa: 63\n- Ken Griffy: 58") ? ;
    assert_eq ! (formatted , "# Ordered maps are represented as\n# A sequence of mappings, with\n# each mapping having one key\n---\n!!omap\n- Mark McGwire: 65\n- Sammy Sosa: 63\n- Ken Griffy: 58");
    Ok(())
}
#[test]
fn test_spec_example_2_26_ordered_mappings_yml_use_tabstrue_format_1_4837c909() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("# Ordered maps are represented as\n# A sequence of mappings, with\n# each mapping having one key\n--- !!omap\n- Mark McGwire: 65\n- Sammy Sosa: 63\n- Ken Griffy: 58") ? ;
    assert_eq ! (formatted , "# Ordered maps are represented as\n# A sequence of mappings, with\n# each mapping having one key\n---\n!!omap\n- Mark McGwire: 65\n- Sammy Sosa: 63\n- Ken Griffy: 58");
    Ok(())
}
#[test]
fn test_spec_example_2_27_invoice_yml_prose_wrapalways_format_1_8c45c1b0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("--- !<tag:clarkevans.com,2002:invoice>\ninvoice: 34843\ndate   : 2001-01-23\nbill-to: &id001\n    given  : Chris\n    family : Dumars\n    address:\n        lines: |\n            458 Walkman Dr.\n            Suite #292\n        city    : Royal Oak\n        state   : MI\n        postal  : 48046\nship-to: *id001\nproduct:\n    - sku         : BL394D\n      quantity    : 4\n      description : Basketball\n      price       : 450.00\n    - sku         : BL4438H\n      quantity    : 1\n      description : Super Hoop\n      price       : 2392.00\ntax  : 251.42\ntotal: 4443.52\ncomments:\n    Late afternoon is best.\n    Backup contact is Nancy\n    Billsmer @ 338-4338.") ? ;
    assert_eq ! (formatted , "---\n!<tag:clarkevans.com,2002:invoice>\ninvoice: 34843\ndate: 2001-01-23\nbill-to: &id001\n  given: Chris\n  family: Dumars\n  address:\n    lines: |\n      458 Walkman Dr.\n      Suite #292\n    city: Royal Oak\n    state: MI\n    postal: 48046\nship-to: *id001\nproduct:\n  - sku: BL394D\n    quantity: 4\n    description: Basketball\n    price: 450.00\n  - sku: BL4438H\n    quantity: 1\n    description: Super Hoop\n    price: 2392.00\ntax: 251.42\ntotal: 4443.52\ncomments: Late afternoon is best. Backup contact is Nancy Billsmer @ 338-4338.");
    Ok(())
}
#[test]
fn test_spec_example_2_27_invoice_yml_use_tabstrue_format_1_8c45c1b0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("--- !<tag:clarkevans.com,2002:invoice>\ninvoice: 34843\ndate   : 2001-01-23\nbill-to: &id001\n    given  : Chris\n    family : Dumars\n    address:\n        lines: |\n            458 Walkman Dr.\n            Suite #292\n        city    : Royal Oak\n        state   : MI\n        postal  : 48046\nship-to: *id001\nproduct:\n    - sku         : BL394D\n      quantity    : 4\n      description : Basketball\n      price       : 450.00\n    - sku         : BL4438H\n      quantity    : 1\n      description : Super Hoop\n      price       : 2392.00\ntax  : 251.42\ntotal: 4443.52\ncomments:\n    Late afternoon is best.\n    Backup contact is Nancy\n    Billsmer @ 338-4338.") ? ;
    assert_eq ! (formatted , "---\n!<tag:clarkevans.com,2002:invoice>\ninvoice: 34843\ndate: 2001-01-23\nbill-to: &id001\n  given: Chris\n  family: Dumars\n  address:\n    lines: |\n      458 Walkman Dr.\n      Suite #292\n    city: Royal Oak\n    state: MI\n    postal: 48046\nship-to: *id001\nproduct:\n  - sku: BL394D\n    quantity: 4\n    description: Basketball\n    price: 450.00\n  - sku: BL4438H\n    quantity: 1\n    description: Super Hoop\n    price: 2392.00\ntax: 251.42\ntotal: 4443.52\ncomments: Late afternoon is best.\n  Backup contact is Nancy\n  Billsmer @ 338-4338.");
    Ok(())
}
#[test]
fn test_spec_example_2_28_log_file_yml_prose_wrapalways_format_1_1aef01c4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("---\nTime: 2001-11-23 15:01:42 -5\nUser: ed\nWarning:\n  This is an error message\n  for the log file\n---\nTime: 2001-11-23 15:02:31 -5\nUser: ed\nWarning:\n  A slightly different error\n  message.\n---\nDate: 2001-11-23 15:03:17 -5\nUser: ed\nFatal:\n  Unknown variable \"bar\"\nStack:\n  - file: TopClass.py\n    line: 23\n    code: |\n      x = MoreObject(\"345\\n\")\n  - file: MoreClass.py\n    line: 58\n    code: |-\n      foo = bar") ? ;
    assert_eq ! (formatted , "---\nTime: 2001-11-23 15:01:42 -5\nUser: ed\nWarning: This is an error message for the log file\n---\nTime: 2001-11-23 15:02:31 -5\nUser: ed\nWarning: A slightly different error message.\n---\nDate: 2001-11-23 15:03:17 -5\nUser: ed\nFatal: Unknown variable \"bar\"\nStack:\n  - file: TopClass.py\n    line: 23\n    code: |\n      x = MoreObject(\"345\\n\")\n  - file: MoreClass.py\n    line: 58\n    code: |-\n      foo = bar");
    Ok(())
}
#[test]
fn test_spec_example_2_28_log_file_yml_use_tabstrue_format_1_1aef01c4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("---\nTime: 2001-11-23 15:01:42 -5\nUser: ed\nWarning:\n  This is an error message\n  for the log file\n---\nTime: 2001-11-23 15:02:31 -5\nUser: ed\nWarning:\n  A slightly different error\n  message.\n---\nDate: 2001-11-23 15:03:17 -5\nUser: ed\nFatal:\n  Unknown variable \"bar\"\nStack:\n  - file: TopClass.py\n    line: 23\n    code: |\n      x = MoreObject(\"345\\n\")\n  - file: MoreClass.py\n    line: 58\n    code: |-\n      foo = bar") ? ;
    assert_eq ! (formatted , "---\nTime: 2001-11-23 15:01:42 -5\nUser: ed\nWarning: This is an error message\n  for the log file\n---\nTime: 2001-11-23 15:02:31 -5\nUser: ed\nWarning: A slightly different error\n  message.\n---\nDate: 2001-11-23 15:03:17 -5\nUser: ed\nFatal: Unknown variable \"bar\"\nStack:\n  - file: TopClass.py\n    line: 23\n    code: |\n      x = MoreObject(\"345\\n\")\n  - file: MoreClass.py\n    line: 58\n    code: |-\n      foo = bar");
    Ok(())
}
#[test]
fn test_spec_example_5_3_block_structure_indicators_yml_prose_wrapalways_format_1_acc5f2ee(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("sequence:\n- one\n- two\nmapping:\n  ? sky\n  : blue\n  sea : green")?;
    assert_eq!(
        formatted,
        "sequence:\n  - one\n  - two\nmapping:\n  sky: blue\n  sea: green"
    );
    Ok(())
}
#[test]
fn test_spec_example_5_3_block_structure_indicators_yml_use_tabstrue_format_1_acc5f2ee(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("sequence:\n- one\n- two\nmapping:\n  ? sky\n  : blue\n  sea : green")?;
    assert_eq!(
        formatted,
        "sequence:\n  - one\n  - two\nmapping:\n  sky: blue\n  sea: green"
    );
    Ok(())
}
#[test]
fn test_spec_example_5_4_flow_collection_indicators_yml_prose_wrapalways_format_1_c783d48b(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("sequence: [ one, two, ]\nmapping: { sky: blue, sea: green }")?;
    assert_eq!(
        formatted,
        "sequence: [one, two]\nmapping: { sky: blue, sea: green }"
    );
    Ok(())
}
#[test]
fn test_spec_example_5_4_flow_collection_indicators_yml_use_tabstrue_format_1_c783d48b(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("sequence: [ one, two, ]\nmapping: { sky: blue, sea: green }")?;
    assert_eq!(
        formatted,
        "sequence: [one, two]\nmapping: { sky: blue, sea: green }"
    );
    Ok(())
}
#[test]
fn test_spec_example_5_5_comment_indicator_yml_prose_wrapalways_format_1_b3a97720() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("# Comment only.")?;
    assert_eq!(formatted, "# Comment only.");
    Ok(())
}
#[test]
fn test_spec_example_5_5_comment_indicator_yml_use_tabstrue_format_1_b3a97720() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("# Comment only.")?;
    assert_eq!(formatted, "# Comment only.");
    Ok(())
}
#[test]
fn test_spec_example_5_6_node_property_indicators_yml_prose_wrapalways_format_1_9548acaf(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("anchored: !local &anchor value\nalias: *anchor")?;
    assert_eq!(formatted, "anchored: !local &anchor value\nalias: *anchor");
    Ok(())
}
#[test]
fn test_spec_example_5_6_node_property_indicators_yml_use_tabstrue_format_1_9548acaf() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("anchored: !local &anchor value\nalias: *anchor")?;
    assert_eq!(formatted, "anchored: !local &anchor value\nalias: *anchor");
    Ok(())
}
#[test]
fn test_spec_example_5_7_block_scalar_indicators_yml_prose_wrapalways_format_1_780db99c(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("literal: |\n  some\n  text\nfolded: >\n  some\n  text")?;
    assert_eq!(
        formatted,
        "literal: |\n  some\n  text\nfolded: >\n  some text"
    );
    Ok(())
}
#[test]
fn test_spec_example_5_7_block_scalar_indicators_yml_use_tabstrue_format_1_780db99c() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("literal: |\n  some\n  text\nfolded: >\n  some\n  text")?;
    assert_eq!(
        formatted,
        "literal: |\n  some\n  text\nfolded: >\n  some\n  text"
    );
    Ok(())
}
#[test]
fn test_spec_example_5_8_quoted_scalar_indicators_yml_prose_wrapalways_format_1_9981e54d(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("single: 'text'\ndouble: \"text\"")?;
    assert_eq!(formatted, "single: \"text\"\ndouble: \"text\"");
    Ok(())
}
#[test]
fn test_spec_example_5_8_quoted_scalar_indicators_yml_use_tabstrue_format_1_9981e54d() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("single: 'text'\ndouble: \"text\"")?;
    assert_eq!(formatted, "single: \"text\"\ndouble: \"text\"");
    Ok(())
}
#[test]
fn test_spec_example_5_9_directive_indicator_yml_prose_wrapalways_format_1_85010331() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("%YAML 1.2\n--- text")?;
    assert_eq!(formatted, "%YAML 1.2\n---\ntext");
    Ok(())
}
#[test]
fn test_spec_example_5_9_directive_indicator_yml_use_tabstrue_format_1_85010331() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("%YAML 1.2\n--- text")?;
    assert_eq!(formatted, "%YAML 1.2\n---\ntext");
    Ok(())
}
#[test]
fn test_spec_example_5_12_tabs_and_spaces_yml_prose_wrapalways_format_1_aabf9fff() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("# Tabs and spaces\nquoted: \"Quoted \t\"\nblock:\t|\n  void main() {\n  \tprintf(\"Hello, world!\\n\");\n  }") ? ;
    assert_eq ! (formatted , "# Tabs and spaces\nquoted: \"Quoted \t\"\nblock: |\n  void main() {\n  \tprintf(\"Hello, world!\\n\");\n  }");
    Ok(())
}
#[test]
fn test_spec_example_5_12_tabs_and_spaces_yml_use_tabstrue_format_1_aabf9fff() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("# Tabs and spaces\nquoted: \"Quoted \t\"\nblock:\t|\n  void main() {\n  \tprintf(\"Hello, world!\\n\");\n  }") ? ;
    assert_eq ! (formatted , "# Tabs and spaces\nquoted: \"Quoted \t\"\nblock: |\n  void main() {\n  \tprintf(\"Hello, world!\\n\");\n  }");
    Ok(())
}
#[test]
fn test_spec_example_6_1_indentation_spaces_yml_prose_wrapalways_format_1_02c03d21() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  # Leading comment line spaces are\n   # neither content nor indentation.\n    \nNot indented:\n By one space: |\n    By four\n      spaces\n Flow style: [    # Leading spaces\n   By two,        # in flow style\n  Also by two,    # are neither\n  \tStill by two   # content nor\n    ]             # indentation.") ? ;
    assert_eq ! (formatted , "# Leading comment line spaces are\n# neither content nor indentation.\n\nNot indented:\n  By one space: |\n    By four\n      spaces\n  Flow style: # Leading spaces\n    [\n      By two, # in flow style\n      Also by two, # are neither\n      Still by two, # content nor\n    ] # indentation.");
    Ok(())
}
#[test]
fn test_spec_example_6_1_indentation_spaces_yml_use_tabstrue_format_1_02c03d21() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  # Leading comment line spaces are\n   # neither content nor indentation.\n    \nNot indented:\n By one space: |\n    By four\n      spaces\n Flow style: [    # Leading spaces\n   By two,        # in flow style\n  Also by two,    # are neither\n  \tStill by two   # content nor\n    ]             # indentation.") ? ;
    assert_eq ! (formatted , "# Leading comment line spaces are\n# neither content nor indentation.\n\nNot indented:\n  By one space: |\n    By four\n      spaces\n  Flow style: # Leading spaces\n    [\n      By two, # in flow style\n      Also by two, # are neither\n      Still by two, # content nor\n    ] # indentation.");
    Ok(())
}
#[test]
fn test_spec_example_6_2_indentation_indicators_yml_prose_wrapalways_format_1_53da9a4b(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("? a\n: -\tb\n  -  -\tc\n     - d")?;
    assert_eq!(formatted, "a:\n  - b\n  - - c\n    - d");
    Ok(())
}
#[test]
fn test_spec_example_6_2_indentation_indicators_yml_use_tabstrue_format_1_53da9a4b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("? a\n: -\tb\n  -  -\tc\n     - d")?;
    assert_eq!(formatted, "a:\n  - b\n  - - c\n    - d");
    Ok(())
}
#[test]
fn test_spec_example_6_3_separation_spaces_yml_prose_wrapalways_format_1_b257510e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- foo:\t bar\n- - baz\n  -\tbaz")?;
    assert_eq!(formatted, "- foo: bar\n- - baz\n  - baz");
    Ok(())
}
#[test]
fn test_spec_example_6_3_separation_spaces_yml_use_tabstrue_format_1_b257510e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- foo:\t bar\n- - baz\n  -\tbaz")?;
    assert_eq!(formatted, "- foo: bar\n- - baz\n  - baz");
    Ok(())
}
#[test]
fn test_spec_example_6_4_line_prefixes_yml_prose_wrapalways_format_1_197a4067() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "plain: text\n  lines\nquoted: \"text\n  \tlines\"\nblock: |\n  text\n   \tlines",
    )?;
    assert_eq!(
        formatted,
        "plain: text lines\nquoted: \"text lines\"\nblock: |\n  text\n   \tlines"
    );
    Ok(())
}
#[test]
fn test_spec_example_6_4_line_prefixes_yml_use_tabstrue_format_1_197a4067() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "plain: text\n  lines\nquoted: \"text\n  \tlines\"\nblock: |\n  text\n   \tlines",
    )?;
    assert_eq!(
        formatted,
        "plain: text\n  lines\nquoted: \"text\n  lines\"\nblock: |\n  text\n   \tlines"
    );
    Ok(())
}
#[test]
fn test_spec_example_6_5_empty_lines_yml_prose_wrapalways_format_1_8280f655() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("Folding:\n  \"Empty line\n   \t\n  as a line feed\"\nChomping: |\n  Clipped empty lines\n \n") ? ;
    assert_eq!(
        formatted,
        "Folding: \"Empty line\n\n  as a line feed\"\nChomping: |\n  Clipped empty lines"
    );
    Ok(())
}
#[test]
fn test_spec_example_6_5_empty_lines_yml_use_tabstrue_format_1_8280f655() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("Folding:\n  \"Empty line\n   \t\n  as a line feed\"\nChomping: |\n  Clipped empty lines\n \n") ? ;
    assert_eq!(
        formatted,
        "Folding: \"Empty line\n\n  as a line feed\"\nChomping: |\n  Clipped empty lines"
    );
    Ok(())
}
#[test]
fn test_spec_example_6_6_line_folding_yml_prose_wrapalways_format_1_1f08e0c5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(">-\n  trimmed\n  \n \n\n  as\n  space")?;
    assert_eq!(formatted, ">-\n  trimmed\n\n\n\n  as space");
    Ok(())
}
#[test]
fn test_spec_example_6_6_line_folding_yml_use_tabstrue_format_1_1f08e0c5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(">-\n  trimmed\n  \n \n\n  as\n  space")?;
    assert_eq!(formatted, ">-\n  trimmed\n\n\n\n  as\n  space");
    Ok(())
}
#[test]
fn test_spec_example_6_7_block_folding_yml_prose_wrapalways_format_1_721da24a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(">\n  foo \n \n  \t bar\n\n  baz")?;
    assert_eq!(formatted, ">\n  foo \n\n  \t bar\n\n  baz");
    Ok(())
}
#[test]
fn test_spec_example_6_7_block_folding_yml_use_tabstrue_format_1_721da24a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(">\n  foo \n \n  \t bar\n\n  baz")?;
    assert_eq!(formatted, ">\n  foo \n\n  \t bar\n\n  baz");
    Ok(())
}
#[test]
fn test_spec_example_6_8_flow_folding_yml_prose_wrapalways_format_1_599d8e66() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\"\n  foo \n \n  \t bar\n\n  baz\n\"")?;
    assert_eq!(formatted, "\"\nfoo\n\nbar\n\nbaz\n\"");
    Ok(())
}
#[test]
fn test_spec_example_6_8_flow_folding_yml_use_tabstrue_format_1_599d8e66() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\"\n  foo \n \n  \t bar\n\n  baz\n\"")?;
    assert_eq!(formatted, "\"\nfoo\n\nbar\n\nbaz\n\"");
    Ok(())
}
#[test]
fn test_spec_example_6_9_separated_comment_yml_prose_wrapalways_format_1_83e9f7c1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("key:    # Comment\n  value")?;
    assert_eq!(formatted, "key: # Comment\n  value");
    Ok(())
}
#[test]
fn test_spec_example_6_9_separated_comment_yml_use_tabstrue_format_1_83e9f7c1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("key:    # Comment\n  value")?;
    assert_eq!(formatted, "key: # Comment\n  value");
    Ok(())
}
#[test]
fn test_spec_example_6_10_comment_lines_yml_prose_wrapalways_format_1_000deb0c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("  # Comment\n   \n\n")?;
    assert_eq!(formatted, "# Comment");
    Ok(())
}
#[test]
fn test_spec_example_6_10_comment_lines_yml_use_tabstrue_format_1_000deb0c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("  # Comment\n   \n\n")?;
    assert_eq!(formatted, "# Comment");
    Ok(())
}
#[test]
fn test_spec_example_6_11_multi_line_comments_yml_prose_wrapalways_format_1_50ad5c62() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("key:    # Comment\n        # lines\n  value\n\n")?;
    assert_eq!(formatted, "key: # Comment\n  # lines\n  value");
    Ok(())
}
#[test]
fn test_spec_example_6_11_multi_line_comments_yml_use_tabstrue_format_1_50ad5c62() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("key:    # Comment\n        # lines\n  value\n\n")?;
    assert_eq!(formatted, "key: # Comment\n  # lines\n  value");
    Ok(())
}
#[test]
fn test_spec_example_6_12_separation_spaces_yml_prose_wrapalways_format_1_e0698efc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{ first: Sammy, last: Sosa }:\n# Statistics:\n  hr:  # Home runs\n     65\n  avg: # Average\n   0.278") ? ;
    assert_eq ! (formatted , "{ first: Sammy, last: Sosa }:\n  # Statistics:\n  hr: # Home runs\n    65\n  avg: # Average\n    0.278");
    Ok(())
}
#[test]
fn test_spec_example_6_12_separation_spaces_yml_use_tabstrue_format_1_e0698efc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{ first: Sammy, last: Sosa }:\n# Statistics:\n  hr:  # Home runs\n     65\n  avg: # Average\n   0.278") ? ;
    assert_eq ! (formatted , "{ first: Sammy, last: Sosa }:\n  # Statistics:\n  hr: # Home runs\n    65\n  avg: # Average\n    0.278");
    Ok(())
}
#[test]
fn test_spec_example_6_13_reserved_directives_yml_prose_wrapalways_format_1_2cffa122() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "%FOO  bar baz # Should be ignored\n              # with a warning.\n--- \"foo\"",
    )?;
    assert_eq!(
        formatted,
        "%FOO bar baz # Should be ignored\n# with a warning.\n---\n\"foo\""
    );
    Ok(())
}
#[test]
fn test_spec_example_6_13_reserved_directives_yml_use_tabstrue_format_1_2cffa122() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "%FOO  bar baz # Should be ignored\n              # with a warning.\n--- \"foo\"",
    )?;
    assert_eq!(
        formatted,
        "%FOO bar baz # Should be ignored\n# with a warning.\n---\n\"foo\""
    );
    Ok(())
}
#[test]
fn test_spec_example_6_14_yaml_directive_yml_prose_wrapalways_format_1_50836328() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("%YAML 1.3 # Attempt parsing\n          # with a warning\n---\n\"foo\"")?;
    assert_eq!(
        formatted,
        "%YAML 1.3 # Attempt parsing\n# with a warning\n---\n\"foo\""
    );
    Ok(())
}
#[test]
fn test_spec_example_6_14_yaml_directive_yml_use_tabstrue_format_1_50836328() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("%YAML 1.3 # Attempt parsing\n          # with a warning\n---\n\"foo\"")?;
    assert_eq!(
        formatted,
        "%YAML 1.3 # Attempt parsing\n# with a warning\n---\n\"foo\""
    );
    Ok(())
}
#[test]
fn test_spec_example_6_16_tag_directive_yml_prose_wrapalways_format_1_cf1f9bda() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("%TAG !yaml! tag:yaml.org,2002:\n---\n!yaml!str \"foo\"")?;
    assert_eq!(
        formatted,
        "%TAG !yaml! tag:yaml.org,2002:\n---\n!yaml!str \"foo\""
    );
    Ok(())
}
#[test]
fn test_spec_example_6_16_tag_directive_yml_use_tabstrue_format_1_cf1f9bda() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("%TAG !yaml! tag:yaml.org,2002:\n---\n!yaml!str \"foo\"")?;
    assert_eq!(
        formatted,
        "%TAG !yaml! tag:yaml.org,2002:\n---\n!yaml!str \"foo\""
    );
    Ok(())
}
#[test]
fn test_spec_example_6_18_primary_tag_handle_yml_prose_wrapalways_format_1_c1560875() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("# Private\n!foo \"bar\"\n...\n# Global\n%TAG ! tag:example.com,2000:app/\n---\n!foo \"bar\"") ? ;
    assert_eq ! (formatted , "# Private\n!foo \"bar\"\n...\n# Global\n%TAG ! tag:example.com,2000:app/\n---\n!foo \"bar\"");
    Ok(())
}
#[test]
fn test_spec_example_6_18_primary_tag_handle_yml_use_tabstrue_format_1_c1560875() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("# Private\n!foo \"bar\"\n...\n# Global\n%TAG ! tag:example.com,2000:app/\n---\n!foo \"bar\"") ? ;
    assert_eq ! (formatted , "# Private\n!foo \"bar\"\n...\n# Global\n%TAG ! tag:example.com,2000:app/\n---\n!foo \"bar\"");
    Ok(())
}
#[test]
fn test_spec_example_6_19_secondary_tag_handle_yml_prose_wrapalways_format_1_50c7141f() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("%TAG !! tag:example.com,2000:app/\n---\n!!int 1 - 3 # Interval, not integer")?;
    assert_eq!(
        formatted,
        "%TAG !! tag:example.com,2000:app/\n---\n!!int 1 - 3 # Interval, not integer"
    );
    Ok(())
}
#[test]
fn test_spec_example_6_19_secondary_tag_handle_yml_use_tabstrue_format_1_50c7141f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("%TAG !! tag:example.com,2000:app/\n---\n!!int 1 - 3 # Interval, not integer")?;
    assert_eq!(
        formatted,
        "%TAG !! tag:example.com,2000:app/\n---\n!!int 1 - 3 # Interval, not integer"
    );
    Ok(())
}
#[test]
fn test_spec_example_6_20_tag_handles_yml_prose_wrapalways_format_1_32e3ca8b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("%TAG !e! tag:example.com,2000:app/\n---\n!e!foo \"bar\"")?;
    assert_eq!(
        formatted,
        "%TAG !e! tag:example.com,2000:app/\n---\n!e!foo \"bar\""
    );
    Ok(())
}
#[test]
fn test_spec_example_6_20_tag_handles_yml_use_tabstrue_format_1_32e3ca8b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("%TAG !e! tag:example.com,2000:app/\n---\n!e!foo \"bar\"")?;
    assert_eq!(
        formatted,
        "%TAG !e! tag:example.com,2000:app/\n---\n!e!foo \"bar\""
    );
    Ok(())
}
#[test]
fn test_spec_example_6_21_local_tag_prefix_yml_prose_wrapalways_format_1_a915f092() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("%TAG !m! !my-\n--- # Bulb here\n!m!light fluorescent\n...\n%TAG !m! !my-\n--- # Color here\n!m!light green") ? ;
    assert_eq ! (formatted , "%TAG !m! !my-\n--- # Bulb here\n!m!light fluorescent\n...\n%TAG !m! !my-\n--- # Color here\n!m!light green");
    Ok(())
}
#[test]
fn test_spec_example_6_21_local_tag_prefix_yml_use_tabstrue_format_1_a915f092() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("%TAG !m! !my-\n--- # Bulb here\n!m!light fluorescent\n...\n%TAG !m! !my-\n--- # Color here\n!m!light green") ? ;
    assert_eq ! (formatted , "%TAG !m! !my-\n--- # Bulb here\n!m!light fluorescent\n...\n%TAG !m! !my-\n--- # Color here\n!m!light green");
    Ok(())
}
#[test]
fn test_spec_example_6_22_global_tag_prefix_yml_prose_wrapalways_format_1_5e5295da() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("%TAG !e! tag:example.com,2000:app/\n---\n- !e!foo \"bar\"")?;
    assert_eq!(
        formatted,
        "%TAG !e! tag:example.com,2000:app/\n---\n- !e!foo \"bar\""
    );
    Ok(())
}
#[test]
fn test_spec_example_6_22_global_tag_prefix_yml_use_tabstrue_format_1_5e5295da() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("%TAG !e! tag:example.com,2000:app/\n---\n- !e!foo \"bar\"")?;
    assert_eq!(
        formatted,
        "%TAG !e! tag:example.com,2000:app/\n---\n- !e!foo \"bar\""
    );
    Ok(())
}
#[test]
fn test_spec_example_6_23_node_properties_yml_prose_wrapalways_format_1_33d77bc7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!str &a1 \"foo\":\n  !!str bar\n&a2 baz : *a1")?;
    assert_eq!(formatted, "!!str &a1 \"foo\": !!str bar\n&a2 baz: *a1");
    Ok(())
}
#[test]
fn test_spec_example_6_23_node_properties_yml_use_tabstrue_format_1_33d77bc7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!str &a1 \"foo\":\n  !!str bar\n&a2 baz : *a1")?;
    assert_eq!(formatted, "!!str &a1 \"foo\": !!str bar\n&a2 baz: *a1");
    Ok(())
}
#[test]
fn test_spec_example_6_24_verbatim_tags_yml_prose_wrapalways_format_1_a95cc689() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!<tag:yaml.org,2002:str> foo :\n  !<!bar> baz")?;
    assert_eq!(formatted, "!<tag:yaml.org,2002:str> foo: !<!bar> baz");
    Ok(())
}
#[test]
fn test_spec_example_6_24_verbatim_tags_yml_use_tabstrue_format_1_a95cc689() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!<tag:yaml.org,2002:str> foo :\n  !<!bar> baz")?;
    assert_eq!(formatted, "!<tag:yaml.org,2002:str> foo: !<!bar> baz");
    Ok(())
}
#[test]
fn test_spec_example_6_26_tag_shorthands_yml_prose_wrapalways_format_1_3309653f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "%TAG !e! tag:example.com,2000:app/\n---\n- !local foo\n- !!str bar\n- !e!tag%21 baz",
    )?;
    assert_eq!(
        formatted,
        "%TAG !e! tag:example.com,2000:app/\n---\n- !local foo\n- !!str bar\n- !e!tag%21 baz"
    );
    Ok(())
}
#[test]
fn test_spec_example_6_26_tag_shorthands_yml_use_tabstrue_format_1_3309653f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "%TAG !e! tag:example.com,2000:app/\n---\n- !local foo\n- !!str bar\n- !e!tag%21 baz",
    )?;
    assert_eq!(
        formatted,
        "%TAG !e! tag:example.com,2000:app/\n---\n- !local foo\n- !!str bar\n- !e!tag%21 baz"
    );
    Ok(())
}
#[test]
fn test_spec_example_6_28_non_specific_tags_yml_prose_wrapalways_format_1_ed7d4482() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("# Assuming conventional resolution:\n- \"12\"\n- 12\n- ! 12")?;
    assert_eq!(
        formatted,
        "# Assuming conventional resolution:\n- \"12\"\n- 12\n- ! 12"
    );
    Ok(())
}
#[test]
fn test_spec_example_6_28_non_specific_tags_yml_use_tabstrue_format_1_ed7d4482() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("# Assuming conventional resolution:\n- \"12\"\n- 12\n- ! 12")?;
    assert_eq!(
        formatted,
        "# Assuming conventional resolution:\n- \"12\"\n- 12\n- ! 12"
    );
    Ok(())
}
#[test]
fn test_spec_example_6_29_node_anchors_yml_prose_wrapalways_format_1_8d25abbf() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("First occurrence: &anchor Value\nSecond occurrence: *anchor")?;
    assert_eq!(
        formatted,
        "First occurrence: &anchor Value\nSecond occurrence: *anchor"
    );
    Ok(())
}
#[test]
fn test_spec_example_6_29_node_anchors_yml_use_tabstrue_format_1_8d25abbf() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("First occurrence: &anchor Value\nSecond occurrence: *anchor")?;
    assert_eq!(
        formatted,
        "First occurrence: &anchor Value\nSecond occurrence: *anchor"
    );
    Ok(())
}
#[test]
fn test_spec_example_7_1_alias_nodes_yml_prose_wrapalways_format_1_5898cf5b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("First occurrence: &anchor Foo\nSecond occurrence: *anchor\nOverride anchor: &anchor Bar\nReuse anchor: *anchor") ? ;
    assert_eq ! (formatted , "First occurrence: &anchor Foo\nSecond occurrence: *anchor\nOverride anchor: &anchor Bar\nReuse anchor: *anchor");
    Ok(())
}
#[test]
fn test_spec_example_7_1_alias_nodes_yml_use_tabstrue_format_1_5898cf5b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("First occurrence: &anchor Foo\nSecond occurrence: *anchor\nOverride anchor: &anchor Bar\nReuse anchor: *anchor") ? ;
    assert_eq ! (formatted , "First occurrence: &anchor Foo\nSecond occurrence: *anchor\nOverride anchor: &anchor Bar\nReuse anchor: *anchor");
    Ok(())
}
#[test]
fn test_spec_example_7_2_empty_content_yml_prose_wrapalways_format_1_899d871d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{\n  foo : !!str,\n  !!str : bar,\n}")?;
    assert_eq!(formatted, "{ foo: !!str , !!str : bar }");
    Ok(())
}
#[test]
fn test_spec_example_7_2_empty_content_yml_use_tabstrue_format_1_899d871d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{\n  foo : !!str,\n  !!str : bar,\n}")?;
    assert_eq!(formatted, "{ foo: !!str , !!str : bar }");
    Ok(())
}
#[test]
fn test_spec_example_7_3_completely_empty_flow_nodes_yml_prose_wrapalways_format_1_f0c5b5ed(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{\n  ? foo :,\n  : bar,\n}")?;
    assert_eq!(formatted, "{ foo, : bar }");
    Ok(())
}
#[test]
fn test_spec_example_7_3_completely_empty_flow_nodes_yml_use_tabstrue_format_1_f0c5b5ed(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{\n  ? foo :,\n  : bar,\n}")?;
    assert_eq!(formatted, "{ foo, : bar }");
    Ok(())
}
#[test]
fn test_spec_example_7_4_double_quoted_implicit_keys_yml_prose_wrapalways_format_1_6771c930(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("\"implicit block key\" : [\n  \"implicit flow key\" : value,\n ]")?;
    assert_eq!(
        formatted,
        "\"implicit block key\": [\"implicit flow key\": value]"
    );
    Ok(())
}
#[test]
fn test_spec_example_7_4_double_quoted_implicit_keys_yml_use_tabstrue_format_1_6771c930(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("\"implicit block key\" : [\n  \"implicit flow key\" : value,\n ]")?;
    assert_eq!(
        formatted,
        "\"implicit block key\": [\"implicit flow key\": value]"
    );
    Ok(())
}
#[test]
fn test_spec_example_7_5_double_quoted_line_breaks_yml_prose_wrapalways_format_1_47b9d2dd(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("\"folded \nto a space,\t\n \nto a line feed, or \t\\\n \\ \tnon-content\"")?;
    assert_eq!(
        formatted,
        "\"folded to a space,\n\nto a line feed, or \t\\\n\\ \tnon-content\""
    );
    Ok(())
}
#[test]
fn test_spec_example_7_5_double_quoted_line_breaks_yml_use_tabstrue_format_1_47b9d2dd() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("\"folded \nto a space,\t\n \nto a line feed, or \t\\\n \\ \tnon-content\"")?;
    assert_eq!(
        formatted,
        "\"folded\nto a space,\n\nto a line feed, or \t\\\n\\ \tnon-content\""
    );
    Ok(())
}
#[test]
fn test_spec_example_7_6_double_quoted_lines_yml_prose_wrapalways_format_1_b152204e() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("\" 1st non-empty\n\n 2nd non-empty \n\t3rd non-empty \"")?;
    assert_eq!(
        formatted,
        "\" 1st non-empty\n\n2nd non-empty 3rd non-empty \""
    );
    Ok(())
}
#[test]
fn test_spec_example_7_6_double_quoted_lines_yml_use_tabstrue_format_1_b152204e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("\" 1st non-empty\n\n 2nd non-empty \n\t3rd non-empty \"")?;
    assert_eq!(
        formatted,
        "\" 1st non-empty\n\n2nd non-empty\n3rd non-empty \""
    );
    Ok(())
}
#[test]
fn test_spec_example_7_7_single_quoted_characters_yml_prose_wrapalways_format_1_596d6a4f(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("'here''s to \"quotes\"'")?;
    assert_eq!(formatted, "'here''s to \"quotes\"'");
    Ok(())
}
#[test]
fn test_spec_example_7_7_single_quoted_characters_yml_use_tabstrue_format_1_596d6a4f() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("'here''s to \"quotes\"'")?;
    assert_eq!(formatted, "'here''s to \"quotes\"'");
    Ok(())
}
#[test]
fn test_spec_example_7_8_single_quoted_implicit_keys_yml_prose_wrapalways_format_1_710257db(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("'implicit block key' : [\n  'implicit flow key' : value,\n ]")?;
    assert_eq!(
        formatted,
        "\"implicit block key\": [\"implicit flow key\": value]"
    );
    Ok(())
}
#[test]
fn test_spec_example_7_8_single_quoted_implicit_keys_yml_use_tabstrue_format_1_710257db(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("'implicit block key' : [\n  'implicit flow key' : value,\n ]")?;
    assert_eq!(
        formatted,
        "\"implicit block key\": [\"implicit flow key\": value]"
    );
    Ok(())
}
#[test]
fn test_spec_example_7_9_single_quoted_lines_yml_prose_wrapalways_format_1_13be5fd9() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("' 1st non-empty\n\n 2nd non-empty \n\t3rd non-empty '")?;
    assert_eq!(
        formatted,
        "\" 1st non-empty\n\n2nd non-empty 3rd non-empty \""
    );
    Ok(())
}
#[test]
fn test_spec_example_7_9_single_quoted_lines_yml_use_tabstrue_format_1_13be5fd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("' 1st non-empty\n\n 2nd non-empty \n\t3rd non-empty '")?;
    assert_eq!(
        formatted,
        "\" 1st non-empty\n\n2nd non-empty\n3rd non-empty \""
    );
    Ok(())
}
#[test]
fn test_spec_example_7_10_plain_characters_yml_prose_wrapalways_format_1_8802aabf() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("# Outside flow collection:\n- ::vector\n- \": - ()\"\n- Up, up, and away!\n- -123\n- http://example.com/foo#bar\n# Inside flow collection:\n- [ ::vector,\n  \": - ()\",\n  \"Up, up and away!\",\n  -123,\n  http://example.com/foo#bar ]") ? ;
    assert_eq ! (formatted , "# Outside flow collection:\n- ::vector\n- \": - ()\"\n- Up, up, and away!\n- -123\n- http://example.com/foo#bar\n# Inside flow collection:\n- [::vector, \": - ()\", \"Up, up and away!\", -123, http://example.com/foo#bar]");
    Ok(())
}
#[test]
fn test_spec_example_7_10_plain_characters_yml_use_tabstrue_format_1_8802aabf() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("# Outside flow collection:\n- ::vector\n- \": - ()\"\n- Up, up, and away!\n- -123\n- http://example.com/foo#bar\n# Inside flow collection:\n- [ ::vector,\n  \": - ()\",\n  \"Up, up and away!\",\n  -123,\n  http://example.com/foo#bar ]") ? ;
    assert_eq ! (formatted , "# Outside flow collection:\n- ::vector\n- \": - ()\"\n- Up, up, and away!\n- -123\n- http://example.com/foo#bar\n# Inside flow collection:\n- [::vector, \": - ()\", \"Up, up and away!\", -123, http://example.com/foo#bar]");
    Ok(())
}
#[test]
fn test_spec_example_7_11_plain_implicit_keys_yml_prose_wrapalways_format_1_c224e34d() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("implicit block key : [\n  implicit flow key : value,\n ]")?;
    assert_eq!(formatted, "implicit block key: [implicit flow key: value]");
    Ok(())
}
#[test]
fn test_spec_example_7_11_plain_implicit_keys_yml_use_tabstrue_format_1_c224e34d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("implicit block key : [\n  implicit flow key : value,\n ]")?;
    assert_eq!(formatted, "implicit block key: [implicit flow key: value]");
    Ok(())
}
#[test]
fn test_spec_example_7_12_plain_lines_yml_prose_wrapalways_format_1_2c723b01() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1st non-empty\n\n 2nd non-empty \n\t3rd non-empty")?;
    assert_eq!(formatted, "1st non-empty\n\n2nd non-empty 3rd non-empty");
    Ok(())
}
#[test]
fn test_spec_example_7_12_plain_lines_yml_use_tabstrue_format_1_2c723b01() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1st non-empty\n\n 2nd non-empty \n\t3rd non-empty")?;
    assert_eq!(formatted, "1st non-empty\n\n2nd non-empty\n3rd non-empty");
    Ok(())
}
#[test]
fn test_spec_example_7_13_flow_sequence_yml_prose_wrapalways_format_1_5ca44a5e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- [ one, two, ]\n- [three ,four]")?;
    assert_eq!(formatted, "- [one, two]\n- [three, four]");
    Ok(())
}
#[test]
fn test_spec_example_7_13_flow_sequence_yml_use_tabstrue_format_1_5ca44a5e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- [ one, two, ]\n- [three ,four]")?;
    assert_eq!(formatted, "- [one, two]\n- [three, four]");
    Ok(())
}
#[test]
fn test_spec_example_7_14_flow_sequence_entries_yml_prose_wrapalways_format_1_970c5881(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[\n\"double\n quoted\", 'single\n           quoted',\nplain\n text, [ nested ],\nsingle: pair,\n]") ? ;
    assert_eq!(
        formatted,
        "[\"double quoted\", \"single quoted\", plain text, [nested], single: pair]"
    );
    Ok(())
}
#[test]
fn test_spec_example_7_14_flow_sequence_entries_yml_use_tabstrue_format_1_970c5881() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[\n\"double\n quoted\", 'single\n           quoted',\nplain\n text, [ nested ],\nsingle: pair,\n]") ? ;
    assert_eq ! (formatted , "[\n  \"double\n  quoted\",\n  \"single\n  quoted\",\n  plain\n  text,\n  [nested],\n  single: pair,\n]");
    Ok(())
}
#[test]
fn test_spec_example_7_15_flow_mappings_yml_prose_wrapalways_format_1_d2f98c58() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("- { one : two , three: four , }\n- {five: six,seven : eight}")?;
    assert_eq!(
        formatted,
        "- { one: two, three: four }\n- { five: six, seven: eight }"
    );
    Ok(())
}
#[test]
fn test_spec_example_7_15_flow_mappings_yml_use_tabstrue_format_1_d2f98c58() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("- { one : two , three: four , }\n- {five: six,seven : eight}")?;
    assert_eq!(
        formatted,
        "- { one: two, three: four }\n- { five: six, seven: eight }"
    );
    Ok(())
}
#[test]
fn test_spec_example_7_16_flow_mapping_entries_yml_prose_wrapalways_format_1_9b01d199() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{\n? explicit: entry,\nimplicit: entry,\n?\n}")?;
    assert_eq!(formatted, "{ explicit: entry, implicit: entry, : }");
    Ok(())
}
#[test]
fn test_spec_example_7_16_flow_mapping_entries_yml_use_tabstrue_format_1_9b01d199() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{\n? explicit: entry,\nimplicit: entry,\n?\n}")?;
    assert_eq!(formatted, "{ explicit: entry, implicit: entry, : }");
    Ok(())
}
#[test]
fn test_spec_example_7_17_flow_mapping_separate_values_yml_prose_wrapalways_format_1_963bacd8(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "{\nunquoted : \"separate\",\nhttp://foo.com,\nomitted value:,\n: omitted key,\n}",
    )?;
    assert_eq!(
        formatted,
        "{ unquoted: \"separate\", http://foo.com, omitted value, : omitted key }"
    );
    Ok(())
}
#[test]
fn test_spec_example_7_17_flow_mapping_separate_values_yml_use_tabstrue_format_1_963bacd8(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "{\nunquoted : \"separate\",\nhttp://foo.com,\nomitted value:,\n: omitted key,\n}",
    )?;
    assert_eq!(
        formatted,
        "{ unquoted: \"separate\", http://foo.com, omitted value, : omitted key }"
    );
    Ok(())
}
#[test]
fn test_spec_example_7_18_flow_mapping_adjacent_values_yml_prose_wrapalways_format_1_57b71a8c(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("{\n\"adjacent\":value,\n\"readable\": value,\n\"empty\":\n}")?;
    assert_eq!(
        formatted,
        "{ \"adjacent\": value, \"readable\": value, \"empty\" }"
    );
    Ok(())
}
#[test]
fn test_spec_example_7_18_flow_mapping_adjacent_values_yml_use_tabstrue_format_1_57b71a8c(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("{\n\"adjacent\":value,\n\"readable\": value,\n\"empty\":\n}")?;
    assert_eq!(
        formatted,
        "{ \"adjacent\": value, \"readable\": value, \"empty\" }"
    );
    Ok(())
}
#[test]
fn test_spec_example_7_19_single_pair_flow_mappings_yml_prose_wrapalways_format_1_4798ef1a(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[\nfoo: bar\n]")?;
    assert_eq!(formatted, "[foo: bar]");
    Ok(())
}
#[test]
fn test_spec_example_7_19_single_pair_flow_mappings_yml_use_tabstrue_format_1_4798ef1a(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[\nfoo: bar\n]")?;
    assert_eq!(formatted, "[foo: bar]");
    Ok(())
}
#[test]
fn test_spec_example_7_20_single_pair_explicit_entry_yml_prose_wrapalways_format_1_d5f08b8c(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[\n? foo\n bar : baz\n]")?;
    assert_eq!(formatted, "[foo bar: baz]");
    Ok(())
}
#[test]
fn test_spec_example_7_20_single_pair_explicit_entry_yml_use_tabstrue_format_1_d5f08b8c(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[\n? foo\n bar : baz\n]")?;
    assert_eq!(formatted, "[? foo\n    bar\n  : baz]");
    Ok(())
}
#[test]
fn test_spec_example_7_21_single_pair_implicit_entries_yml_prose_wrapalways_format_1_893e9c14(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("- [ YAML : separate ]\n- [ : empty key entry ]\n- [ {JSON: like}:adjacent ]")?;
    assert_eq!(
        formatted,
        "- [YAML: separate]\n- [: empty key entry]\n- [{ JSON: like }: adjacent]"
    );
    Ok(())
}
#[test]
fn test_spec_example_7_21_single_pair_implicit_entries_yml_use_tabstrue_format_1_893e9c14(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("- [ YAML : separate ]\n- [ : empty key entry ]\n- [ {JSON: like}:adjacent ]")?;
    assert_eq!(
        formatted,
        "- [YAML: separate]\n- [: empty key entry]\n- [{ JSON: like }: adjacent]"
    );
    Ok(())
}
#[test]
fn test_spec_example_7_23_flow_content_yml_prose_wrapalways_format_1_fb2b2216() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- [ a, b ]\n- { a: b }\n- \"a\"\n- 'b'\n- c")?;
    assert_eq!(formatted, "- [a, b]\n- { a: b }\n- \"a\"\n- \"b\"\n- c");
    Ok(())
}
#[test]
fn test_spec_example_7_23_flow_content_yml_use_tabstrue_format_1_fb2b2216() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- [ a, b ]\n- { a: b }\n- \"a\"\n- 'b'\n- c")?;
    assert_eq!(formatted, "- [a, b]\n- { a: b }\n- \"a\"\n- \"b\"\n- c");
    Ok(())
}
#[test]
fn test_spec_example_7_24_flow_nodes_yml_prose_wrapalways_format_1_97eecaeb() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("- !!str \"a\"\n- 'b'\n- &anchor \"c\"\n- *anchor\n- !!str")?;
    assert_eq!(
        formatted,
        "- !!str \"a\"\n- \"b\"\n- &anchor \"c\"\n- *anchor\n- !!str"
    );
    Ok(())
}
#[test]
fn test_spec_example_7_24_flow_nodes_yml_use_tabstrue_format_1_97eecaeb() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("- !!str \"a\"\n- 'b'\n- &anchor \"c\"\n- *anchor\n- !!str")?;
    assert_eq!(
        formatted,
        "- !!str \"a\"\n- \"b\"\n- &anchor \"c\"\n- *anchor\n- !!str"
    );
    Ok(())
}
#[test]
fn test_spec_example_8_1_block_scalar_header_yml_prose_wrapalways_format_1_2e042a9a() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("- | # Empty header↓\n literal\n- >1 # Indentation indicator↓\n  folded\n- |+ # Chomping indicator↓\n keep\n\n- >1- # Both indicators↓\n  strip") ? ;
    assert_eq ! (formatted , "- | # Empty header↓\n  literal\n- >1 # Indentation indicator↓\n  folded\n- |+ # Chomping indicator↓\n  keep\n\n- >1- # Both indicators↓\n  strip");
    Ok(())
}
#[test]
fn test_spec_example_8_1_block_scalar_header_yml_use_tabstrue_format_1_2e042a9a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("- | # Empty header↓\n literal\n- >1 # Indentation indicator↓\n  folded\n- |+ # Chomping indicator↓\n keep\n\n- >1- # Both indicators↓\n  strip") ? ;
    assert_eq ! (formatted , "- | # Empty header↓\n  literal\n- >1 # Indentation indicator↓\n  folded\n- |+ # Chomping indicator↓\n  keep\n\n- >1- # Both indicators↓\n  strip");
    Ok(())
}
#[test]
fn test_spec_example_8_2_block_indentation_indicator_yml_prose_wrapalways_format_1_ac3f4a55(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "- |\n detected\n- >\n \n  \n  # detected\n- |1\n  explicit\n- >\n \t\n detected",
    )?;
    assert_eq!(
        formatted,
        "- |\n  detected\n- >\n\n\n  # detected\n- |1\n  explicit\n- >\n  \t\n  detected"
    );
    Ok(())
}
#[test]
fn test_spec_example_8_2_block_indentation_indicator_yml_use_tabstrue_format_1_ac3f4a55(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "- |\n detected\n- >\n \n  \n  # detected\n- |1\n  explicit\n- >\n \t\n detected",
    )?;
    assert_eq!(
        formatted,
        "- |\n  detected\n- >\n\n\n  # detected\n- |1\n  explicit\n- >\n  \t\n  detected"
    );
    Ok(())
}
#[test]
fn test_spec_example_8_4_chomping_final_line_break_yml_prose_wrapalways_format_1_3aa60445(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("strip: |-\n  text\nclip: |\n  text\nkeep: |+\n  text")?;
    assert_eq!(
        formatted,
        "strip: |-\n  text\nclip: |\n  text\nkeep: |+\n  text"
    );
    Ok(())
}
#[test]
fn test_spec_example_8_4_chomping_final_line_break_yml_use_tabstrue_format_1_3aa60445() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("strip: |-\n  text\nclip: |\n  text\nkeep: |+\n  text")?;
    assert_eq!(
        formatted,
        "strip: |-\n  text\nclip: |\n  text\nkeep: |+\n  text"
    );
    Ok(())
}
#[test]
fn test_spec_example_8_5_chomping_trailing_lines_yml_prose_wrapalways_format_1_1aaa2b0d(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format (" # Strip\n  # Comments:\nstrip: |-\n  # text\n  \n # Clip\n  # comments:\n\nclip: |\n  # text\n \n # Keep\n  # comments:\n\nkeep: |+\n  # text\n\n # Trail\n  # comments.") ? ;
    assert_eq ! (formatted , "# Strip\n# Comments:\nstrip: |-\n  # text\n\n# Clip\n# comments:\n\nclip: |\n  # text\n\n# Keep\n# comments:\n\nkeep: |+\n  # text\n\n# Trail\n# comments");
    Ok(())
}
#[test]
fn test_spec_example_8_5_chomping_trailing_lines_yml_use_tabstrue_format_1_1aaa2b0d() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format (" # Strip\n  # Comments:\nstrip: |-\n  # text\n  \n # Clip\n  # comments:\n\nclip: |\n  # text\n \n # Keep\n  # comments:\n\nkeep: |+\n  # text\n\n # Trail\n  # comments.") ? ;
    assert_eq ! (formatted , "# Strip\n# Comments:\nstrip: |-\n  # text\n\n# Clip\n# comments:\n\nclip: |\n  # text\n\n# Keep\n# comments:\n\nkeep: |+\n  # text\n\n# Trail\n# comments");
    Ok(())
}
#[test]
fn test_spec_example_8_6_empty_scalar_chomping_yml_prose_wrapalways_format_1_a558a2fe() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("strip: >-\n\nclip: >\n\nkeep: |+\n")?;
    assert_eq!(formatted, "strip: >-\n\nclip: >\n\nkeep: |+\n");
    Ok(())
}
#[test]
fn test_spec_example_8_6_empty_scalar_chomping_yml_use_tabstrue_format_1_a558a2fe() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("strip: >-\n\nclip: >\n\nkeep: |+\n")?;
    assert_eq!(formatted, "strip: >-\n\nclip: >\n\nkeep: |+\n");
    Ok(())
}
#[test]
fn test_spec_example_8_7_literal_scalar_yml_prose_wrapalways_format_1_2508d56b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("|\n literal\n \ttext\n\n")?;
    assert_eq!(formatted, "|\n  literal\n  \ttext");
    Ok(())
}
#[test]
fn test_spec_example_8_7_literal_scalar_yml_use_tabstrue_format_1_2508d56b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("|\n literal\n \ttext\n\n")?;
    assert_eq!(formatted, "|\n  literal\n  \ttext");
    Ok(())
}
#[test]
fn test_spec_example_8_8_literal_content_yml_prose_wrapalways_format_1_48ff2e63() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("|\n \n  \n  literal\n   \n  \n  text\n\n # Comment")?;
    assert_eq!(formatted, "|\n\n\n  literal\n   \n\n  text\n\n# Comment");
    Ok(())
}
#[test]
fn test_spec_example_8_8_literal_content_yml_use_tabstrue_format_1_48ff2e63() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("|\n \n  \n  literal\n   \n  \n  text\n\n # Comment")?;
    assert_eq!(formatted, "|\n\n\n  literal\n   \n\n  text\n\n# Comment");
    Ok(())
}
#[test]
fn test_spec_example_8_9_folded_scalar_yml_prose_wrapalways_format_1_5def84fc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(">\n folded\n text\n\n")?;
    assert_eq!(formatted, ">\n  folded text");
    Ok(())
}
#[test]
fn test_spec_example_8_9_folded_scalar_yml_use_tabstrue_format_1_5def84fc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(">\n folded\n text\n\n")?;
    assert_eq!(formatted, ">\n  folded\n  text");
    Ok(())
}
#[test]
fn test_spec_example_8_10_folded_lines_8_13_final_empty_lines_yml_prose_wrapalways_format_1_593f3286(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format (">\n\n folded\n line\n\n next\n line\n   * bullet\n\n   * list\n   * lines\n\n last\n line\n\n# Comment") ? ;
    assert_eq ! (formatted , ">\n\n  folded line\n\n  next line\n    * bullet\n\n    * list\n    * lines\n\n  last line\n\n# Comment");
    Ok(())
}
#[test]
fn test_spec_example_8_10_folded_lines_8_13_final_empty_lines_yml_use_tabstrue_format_1_593f3286(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format (">\n\n folded\n line\n\n next\n line\n   * bullet\n\n   * list\n   * lines\n\n last\n line\n\n# Comment") ? ;
    assert_eq ! (formatted , ">\n\n  folded\n  line\n\n  next\n  line\n    * bullet\n\n    * list\n    * lines\n\n  last\n  line\n\n# Comment");
    Ok(())
}
#[test]
fn test_spec_example_8_14_block_sequence_yml_prose_wrapalways_format_1_d7264990() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("block sequence:\n  - one\n  - two : three")?;
    assert_eq!(formatted, "block sequence:\n  - one\n  - two: three");
    Ok(())
}
#[test]
fn test_spec_example_8_14_block_sequence_yml_use_tabstrue_format_1_d7264990() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("block sequence:\n  - one\n  - two : three")?;
    assert_eq!(formatted, "block sequence:\n  - one\n  - two: three");
    Ok(())
}
#[test]
fn test_spec_example_8_15_block_sequence_entry_types_yml_prose_wrapalways_format_1_10605a8a(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("- # Empty\n- |\n block node\n- - one # Compact\n  - two # sequence\n- one: two # Compact mapping") ? ;
    assert_eq ! (formatted , "-  # Empty\n- |\n  block node\n- - one # Compact\n  - two # sequence\n- one: two # Compact mapping");
    Ok(())
}
#[test]
fn test_spec_example_8_15_block_sequence_entry_types_yml_use_tabstrue_format_1_10605a8a(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("- # Empty\n- |\n block node\n- - one # Compact\n  - two # sequence\n- one: two # Compact mapping") ? ;
    assert_eq ! (formatted , "-  # Empty\n- |\n  block node\n- - one # Compact\n  - two # sequence\n- one: two # Compact mapping");
    Ok(())
}
#[test]
fn test_spec_example_8_16_block_mappings_yml_prose_wrapalways_format_1_9ae98588() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("block mapping:\n key: value")?;
    assert_eq!(formatted, "block mapping:\n  key: value");
    Ok(())
}
#[test]
fn test_spec_example_8_16_block_mappings_yml_use_tabstrue_format_1_9ae98588() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("block mapping:\n key: value")?;
    assert_eq!(formatted, "block mapping:\n  key: value");
    Ok(())
}
#[test]
fn test_spec_example_8_17_explicit_block_mapping_entries_yml_prose_wrapalways_format_1_12554a1b(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("? explicit key # Empty value\n? |\n  block key\n: - one # Explicit compact\n  - two # block value") ? ;
    assert_eq ! (formatted , "? explicit key # Empty value\n? |\n  block key\n: - one # Explicit compact\n  - two # block value");
    Ok(())
}
#[test]
fn test_spec_example_8_17_explicit_block_mapping_entries_yml_use_tabstrue_format_1_12554a1b(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("? explicit key # Empty value\n? |\n  block key\n: - one # Explicit compact\n  - two # block value") ? ;
    assert_eq ! (formatted , "? explicit key # Empty value\n? |\n  block key\n: - one # Explicit compact\n  - two # block value");
    Ok(())
}
#[test]
fn test_spec_example_8_18_implicit_block_mapping_entries_yml_prose_wrapalways_format_1_707b719f(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("plain key: in-line value\n: # Both empty\n\"quoted key\":\n- entry")?;
    assert_eq!(
        formatted,
        "plain key: in-line value\n: # Both empty\n\"quoted key\":\n  - entry"
    );
    Ok(())
}
#[test]
fn test_spec_example_8_18_implicit_block_mapping_entries_yml_use_tabstrue_format_1_707b719f(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("plain key: in-line value\n: # Both empty\n\"quoted key\":\n- entry")?;
    assert_eq!(
        formatted,
        "plain key: in-line value\n: # Both empty\n\"quoted key\":\n  - entry"
    );
    Ok(())
}
#[test]
fn test_spec_example_8_19_compact_block_mappings_yml_prose_wrapalways_format_1_1523d5bb(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- sun: yellow\n- ? earth: blue\n  : moon: white")?;
    assert_eq!(formatted, "- sun: yellow\n- ? earth: blue\n  : moon: white");
    Ok(())
}
#[test]
fn test_spec_example_8_19_compact_block_mappings_yml_use_tabstrue_format_1_1523d5bb() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("- sun: yellow\n- ? earth: blue\n  : moon: white")?;
    assert_eq!(formatted, "- sun: yellow\n- ? earth: blue\n  : moon: white");
    Ok(())
}
#[test]
fn test_spec_example_8_20_block_node_types_yml_prose_wrapalways_format_1_ede0177a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "-\n  \"flow in block\"\n- >\n Block scalar\n- !!map # Block collection\n  foo : bar",
    )?;
    assert_eq!(
        formatted,
        "- \"flow in block\"\n- >\n  Block scalar\n- !!map # Block collection\n  foo: bar"
    );
    Ok(())
}
#[test]
fn test_spec_example_8_20_block_node_types_yml_use_tabstrue_format_1_ede0177a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "-\n  \"flow in block\"\n- >\n Block scalar\n- !!map # Block collection\n  foo : bar",
    )?;
    assert_eq!(
        formatted,
        "- \"flow in block\"\n- >\n  Block scalar\n- !!map # Block collection\n  foo: bar"
    );
    Ok(())
}
#[test]
fn test_spec_example_8_21_block_scalar_nodes_yml_prose_wrapalways_format_1_2cf8efcb() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("literal: |2\n  value\nfolded:\n   !foo\n  >1\n value")?;
    assert_eq!(formatted, "literal: |2\n  value\nfolded: !foo >1\n value");
    Ok(())
}
#[test]
fn test_spec_example_8_21_block_scalar_nodes_yml_use_tabstrue_format_1_2cf8efcb() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("literal: |2\n  value\nfolded:\n   !foo\n  >1\n value")?;
    assert_eq!(formatted, "literal: |2\n  value\nfolded: !foo >1\n value");
    Ok(())
}
#[test]
fn test_spec_example_8_22_block_collection_nodes_yml_prose_wrapalways_format_1_8839d194(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("sequence: !!seq\n- entry\n- !!seq\n - nested\nmapping: !!map\n foo: bar")?;
    assert_eq!(
        formatted,
        "sequence: !!seq\n  - entry\n  - !!seq\n    - nested\nmapping: !!map\n  foo: bar"
    );
    Ok(())
}
#[test]
fn test_spec_example_8_22_block_collection_nodes_yml_use_tabstrue_format_1_8839d194() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("sequence: !!seq\n- entry\n- !!seq\n - nested\nmapping: !!map\n foo: bar")?;
    assert_eq!(
        formatted,
        "sequence: !!seq\n  - entry\n  - !!seq\n    - nested\nmapping: !!map\n  foo: bar"
    );
    Ok(())
}
#[test]
fn test_spec_example_9_2_document_markers_yml_prose_wrapalways_format_1_6a45c035() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("%YAML 1.2\n---\nDocument\n... # Suffix")?;
    assert_eq!(formatted, "%YAML 1.2\n---\nDocument\n... # Suffix");
    Ok(())
}
#[test]
fn test_spec_example_9_2_document_markers_yml_use_tabstrue_format_1_6a45c035() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("%YAML 1.2\n---\nDocument\n... # Suffix")?;
    assert_eq!(formatted, "%YAML 1.2\n---\nDocument\n... # Suffix");
    Ok(())
}
#[test]
fn test_spec_example_9_3_bare_documents_yml_prose_wrapalways_format_1_cbbf937c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "Bare\ndocument\n...\n# No document\n...\n|\n%!PS-Adobe-2.0 # Not the first line",
    )?;
    assert_eq!(
        formatted,
        "Bare document\n---\n# No document\n---\n|\n  %!PS-Adobe-2.0 # Not the first line"
    );
    Ok(())
}
#[test]
fn test_spec_example_9_3_bare_documents_yml_use_tabstrue_format_1_cbbf937c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "Bare\ndocument\n...\n# No document\n...\n|\n%!PS-Adobe-2.0 # Not the first line",
    )?;
    assert_eq!(
        formatted,
        "Bare\ndocument\n---\n# No document\n---\n|\n  %!PS-Adobe-2.0 # Not the first line"
    );
    Ok(())
}
#[test]
fn test_spec_example_9_4_explicit_documents_yml_prose_wrapalways_format_1_74f5c941() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\n{ matches\n% : 20 }\n...\n---\n# Empty\n...")?;
    assert_eq!(formatted, "---\n{ matches %: 20 }\n---\n# Empty");
    Ok(())
}
#[test]
fn test_spec_example_9_4_explicit_documents_yml_use_tabstrue_format_1_74f5c941() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\n{ matches\n% : 20 }\n...\n---\n# Empty\n...")?;
    assert_eq!(formatted, "---\n{ ? matches\n    %\n  : 20 }\n---\n# Empty");
    Ok(())
}
#[test]
fn test_spec_example_9_5_directives_documents_yml_prose_wrapalways_format_1_7dfee48e() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("%YAML 1.2\n--- |\n%!PS-Adobe-2.0\n...\n%YAML1.2\n---\n# Empty\n...")?;
    assert_eq!(
        formatted,
        "%YAML 1.2\n---\n|\n  %!PS-Adobe-2.0\n...\n%YAML1.2\n---\n# Empty"
    );
    Ok(())
}
#[test]
fn test_spec_example_9_5_directives_documents_yml_use_tabstrue_format_1_7dfee48e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("%YAML 1.2\n--- |\n%!PS-Adobe-2.0\n...\n%YAML1.2\n---\n# Empty\n...")?;
    assert_eq!(
        formatted,
        "%YAML 1.2\n---\n|\n  %!PS-Adobe-2.0\n...\n%YAML1.2\n---\n# Empty"
    );
    Ok(())
}
#[test]
fn test_spec_example_9_6_stream_yml_prose_wrapalways_format_1_c0e7eedd() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("Document\n---\n# Empty\n...\n%YAML 1.2\n---\nmatches %: 20")?;
    assert_eq!(
        formatted,
        "Document\n---\n# Empty\n...\n%YAML 1.2\n---\nmatches %: 20"
    );
    Ok(())
}
#[test]
fn test_spec_example_9_6_stream_yml_use_tabstrue_format_1_c0e7eedd() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("Document\n---\n# Empty\n...\n%YAML 1.2\n---\nmatches %: 20")?;
    assert_eq!(
        formatted,
        "Document\n---\n# Empty\n...\n%YAML 1.2\n---\nmatches %: 20"
    );
    Ok(())
}
#[test]
fn test_tab_after_document_header_yml_prose_wrapalways_format_1_35d0472a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\tscalar")?;
    assert_eq!(formatted, "---\nscalar");
    Ok(())
}
#[test]
fn test_tab_after_document_header_yml_use_tabstrue_format_1_35d0472a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---\tscalar")?;
    assert_eq!(formatted, "---\nscalar");
    Ok(())
}
#[test]
fn test_tab_at_beginning_of_line_followed_by_a_flow_mapping_yml_prose_wrapalways_format_1_a2e50682(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\t{}")?;
    assert_eq!(formatted, "{}");
    Ok(())
}
#[test]
fn test_tab_at_beginning_of_line_followed_by_a_flow_mapping_yml_use_tabstrue_format_1_a2e50682(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\t{}")?;
    assert_eq!(formatted, "{}");
    Ok(())
}
#[test]
fn test_tags_for_block_objects_yml_prose_wrapalways_format_1_87efbd48() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("foo: !!seq\n  - !!str a\n  - !!map\n    key: !!str value")?;
    assert_eq!(
        formatted,
        "foo: !!seq\n  - !!str a\n  - !!map\n    key: !!str value"
    );
    Ok(())
}
#[test]
fn test_tags_for_block_objects_yml_use_tabstrue_format_1_87efbd48() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("foo: !!seq\n  - !!str a\n  - !!map\n    key: !!str value")?;
    assert_eq!(
        formatted,
        "foo: !!seq\n  - !!str a\n  - !!map\n    key: !!str value"
    );
    Ok(())
}
#[test]
fn test_tags_for_flow_objects_yml_prose_wrapalways_format_1_cace071d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!map {\n  k: !!seq\n  [ a, !!str b]\n}")?;
    assert_eq!(formatted, "!!map { k: !!seq [a, !!str b] }");
    Ok(())
}
#[test]
fn test_tags_for_flow_objects_yml_use_tabstrue_format_1_cace071d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!map {\n  k: !!seq\n  [ a, !!str b]\n}")?;
    assert_eq!(formatted, "!!map { k: !!seq [a, !!str b] }");
    Ok(())
}
#[test]
fn test_tags_for_root_objects_yml_prose_wrapalways_format_1_7f1dcb09() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("--- !!map\n? a\n: b\n--- !!seq\n- !!str c\n--- !!str\nd\ne")?;
    assert_eq!(
        formatted,
        "---\n!!map\na: b\n---\n!!seq\n- !!str c\n---\n!!str d e"
    );
    Ok(())
}
#[test]
fn test_tags_for_root_objects_yml_use_tabstrue_format_1_7f1dcb09() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("--- !!map\n? a\n: b\n--- !!seq\n- !!str c\n--- !!str\nd\ne")?;
    assert_eq!(
        formatted,
        "---\n!!map\na: b\n---\n!!seq\n- !!str c\n---\n!!str d\ne"
    );
    Ok(())
}
#[test]
fn test_tags_in_block_sequence_yml_prose_wrapalways_format_1_c570e551() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(" - !!str a\n - b\n - !!int 42\n - d")?;
    assert_eq!(formatted, "- !!str a\n- b\n- !!int 42\n- d");
    Ok(())
}
#[test]
fn test_tags_in_block_sequence_yml_use_tabstrue_format_1_c570e551() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(" - !!str a\n - b\n - !!int 42\n - d")?;
    assert_eq!(formatted, "- !!str a\n- b\n- !!int 42\n- d");
    Ok(())
}
#[test]
fn test_tags_in_explicit_mapping_yml_prose_wrapalways_format_1_658ae294() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("? !!str a\n: !!int 47\n? c\n: !!str d")?;
    assert_eq!(formatted, "!!str a: !!int 47\nc: !!str d");
    Ok(())
}
#[test]
fn test_tags_in_explicit_mapping_yml_use_tabstrue_format_1_658ae294() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("? !!str a\n: !!int 47\n? c\n: !!str d")?;
    assert_eq!(formatted, "!!str a: !!int 47\nc: !!str d");
    Ok(())
}
#[test]
fn test_tags_in_implicit_mapping_yml_prose_wrapalways_format_1_592fc6bb() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("!!str a: b\nc: !!int 42\ne: !!str f\ng: h\n!!int 23: !!bool false")?;
    assert_eq!(
        formatted,
        "!!str a: b\nc: !!int 42\ne: !!str f\ng: h\n!!int 23: !!bool false"
    );
    Ok(())
}
#[test]
fn test_tags_in_implicit_mapping_yml_use_tabstrue_format_1_592fc6bb() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("!!str a: b\nc: !!int 42\ne: !!str f\ng: h\n!!int 23: !!bool false")?;
    assert_eq!(
        formatted,
        "!!str a: b\nc: !!int 42\ne: !!str f\ng: h\n!!int 23: !!bool false"
    );
    Ok(())
}
#[test]
fn test_tags_on_empty_scalars_yml_prose_wrapalways_format_1_8b288569() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("- !!str\n-\n  !!null : a\n  b: !!str\n- !!str : !!null")?;
    assert_eq!(
        formatted,
        "- !!str\n- !!null : a\n  b: !!str\n- !!str : !!null"
    );
    Ok(())
}
#[test]
fn test_tags_on_empty_scalars_yml_use_tabstrue_format_1_8b288569() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("- !!str\n-\n  !!null : a\n  b: !!str\n- !!str : !!null")?;
    assert_eq!(
        formatted,
        "- !!str\n- !!null : a\n  b: !!str\n- !!str : !!null"
    );
    Ok(())
}
#[test]
fn test_three_dashes_and_content_without_space_yml_prose_wrapalways_format_1_40309137() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---word1\nword2")?;
    assert_eq!(formatted, "---word1 word2");
    Ok(())
}
#[test]
fn test_three_dashes_and_content_without_space_yml_use_tabstrue_format_1_40309137() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("---word1\nword2")?;
    assert_eq!(formatted, "---word1\nword2");
    Ok(())
}
#[test]
fn test_various_combinations_of_tags_and_anchors_yml_prose_wrapalways_format_1_68296301(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("---\n&a1\n!!str\nscalar1\n---\n!!str\n&a2\nscalar2\n---\n&a3\n!!str scalar3\n---\n&a4 !!map\n&a5 !!str key5: value4\n---\na6: 1\n&anchor6 b6: 2\n---\n!!map\n&a8 !!str key8: value7\n---\n!!map\n!!str &a10 key10: value9\n---\n!!str &a11\nvalue11") ? ;
    assert_eq ! (formatted , "---\n!!str &a1 scalar1\n---\n!!str &a2 scalar2\n---\n!!str &a3 scalar3\n---\n!!map &a4\n!!str &a5 key5: value4\n---\na6: 1\n&anchor6 b6: 2\n---\n!!map\n!!str &a8 key8: value7\n---\n!!map\n!!str &a10 key10: value9\n---\n!!str &a11 value11");
    Ok(())
}
#[test]
fn test_various_combinations_of_tags_and_anchors_yml_use_tabstrue_format_1_68296301() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("---\n&a1\n!!str\nscalar1\n---\n!!str\n&a2\nscalar2\n---\n&a3\n!!str scalar3\n---\n&a4 !!map\n&a5 !!str key5: value4\n---\na6: 1\n&anchor6 b6: 2\n---\n!!map\n&a8 !!str key8: value7\n---\n!!map\n!!str &a10 key10: value9\n---\n!!str &a11\nvalue11") ? ;
    assert_eq ! (formatted , "---\n!!str &a1 scalar1\n---\n!!str &a2 scalar2\n---\n!!str &a3 scalar3\n---\n!!map &a4\n!!str &a5 key5: value4\n---\na6: 1\n&anchor6 b6: 2\n---\n!!map\n!!str &a8 key8: value7\n---\n!!map\n!!str &a10 key10: value9\n---\n!!str &a11 value11");
    Ok(())
}
#[test]
fn test_various_location_of_anchors_in_flow_sequence_yml_prose_wrapalways_format_1_543c43af(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("&flowseq [\n a: b,\n &c c: d,\n { &e e: f },\n &g { g: h }\n]")?;
    assert_eq!(
        formatted,
        "&flowseq [a: b, &c c: d, { &e e: f }, &g { g: h }]"
    );
    Ok(())
}
#[test]
fn test_various_location_of_anchors_in_flow_sequence_yml_use_tabstrue_format_1_543c43af(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("&flowseq [\n a: b,\n &c c: d,\n { &e e: f },\n &g { g: h }\n]")?;
    assert_eq!(
        formatted,
        "&flowseq [a: b, &c c: d, { &e e: f }, &g { g: h }]"
    );
    Ok(())
}
#[test]
fn test_various_trailing_comments_yml_prose_wrapalways_format_1_1c764335() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("a: \"double\n  quotes\" # lala\nb: plain\n value  # lala\nc  : #lala\n  d\n? # lala\n - seq1\n: # lala\n - #lala\n  seq2\ne:\n &node # lala\n - x: y\nblock: > # lala\n  abcde") ? ;
    assert_eq ! (formatted , "a: \"double quotes\" # lala\nb: plain value # lala\nc: #lala\n  d\n? # lala\n  - seq1\n: # lala\n  - #lala\n    seq2\ne: &node # lala\n  - x: y\nblock: > # lala\n  abcde");
    Ok(())
}
#[test]
fn test_various_trailing_comments_yml_use_tabstrue_format_1_1c764335() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("a: \"double\n  quotes\" # lala\nb: plain\n value  # lala\nc  : #lala\n  d\n? # lala\n - seq1\n: # lala\n - #lala\n  seq2\ne:\n &node # lala\n - x: y\nblock: > # lala\n  abcde") ? ;
    assert_eq ! (formatted , "a: \"double\n  quotes\" # lala\nb: plain\n  value # lala\nc: #lala\n  d\n? # lala\n  - seq1\n: # lala\n  - #lala\n    seq2\ne: &node # lala\n  - x: y\nblock: > # lala\n  abcde");
    Ok(())
}
#[test]
fn test_various_trailing_tabs_yml_prose_wrapalways_format_1_376ca8ae() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: b\t\nseq:\t\n - a\t\nc: d\t#X")?;
    assert_eq!(formatted, "a: b\nseq:\n  - a\nc: d #X");
    Ok(())
}
#[test]
fn test_various_trailing_tabs_yml_use_tabstrue_format_1_376ca8ae() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: b\t\nseq:\t\n - a\t\nc: d\t#X")?;
    assert_eq!(formatted, "a: b\nseq:\n  - a\nc: d #X");
    Ok(())
}
#[test]
fn test_whitespace_after_scalars_in_flow_yml_prose_wrapalways_format_1_a1485398() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("- [a, b , c ]\n- { \"a\"  : b\n   , c : 'd' ,\n   e   : \"f\"\n  }\n- [      ]")?;
    assert_eq!(
        formatted,
        "- [a, b, c]\n- { \"a\": b, c: \"d\", e: \"f\" }\n- []"
    );
    Ok(())
}
#[test]
fn test_whitespace_after_scalars_in_flow_yml_use_tabstrue_format_1_a1485398() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("- [a, b , c ]\n- { \"a\"  : b\n   , c : 'd' ,\n   e   : \"f\"\n  }\n- [      ]")?;
    assert_eq!(
        formatted,
        "- [a, b, c]\n- { \"a\": b, c: \"d\", e: \"f\" }\n- []"
    );
    Ok(())
}
#[test]
fn test_whitespace_around_colon_in_mappings_yml_prose_wrapalways_format_1_b646ce63() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\"top1\" : \n  \"key1\" : &alias1 scalar1\n'top2' : \n  'key2' : &alias2 scalar2\ntop3: &node3 \n  *alias1 : scalar3\ntop4: \n  *alias2 : scalar4\ntop5   :    \n  scalar5\ntop6: \n  &anchor6 'key6' : scalar6") ? ;
    assert_eq ! (formatted , "\"top1\":\n  \"key1\": &alias1 scalar1\n\"top2\":\n  \"key2\": &alias2 scalar2\ntop3: &node3\n  *alias1 : scalar3\ntop4:\n  *alias2 : scalar4\ntop5: scalar5\ntop6:\n  &anchor6 \"key6\": scalar6");
    Ok(())
}
#[test]
fn test_whitespace_around_colon_in_mappings_yml_use_tabstrue_format_1_b646ce63() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\"top1\" : \n  \"key1\" : &alias1 scalar1\n'top2' : \n  'key2' : &alias2 scalar2\ntop3: &node3 \n  *alias1 : scalar3\ntop4: \n  *alias2 : scalar4\ntop5   :    \n  scalar5\ntop6: \n  &anchor6 'key6' : scalar6") ? ;
    assert_eq ! (formatted , "\"top1\":\n  \"key1\": &alias1 scalar1\n\"top2\":\n  \"key2\": &alias2 scalar2\ntop3: &node3\n  *alias1 : scalar3\ntop4:\n  *alias2 : scalar4\ntop5: scalar5\ntop6:\n  &anchor6 \"key6\": scalar6");
    Ok(())
}
#[test]
fn test_zero_indented_block_scalar_yml_prose_wrapalways_format_1_743a28f8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("--- >\nline1\nline2\nline3")?;
    assert_eq!(formatted, "---\n>\n  line1 line2 line3");
    Ok(())
}
#[test]
fn test_zero_indented_block_scalar_yml_use_tabstrue_format_1_743a28f8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("--- >\nline1\nline2\nline3")?;
    assert_eq!(formatted, "---\n>\n  line1\n  line2\n  line3");
    Ok(())
}
#[test]
fn test_zero_indented_block_scalar_with_line_that_looks_like_a_comment_yml_prose_wrapalways_format_1_c3caf2ef(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("--- >\nline1\n# no comment\nline3")?;
    assert_eq!(formatted, "---\n>\n  line1 # no comment line3");
    Ok(())
}
#[test]
fn test_zero_indented_block_scalar_with_line_that_looks_like_a_comment_yml_use_tabstrue_format_1_c3caf2ef(
) -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .use_tabs(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("--- >\nline1\n# no comment\nline3")?;
    assert_eq!(formatted, "---\n>\n  line1\n  # no comment\n  line3");
    Ok(())
}
