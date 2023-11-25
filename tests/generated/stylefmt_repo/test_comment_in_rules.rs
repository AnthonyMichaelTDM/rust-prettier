#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_comment_in_rules_css_format_1_989e0202() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format (".class\n{display: inline-block;\n/* comment */\nfloat: left;}\n\n#id {\n  font-size: 12px;\n\n  /* colors */\n\n  color: pink;\n  background: #fff;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ".class {\n  display: inline-block;\n  /* comment */\n  float: left;\n}\n\n#id {\n  font-size: 12px;\n\n  /* colors */\n\n  color: pink;\n  background: #fff;\n}");
}
