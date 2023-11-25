#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_eslint_disable_js_bracket_same_linetrue_format_1_31e7aee1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_same_line(true)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const render = items => (\n  <div>{ /* eslint-disable */\n    \t items.map(item => null)\n      /* eslint-enable */    }</div>\n)") ? ;
    assert_eq ! (formatted , "const render = (items) => (\n  <div>\n    {\n      /* eslint-disable */\n      items.map((item) => null)\n      /* eslint-enable */\n    }\n  </div>\n);");
    Ok(())
}
#[test]
fn test_in_attributes_js_bracket_same_linetrue_format_1_c12d59a4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_same_line(true)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div\n  attr=/* comment */\"foo\"\n></div>;\n\n<div\n  attr=\n  /* comment */\n  \"foo\"\n></div>;\n\n<div\n  attr= /* comment */\n  \"foo\"\n></div>;\n\n<div\n  attr=\n  /* comment */ \"foo\"\n></div>;\n\n<div\n  attr=\n  // comment\n  \"foo\"\n></div>;\n\n<div\n  attr= // comment\n  \"foo\"\n></div>;") ? ;
    assert_eq ! (formatted , "<div attr=/* comment */ \"foo\"></div>;\n\n<div attr=/* comment */\n\"foo\"></div>;\n\n<div attr /* comment */=\"foo\"></div>;\n\n<div attr=/* comment */ \"foo\"></div>;\n\n<div attr=// comment\n\"foo\"></div>;\n\n<div attr=\"foo\"></div>; // comment");
    Ok(())
}
#[test]
fn test_in_end_tag_js_bracket_same_linetrue_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_in_end_tag_js_bracket_same_linetrue_typescript_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_in_end_tag_js_bracket_same_linetrue_format_1_db3189b1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_same_line(true)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* =========== before slash =========== */\n<a><// line\n/a>;\n<a></* block */\n/a>;\n\n<><// line\n/>;\n<></* block */\n/>;\n\n/* =========== after slash =========== */\n<a></ // line\na>;\n<a></ /* block */\na>;\n\n<></ // line\n>;\n<></ /* block */\n>;\n\n/* =========== after name =========== */\n<a></a // line\n>;\n<a></a /* block */\n>;\n\n\n/* =========== block =========== */\n<a></a /* block */>;\n<></ /* block */>;\n\n/* =========== multiple ===========  */\n<a><// line 1\n// line 2\n/a>;\n<a></* block1 */ /* block2 */\n/a>;\n<a></* block */ // line\n/a>;\n\n<><// line 1\n// line 2\n/>;\n<></* block1 */ /* block2 */\n/>;\n<></* block */ // line\n/>") ? ;
    assert_eq ! (formatted , "/* =========== before slash =========== */\n<a></\n  // line\n  a\n>;\n<a></ /* block */\na>;\n\n<></\n  // line\n>;\n<></ /* block */>;\n\n/* =========== after slash =========== */\n<a></\n  // line\n  a\n>;\n<a></ /* block */\na>;\n\n<></\n  // line\n>;\n<></ /* block */>;\n\n/* =========== after name =========== */\n<a></a>; // line\n<a></a /* block */>;\n\n/* =========== block =========== */\n<a></a /* block */>;\n<></ /* block */>;\n\n/* =========== multiple ===========  */\n<a></\n  // line 1\n  // line 2\n  a\n>;\n<a></ /* block1 */ /* block2 */\na>;\n<a></\n  /* block */ // line\n  a\n>;\n\n<></\n  // line 1\n  // line 2\n>;\n<></ /* block1 */\n  /* block2 */>;\n<></\n  /* block */\n  // line\n>;");
    Ok(())
}
#[test]
fn test_in_tags_js_bracket_same_linetrue_format_1_e5d6a89f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_same_line(true)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div\n  // comment\n>\n  {foo}\n</div>;\n\n<div\n  // comment\n  attr=\"foo\"\n>\n  {foo}\n</div>;\n\n<div\n  attr=\"foo\" // comment\n>\n  {foo}\n</div>;\n\n<div\n  attr=\"foo\"\n  // comment\n>\n  {foo}\n</div>;\n\n<br // comment\n/>;") ? ;
    assert_eq ! (formatted , "<div\n// comment\n>\n  {foo}\n</div>;\n\n<div\n  // comment\n  attr=\"foo\">\n  {foo}\n</div>;\n\n<div\n  attr=\"foo\" // comment\n>\n  {foo}\n</div>;\n\n<div\n  attr=\"foo\"\n  // comment\n>\n  {foo}\n</div>;\n\n<br // comment\n/>;");
    Ok(())
}
#[test]
fn test_jsx_tag_comment_after_prop_js_bracket_same_linetrue_format_1_2a639ce4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_same_line(true)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://github.com/typescript-eslint/typescript-eslint/pull/703\n\nconst pure = () => {\n  return (\n      <Foo\n        // one\n        foo={123}\n        // two\n        bar=\"woof\"\n      />\n  );\n") ? ;
    assert_eq ! (formatted , "// https://github.com/typescript-eslint/typescript-eslint/pull/703\n\nconst pure = () => {\n  return (\n    <Foo\n      // one\n      foo={123}\n      // two\n      bar=\"woof\"\n    />\n  );\n};");
    Ok(())
}
#[test]
fn test_like_a_comment_in_jsx_text_js_bracket_same_linetrue_format_1_04141cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_same_line(true)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<Foo\n>\n  text\n  // comment\n  text\n</Foo")?;
    assert_eq!(formatted, "<Foo>text // comment text</Foo>;");
    Ok(())
}
