use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_comment_js_format_1_2baee741() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function foo() {\n  return {\n    // this comment causes the problem\n    bar: baz() + 1\n  };\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function foo() {\n  return {\n    // this comment causes the problem\n    bar: baz() + 1,\n  };\n}");
}
#[test]
fn test_long_value_js_format_1_6e377fa9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const x = {\n  \"ABC\": \"12345678901234567890123456789012345678901234567890123456789012345678901234567890\"\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const x = {\n  ABC: \"12345678901234567890123456789012345678901234567890123456789012345678901234567890\",\n};");
}
#[test]
fn test_short_keys_js_format_1_55208393() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var obj = {\n  // an entry with a very long string\n  x: \"12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890\",\n  url: 'http://example.com/12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890',\n  longName: \"12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890\",\n  [i]: \"12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890\",\n  [prop]: \"12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890\",\n  'x': \"12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890\",\n  a: \"12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890\",\n  ab: \"12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890\",\n  abc: \"12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890\",\n  abcd: \"12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890\",\n  abcde: \"12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890\",\n  abcdef: \"12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890\",\n  '古': 'https://prettier.io/docs/en/rationale.html#what-prettier-is-concerned-about',\n  '古今': 'https://prettier.io/docs/en/rationale.html#what-prettier-is-concerned-about',\n  '古体诗': 'https://prettier.io/docs/en/rationale.html#what-prettier-is-concerned-about',\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var obj = {\n  // an entry with a very long string\n  x: \"12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890\",\n  url: \"http://example.com/12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890\",\n  longName:\n    \"12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890\",\n  [i]: \"12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890\",\n  [prop]:\n    \"12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890\",\n  x: \"12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890\",\n  a: \"12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890\",\n  ab: \"12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890\",\n  abc: \"12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890\",\n  abcd: \"12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890\",\n  abcde:\n    \"12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890\",\n  abcdef:\n    \"12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890\",\n  古: \"https://prettier.io/docs/en/rationale.html#what-prettier-is-concerned-about\",\n  古今: \"https://prettier.io/docs/en/rationale.html#what-prettier-is-concerned-about\",\n  古体诗:\n    \"https://prettier.io/docs/en/rationale.html#what-prettier-is-concerned-about\",\n};");
}
#[test]
fn test_test_js_format_1_39d0eec5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const a = classnames({\n  \"some-prop\": this.state.longLongLongLongLongLongLongLongLongTooLongProp\n});\n\nconst b = classnames({\n  \"some-prop\": this.state.longLongLongLongLongLongLongLongLongTooLongProp === true\n});\n\nconst c = classnames({\n  \"some-prop\": [ \"foo\", \"bar\", \"foo\", \"bar\", \"foo\", \"bar\", \"foo\", \"bar\", \"foo\" ]\n});\n\nconst d = classnames({\n  \"some-prop\": () => {}\n});\n\nconst e = classnames({\n  \"some-prop\": function bar() {}\n});\n\nconst f = classnames({\n  \"some-prop\": { foo: \"bar\", bar: \"foo\", foo: \"bar\", bar: \"foo\", foo: \"bar\" }\n});\n\nconst g = classnames({\n  \"some-prop\": longLongLongLongLongLongLongLongLongLongLongLongLongTooLongVar || 1337\n});\n\nconst h = { foo: \"bar\", baz: \\`Lorem\nipsum\\` }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const a = classnames({\n  \"some-prop\": this.state.longLongLongLongLongLongLongLongLongTooLongProp,\n});\n\nconst b = classnames({\n  \"some-prop\":\n    this.state.longLongLongLongLongLongLongLongLongTooLongProp === true,\n});\n\nconst c = classnames({\n  \"some-prop\": [\"foo\", \"bar\", \"foo\", \"bar\", \"foo\", \"bar\", \"foo\", \"bar\", \"foo\"],\n});\n\nconst d = classnames({\n  \"some-prop\": () => {},\n});\n\nconst e = classnames({\n  \"some-prop\": function bar() {},\n});\n\nconst f = classnames({\n  \"some-prop\": { foo: \"bar\", bar: \"foo\", foo: \"bar\", bar: \"foo\", foo: \"bar\" },\n});\n\nconst g = classnames({\n  \"some-prop\":\n    longLongLongLongLongLongLongLongLongLongLongLongLongTooLongVar || 1337,\n});\n\nconst h = {\n  foo: \"bar\",\n  baz: \\`Lorem\nipsum\\`,\n};");
}
