use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_comments_js_babel_flow_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_comments_js_format_1_951f80b2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\ntype A = B extends T\n  ? // comment\n    foo\n  : bar;\n\ntype A = B extends test /* comment\n  comment\n      comment\n*/\n  ? foo\n  : bar;\n\ntype T = test extends B\n  ? /* comment\n          comment\n    comment\n          comment\n  */\n    foo\n  : bar;\n\ntype T = test extends B\n  ? /* comment\n       comment\n       comment\n       comment\n    */\n    foo\n  : test extends B\n  ? /* comment\n  comment\n    comment */\n    foo\n  : bar;\n\ntype T = test extends B\n  ? /* comment */\n    foo\n  : bar;\n\ntype T = test extends B\n  ? foo\n  : /* comment\n         comment\n     comment\n           comment\n    */\n  bar;\n\ntype T = test extends B\n  ? foo\n  : /* comment\n         comment\n     comment\n           comment\n    */\n  test extends B\n  ? foo\n  : /* comment\n  comment\n    comment\n   */\n    bar;\n\ntype T = test extends B\n  ? foo\n  : /* comment */\n  bar;\n\ntype T = test extends B ? test extends B /* c\nc */? foo : bar : bar;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\ntype A = B extends T\n  ? // comment\n    foo\n  : bar;\n\ntype A = B extends test /* comment\n  comment\n      comment\n*/\n  ? foo\n  : bar;\n\ntype T = test extends B\n  ? /* comment\n          comment\n    comment\n          comment\n  */\n    foo\n  : bar;\n\ntype T = test extends B\n  ? /* comment\n       comment\n       comment\n       comment\n    */\n    foo\n  : test extends B\n    ? /* comment\n  comment\n    comment */\n      foo\n    : bar;\n\ntype T = test extends B ? /* comment */ foo : bar;\n\ntype T = test extends B\n  ? foo\n  : /* comment\n         comment\n     comment\n           comment\n    */\n    bar;\n\ntype T = test extends B\n  ? foo\n  : /* comment\n         comment\n     comment\n           comment\n    */\n    test extends B\n    ? foo\n    : /* comment\n  comment\n    comment\n   */\n      bar;\n\ntype T = test extends B ? foo : /* comment */ bar;\n\ntype T = test extends B\n  ? test extends B /* c\nc */\n    ? foo\n    : bar\n  : bar;");
}
