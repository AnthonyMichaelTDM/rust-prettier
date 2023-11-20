#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_comment_css_format_1_41b67e4a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["css"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * comment\n */\n\n.class\n{\n  padding:10px; }\n/*comment*/\n\n\n.foo{color:red\n}\n\n\n/*\n  at rule\n */\n\n\n@media only screen {\n  /* inner at rule comment */\n  .foo { display: none; }\n\n  /*\n   * another comment\n   */\n   /* in a row */\n\n.bar {\n  color:yellow;\n}\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * comment\n */\n\n.class {\n  padding: 10px;\n}\n/*comment*/\n\n.foo {\n  color: red;\n}\n\n/*\n  at rule\n */\n\n@media only screen {\n  /* inner at rule comment */\n  .foo {\n    display: none;\n  }\n\n  /*\n   * another comment\n   */\n  /* in a row */\n\n  .bar {\n    color: yellow;\n  }\n}");
}
